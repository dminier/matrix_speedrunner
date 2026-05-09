//! Écran de fin de partie : résumé du run + options (rejouer, scores, menu).

use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::game::GameMode;
use crate::ui::common::{DARK_GREEN, GREEN, RED};

pub(super) fn render_game_over(
    f: &mut Frame,
    centered: Rect,
    name: &str,
    mode: GameMode,
    difficulty: crate::game::Difficulty,
    summary: &crate::game::Summary,
    saved: bool,
) {
    let title = match mode {
        GameMode::SpeedRunner => " GAME OVER ",
        GameMode::HackTimeAttack => " TIME OUT ",
    };
    let lines = vec![
        Line::from(Span::styled(
            match mode {
                GameMode::SpeedRunner => "SYSTEM FAILURE",
                GameMode::HackTimeAttack => "TIME EXPIRED",
            }
            .to_string(),
            Style::default().fg(RED).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("{name} — {} — {}", mode.label(), difficulty.label()),
            Style::default().fg(GREEN).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(format!("Score    : {}", summary.score), Style::default().fg(GREEN).bold())),
        Line::from(Span::styled(format!("Commandes: {}", summary.items_done), Style::default().fg(GREEN))),
        Line::from(Span::styled(format!("WPM      : {}", summary.wpm), Style::default().fg(GREEN))),
        Line::from(Span::styled(format!("Combo max: x{}", summary.max_combo), Style::default().fg(GREEN))),
        Line::from(Span::styled(format!("Durée    : {}s", summary.elapsed_secs), Style::default().fg(GREEN))),
        Line::from(""),
        Line::from(Span::styled(
            if saved { "✓ Score enregistré pour le concours" } else { "✗ Échec sauvegarde" }.to_string(),
            Style::default().fg(if saved { GREEN } else { RED }).italic(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "[R/Enter] rejouer   [D] changer difficulté   [S] scores   [M/Esc] menu   [Q] quitter".to_string(),
            Style::default().fg(DARK_GREEN),
        )),
    ];
    let p = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(RED))
            .title(title)
            .title_style(Style::default().fg(RED).bold()),
    );
    f.render_widget(p, centered);
}
