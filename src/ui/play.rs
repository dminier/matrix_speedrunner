//! Rendu en jeu pour les deux modes.
//! - `render_speedrunner` : commandes qui tombent + HUD vies/score.
//! - `render_hack` : commande à taper centrée + barre de temps + HUD chrono.

use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::hack::HackState;
use crate::speedrunner::SpeedRunnerState;
use crate::ui::common::{chip, AMBER, BRIGHT_GREEN, DARK_GREEN, GREEN, RED};

pub(super) fn render_speedrunner(f: &mut Frame, area: Rect, g: &SpeedRunnerState, name: &str) {
    let [hud, play, input] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .areas(area);

    let hud_text = Line::from(vec![
        Span::styled(format!(" {} ", name), Style::default().fg(Color::Black).bg(GREEN).bold()),
        Span::raw("  "),
        Span::styled(" SPEED RUNNER ", Style::default().fg(Color::Black).bg(BRIGHT_GREEN).bold()),
        Span::raw("  "),
        chip(" SCORE ", &format!(" {:>6} ", g.score)),
        chip(" COMBO ", &format!(" x{:<3} ", g.combo)),
        Span::styled(" LIVES ", Style::default().fg(Color::White).bg(RED).bold()),
        Span::raw(format!(" {} ", "♥".repeat(g.lives as usize))),
        sr_time_chip(g.time_remaining()),
        chip(" WPM ", &format!(" {:>3} ", g.wpm())),
    ]);
    f.render_widget(
        Paragraph::new(hud_text).style(Style::default().fg(GREEN)),
        hud,
    );

    let buf = f.buffer_mut();
    for cmd in &g.commands {
        let y = play.y + cmd.y as u16;
        if y >= play.y + play.height {
            continue;
        }
        let matched = !g.buffer.is_empty() && cmd.text.starts_with(&g.buffer);
        for (i, ch) in cmd.text.chars().enumerate() {
            let x = play.x + cmd.x + i as u16;
            if x >= play.x + play.width {
                break;
            }
            let style = if matched && i < g.buffer.chars().count() {
                Style::default().fg(Color::Black).bg(GREEN).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(GREEN).add_modifier(Modifier::BOLD)
            };
            let cell = &mut buf[(x, y)];
            cell.set_char(ch);
            cell.set_style(style);
        }
    }

    let border = if g.flash_error > 0.0 { RED } else { GREEN };
    let prompt = format!("> {}_", g.buffer);
    let p = Paragraph::new(prompt)
        .style(Style::default().fg(GREEN).bold())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border))
                .title(" jack in ")
                .title_style(Style::default().fg(border)),
        );
    f.render_widget(p, input);
}

// =====================================================================
// Hack Time Attack — gameplay
// =====================================================================

pub(super) fn render_hack(f: &mut Frame, area: Rect, g: &HackState, name: &str) {
    let [hud, body, input] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .areas(area);

    let remaining = g.time_remaining();
    let timer_color = if remaining < 15.0 { RED } else if remaining < 30.0 { AMBER } else { GREEN };
    let mins = (remaining as u32) / 60;
    let secs = (remaining as u32) % 60;

    let hud_text = Line::from(vec![
        Span::styled(format!(" {} ", name), Style::default().fg(Color::Black).bg(GREEN).bold()),
        Span::raw("  "),
        Span::styled(" HACK TIME ATTACK ", Style::default().fg(Color::Black).bg(BRIGHT_GREEN).bold()),
        Span::raw("  "),
        chip(" SCORE ", &format!(" {:>6} ", g.score)),
        chip(" CMD ", &format!(" {:>3} ", g.commands_done)),
        chip(" COMBO ", &format!(" x{:<3} ", g.combo)),
        Span::styled(" TIME ", Style::default().fg(Color::Black).bg(timer_color).bold()),
        Span::styled(format!(" {mins}:{secs:02} "), Style::default().fg(timer_color).bold()),
        chip(" WPM ", &format!(" {:>3} ", g.wpm())),
    ]);
    f.render_widget(
        Paragraph::new(hud_text).style(Style::default().fg(GREEN)),
        hud,
    );

    // Centered "current command" panel + progress bar.
    // body: split vertically: timer bar (1) | command (fill) | hint (1)
    let [bar, target, hint] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(body);

    // Progress bar (time remaining)
    let pct = (remaining / g.time_limit).clamp(0.0, 1.0);
    let filled = (bar.width as f32 * pct) as u16;
    let buf = f.buffer_mut();
    for x in 0..bar.width {
        let cell = &mut buf[(bar.x + x, bar.y)];
        let on = x < filled;
        cell.set_char(if on { '━' } else { '·' });
        cell.set_style(Style::default().fg(if on { timer_color } else { DARK_GREEN }));
    }

    // Current command rendered char-by-char with already-typed chars highlighted.
    let typed_count = g.buffer.chars().count();
    let cmd_chars: Vec<char> = g.current.chars().collect();
    let cmd_len = cmd_chars.len();
    let buf = f.buffer_mut();
    let cy = target.y + target.height / 2;
    let total_w = cmd_len as u16;
    let start_x = if total_w >= target.width {
        target.x
    } else {
        target.x + (target.width - total_w) / 2
    };

    // intro line above target
    if target.height >= 3 {
        let label = "// inject the command :";
        let lx = target.x + (target.width.saturating_sub(label.len() as u16)) / 2;
        for (i, ch) in label.chars().enumerate() {
            let cell = &mut buf[(lx + i as u16, cy.saturating_sub(2))];
            cell.set_char(ch);
            cell.set_style(Style::default().fg(DARK_GREEN).italic());
        }
    }

    for (i, ch) in cmd_chars.iter().enumerate() {
        let x = start_x + i as u16;
        if x >= target.x + target.width {
            break;
        }
        let typed = i < typed_count;
        let style = if typed {
            Style::default().fg(Color::Black).bg(BRIGHT_GREEN).bold()
        } else if i == typed_count {
            // cursor
            Style::default().fg(Color::Black).bg(GREEN).bold()
        } else {
            Style::default().fg(GREEN).bold()
        };
        let cell = &mut buf[(x, cy)];
        cell.set_char(*ch);
        cell.set_style(style);
    }

    // pulse "ACCESS GRANTED" briefly when a command was just completed
    if g.last_complete_pulse > 0.0 && target.height >= 5 {
        let msg = "✓ ACCESS GRANTED";
        let lx = target.x + (target.width.saturating_sub(msg.len() as u16)) / 2;
        let py = cy + 2;
        if py < target.y + target.height {
            for (i, ch) in msg.chars().enumerate() {
                let cell = &mut buf[(lx + i as u16, py)];
                cell.set_char(ch);
                cell.set_style(Style::default().fg(BRIGHT_GREEN).bold());
            }
        }
    }

    // hint line
    let hint_text = format!(
        "Combo : x{} | erreurs : {} | une commande validée → suivante apparaît automatiquement",
        g.combo, g.errors
    );
    let p = Paragraph::new(Span::styled(hint_text, Style::default().fg(DARK_GREEN).italic()))
        .alignment(Alignment::Center);
    f.render_widget(p, hint);

    // Input box
    let border = if g.flash_error > 0.0 { RED } else { GREEN };
    let prompt = format!("> {}_", g.buffer);
    let p = Paragraph::new(prompt)
        .style(Style::default().fg(GREEN).bold())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border))
                .title(" jack in ")
                .title_style(Style::default().fg(border)),
        );
    f.render_widget(p, input);
}

fn sr_time_chip(remaining: f32) -> Span<'static> {
    let color = if remaining < 15.0 { RED }
                else if remaining < 30.0 { AMBER }
                else { GREEN };
    let s = remaining as u32;
    let mins = s / 60;
    let secs = s % 60;
    Span::styled(
        format!(" TIME {mins}:{secs:02} "),
        Style::default().fg(Color::Black).bg(color).bold(),
    )
}
