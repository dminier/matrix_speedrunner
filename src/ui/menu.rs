//! Écran principal : logo MATRIX en ASCII art (avec glitch blanc faisant
//! apparaître B / Z / H), sélection de mode + Scores + Quit.

use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::menu_items;
use crate::ui::common::{DARK_GREEN, GREEN};

const LOGO: [&str; 5] = [
    "  __  __    _  _____ ____  ___ __  __  ",
    " |  \\/  |  / \\|_   _|  _ \\|_ _|\\ \\/ /  ",
    " | |\\/| | / _ \\ | | | |_) || |  \\  /   ",
    " | |  | |/ ___ \\| | |  _ < | |  /  \\   ",
    " |_|  |_/_/   \\_\\_| |_| \\_\\___|/_/\\_\\  ",
];

pub(super) fn render_menu(f: &mut Frame, centered: Rect, selected: usize) {
    let mut lines: Vec<Line> = Vec::with_capacity(LOGO.len() + 16);
    for row in LOGO {
        lines.push(Line::from(Span::styled(
            row.to_string(),
            Style::default().fg(GREEN).bold(),
        )));
    }
    lines.push(glitched_speedrunner());
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Wake up, Neo... the terminal has you.".to_string(),
        Style::default().fg(DARK_GREEN).italic(),
    )));
    lines.push(Line::from(""));

    let items = menu_items();
    for (i, item) in items.iter().enumerate() {
        let lab = item.label();
        let desc = item.description();
        let active = i == selected;
        let label_style = if active {
            Style::default().fg(Color::Black).bg(GREEN).bold()
        } else {
            Style::default().fg(GREEN).bold()
        };
        lines.push(
            Line::from(Span::styled(format!("  > {lab}  "), label_style))
                .alignment(Alignment::Center),
        );
        if active {
            lines.push(
                Line::from(Span::styled(
                    desc.to_string(),
                    Style::default().fg(DARK_GREEN).italic(),
                ))
                .alignment(Alignment::Center),
            );
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

/// Rend la ligne sous-titre, alternant entre `SPEEDRUNNER` (vert phosphore Matrix)
/// et `BREIZHCAMP` (vert plus sombre, teinte « bretonne »), avec un fondu
/// lettre-à-lettre de gauche à droite dans les deux sens.
fn glitched_speedrunner() -> Line<'static> {
    // Les deux chaînes sont alignées à 11 caractères (BREIZHCAMP padded à droite).
    const SOURCE: &str = "SPEEDRUNNER";
    const TARGET: &str = "BREIZHCAMP ";
    const PREFIX: &str = "        ";
    // Vert plus sombre que GREEN/DARK_GREEN pour BREIZHCAMP.
    const BREIZH_GREEN: Color = Color::Rgb(0, 180, 90);

    debug_assert_eq!(SOURCE.chars().count(), TARGET.chars().count());
    let n = SOURCE.chars().count() as f32;

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    // Cycle de 4 s : hold SPEEDRUNNER 1.5 s | morph 0.5 s | hold BREIZHCAMP 1.5 s | morph back 0.5 s.
    let t = (millis % 4000) as f32 / 4000.0;
    let (progress, going_to_target) = if t < 0.375 {
        (0.0, true)
    } else if t < 0.5 {
        ((t - 0.375) / 0.125, true)
    } else if t < 0.875 {
        (1.0, true)
    } else {
        // Morph retour : inverse, on fait avancer les lettres SOURCE de gauche à droite.
        ((t - 0.875) / 0.125, false)
    };

    let mut spans: Vec<Span<'static>> = Vec::with_capacity(2 + SOURCE.len() * 2);
    spans.push(Span::raw(PREFIX));
    let src: Vec<char> = SOURCE.chars().collect();
    let tgt: Vec<char> = TARGET.chars().collect();
    for i in 0..src.len() {
        if i > 0 {
            spans.push(Span::raw(" "));
        }
        let pos = (i as f32) / n;
        // Lettre devenue "TARGET" si pos < progress (forward) ou pos >= progress (reverse).
        let show_target = if going_to_target {
            pos < progress
        } else {
            pos >= progress
        };
        let (ch, color) = if show_target {
            (tgt[i], BREIZH_GREEN)
        } else {
            (src[i], GREEN)
        };
        spans.push(Span::styled(
            ch.to_string(),
            Style::default().fg(color).bold(),
        ));
    }
    Line::from(spans)
}
