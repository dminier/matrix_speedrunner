//! Écran « choisis ta pilule » : Easy / Normal / Insane.

use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::game::GameMode;
use crate::ui::common::{BRIGHT_GREEN, DARK_GREEN, GREEN};

pub(super) fn render_difficulty(f: &mut Frame, centered: Rect, mode: GameMode, name: &str, selected: usize) {
    let labels = ["Easy", "Normal", "Insane"];
    let mut lines = vec![
        Line::from(Span::styled(
            format!("Bienvenue {name}."),
            Style::default().fg(GREEN).bold(),
        )),
        Line::from(Span::styled(
            mode.label().to_string(),
            Style::default().fg(BRIGHT_GREEN).bold(),
        )),
        Line::from(Span::styled(
            "Choisis ta pilule :".to_string(),
            Style::default().fg(DARK_GREEN).italic(),
        )),
        Line::from(""),
    ];
    for (i, lab) in labels.iter().enumerate() {
        let style = if i == selected {
            Style::default().fg(Color::Black).bg(GREEN).bold()
        } else {
            Style::default().fg(GREEN)
        };
        lines.push(Line::from(Span::styled(format!("  > {lab}  "), style)).alignment(Alignment::Center));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Enter] jack in   [Esc] retour".to_string(),
        Style::default().fg(DARK_GREEN),
    )));
    let p = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(GREEN))
            .title(" Difficulté ")
            .title_style(Style::default().fg(GREEN).bold()),
    );
    f.render_widget(p, centered);
}
