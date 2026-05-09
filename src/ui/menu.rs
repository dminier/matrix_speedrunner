//! Écran principal : logo ASCII + sélection de mode + Scores + Quit.

use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::game::GameMode;
use crate::ui::common::{DARK_GREEN, GREEN};

pub(super) fn render_menu(f: &mut Frame, centered: Rect, selected: usize) {
    let logo = vec![
        "  __  __    _  _____ ____  ___ __  __  ",
        " |  \\/  |  / \\|_   _|  _ \\|_ _|\\ \\/ /  ",
        " | |\\/| | / _ \\ | | | |_) || |  \\  /   ",
        " | |  | |/ ___ \\| | |  _ < | |  /  \\   ",
        " |_|  |_/_/   \\_\\_| |_| \\_\\___|/_/\\_\\  ",
    ];
    let mut lines: Vec<Line> = logo
        .into_iter()
        .map(|l| Line::from(Span::styled(l.to_string(), Style::default().fg(GREEN).bold())))
        .collect();
    lines.push(Line::from(Span::styled(
        "Wake up, Neo... the terminal has you.".to_string(),
        Style::default().fg(DARK_GREEN).italic(),
    )));
    lines.push(Line::from(""));

    let mut options: Vec<(&str, &str)> = GameMode::ALL
        .iter()
        .map(|m| (m.label(), m.description()))
        .collect();
    options.push(("Scores", "Tableau des scores du concours, par jour."));
    options.push(("Quit", "Sortir de la Matrice."));

    for (i, (lab, desc)) in options.iter().enumerate() {
        let active = i == selected;
        let label_style = if active {
            Style::default().fg(Color::Black).bg(GREEN).bold()
        } else {
            Style::default().fg(GREEN).bold()
        };
        lines.push(Line::from(Span::styled(format!("  > {lab}  "), label_style))
            .alignment(Alignment::Center));
        if active {
            lines.push(Line::from(Span::styled(
                desc.to_string(),
                Style::default().fg(DARK_GREEN).italic(),
            )).alignment(Alignment::Center));
        } else {
            lines.push(Line::from(""));
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Up/Down] choose   [Enter] select   [Q/Esc] quit".to_string(),
        Style::default().fg(DARK_GREEN),
    )));

    let p = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(GREEN))
            .title(" MATRIX SPEEDRUNNER ")
            .title_style(Style::default().fg(GREEN).bold()),
    );
    f.render_widget(p, centered);
}
