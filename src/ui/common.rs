//! Helpers partagés par tous les écrans : palette de couleurs, helpers de
//! layout (centrage, padding), formatage (troncature, libellés de jour),
//! et petits widgets stylistiques (chips de HUD, label de champ de saisie).

use chrono::{Datelike, NaiveDate};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Widget};
use ratatui::Frame;

pub const GREEN: Color = Color::Rgb(0, 255, 65);
pub const DARK_GREEN: Color = Color::Rgb(0, 140, 40);
pub const BRIGHT_GREEN: Color = Color::Rgb(180, 255, 200);
pub const RED: Color = Color::Rgb(255, 60, 60);
pub const AMBER: Color = Color::Rgb(255, 200, 80);

/// Efface une zone du buffer en cellule-par-cellule. Plus fiable que
/// `Clear` seul : ce dernier laisse parfois `cell.symbol` à la chaîne vide,
/// que certains backends interprètent comme « ne pas modifier ce caractère ».
pub fn clear_rect(f: &mut Frame, r: Rect) {
    Clear.render(r, f.buffer_mut());
    let buf: &mut Buffer = f.buffer_mut();
    for y in r.y..r.y.saturating_add(r.height) {
        for x in r.x..r.x.saturating_add(r.width) {
            let cell = &mut buf[(x, y)];
            cell.set_char(' ');
            cell.set_style(Style::reset());
        }
    }
}

/// Petite étiquette HUD `LABEL  valeur`, en gras vert.
pub fn chip(label: &str, value: &str) -> Span<'static> {
    Span::styled(format!("{label}{value}"), Style::default().fg(GREEN)).bold()
}

/// Ligne `▶ Label : valeur` pour formulaire (champ actif vs inactif).
pub fn label_line<'a>(label: &'a str, value: &'a str, active: bool) -> Line<'a> {
    let marker = if active { "▶ " } else { "  " };
    let style_label = if active {
        Style::default().fg(GREEN).bold()
    } else {
        Style::default().fg(DARK_GREEN)
    };
    let style_value = if active {
        Style::default().fg(Color::Black).bg(GREEN).bold()
    } else {
        Style::default().fg(GREEN)
    };
    let cursor = if active { "_" } else { "" };
    Line::from(vec![
        Span::styled(marker.to_string(), style_label),
        Span::styled(format!("{label}: "), style_label),
        Span::styled(format!(" {value}{cursor} "), style_value),
    ])
}

pub fn inner_of(r: Rect, dx: u16, dy: u16) -> Rect {
    Rect {
        x: r.x + dx,
        y: r.y + dy,
        width: r.width.saturating_sub(2 * dx),
        height: r.height.saturating_sub(2 * dy),
    }
}

pub fn day_label(day: NaiveDate, today: NaiveDate) -> String {
    let delta = today.num_days_from_ce() - day.num_days_from_ce();
    match delta {
        0 => format!("Aujourd'hui — {}", day.format("%A %d %B %Y")),
        1 => format!("Hier — {}", day.format("%A %d %B %Y")),
        _ => format!("{}", day.format("%A %d %B %Y")),
    }
}

pub fn truncate(s: &str, max: usize) -> String {
    let n = s.chars().count();
    if n <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}

pub fn center_rect(area: Rect, w: u16, h: u16) -> Rect {
    let w = w.min(area.width);
    let h = h.min(area.height);
    Rect {
        x: area.x + (area.width.saturating_sub(w)) / 2,
        y: area.y + (area.height.saturating_sub(h)) / 2,
        width: w,
        height: h,
    }
}
