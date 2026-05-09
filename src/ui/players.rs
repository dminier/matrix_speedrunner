//! Liste des joueurs agrégés par identité (clé = contact normalisé).

use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::score::{self, ScoreEntry};
use crate::ui::common::{inner_of, truncate, DARK_GREEN, GREEN};

pub(super) fn render_players(
    f: &mut Frame,
    centered: Rect,
    entries: &[ScoreEntry],
    selected: usize,
    scroll: u16,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(GREEN))
        .title(" JOUEURS — agrégation par identité ")
        .title_style(Style::default().fg(GREEN).bold());
    f.render_widget(block, centered);
    let inner = inner_of(centered, 2, 1);

    let players = score::aggregate_players(entries);

    if players.is_empty() {
        let p = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "Aucun joueur enregistré.".to_string(),
                Style::default().fg(DARK_GREEN).italic(),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "[Esc] retour".to_string(),
                Style::default().fg(DARK_GREEN),
            )),
        ])
        .alignment(Alignment::Center);
        f.render_widget(p, inner);
        return;
    }

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        format!(
            "  {:>3}  {:<22}  {:<28}  {:>5}  {:>8}  {}",
            "#", "Nom", "Contact", "Runs", "Best", "Dernier run"
        ),
        Style::default().fg(DARK_GREEN).bold(),
    )));

    for (i, p) in players.iter().enumerate() {
        let rank = i + 1;
        let active = i == selected;
        let style = if active {
            Style::default().fg(Color::Black).bg(GREEN).bold()
        } else {
            match rank {
                1 => Style::default().fg(Color::Rgb(255, 215, 0)).bold(),
                2 => Style::default().fg(Color::Rgb(192, 192, 192)).bold(),
                3 => Style::default().fg(Color::Rgb(205, 127, 50)).bold(),
                _ => Style::default().fg(GREEN),
            }
        };
        let marker = if active { "▶" } else { " " };
        lines.push(Line::from(Span::styled(
            format!(
                "{marker} {:>3}  {:<22}  {:<28}  {:>5}  {:>8}  {}",
                rank,
                truncate(&p.name, 22),
                truncate(&p.contact, 28),
                p.runs,
                p.best_score,
                p.last_played.format("%d/%m %H:%M"),
            ),
            style,
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Up/Down] navigue   [Enter] détails + courbe   [Esc] retour scores   [Q] quitter"
            .to_string(),
        Style::default().fg(DARK_GREEN),
    )));

    let p = Paragraph::new(lines).scroll((scroll, 0));
    f.render_widget(p, inner);
}
