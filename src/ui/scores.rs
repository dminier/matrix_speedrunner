//! Tableau des scores groupé par jour. Filtre par mode (cycle via [F]).

use std::collections::BTreeMap;

use chrono::{Local, NaiveDate};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::game::GameMode;
use crate::score::ScoreEntry;
use crate::ui::common::{day_label, inner_of, truncate, DARK_GREEN, GREEN};

pub(super) fn render_scores(
    f: &mut Frame,
    centered: Rect,
    entries: &[ScoreEntry],
    scroll: u16,
    filter: Option<GameMode>,
) {
    let title = match filter {
        None => " SCORES — tous modes ".to_string(),
        Some(m) => format!(" SCORES — {} ", m.label()),
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(GREEN))
        .title(title)
        .title_style(Style::default().fg(GREEN).bold());
    f.render_widget(block, centered);

    let inner = inner_of(centered, 2, 1);

    let filtered: Vec<&ScoreEntry> = match filter {
        None => entries.iter().collect(),
        Some(m) => {
            let lab = m.label();
            entries.iter().filter(|e| e.mode == lab).collect()
        }
    };

    if filtered.is_empty() {
        let p = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "Aucun score pour ce filtre.".to_string(),
                Style::default().fg(DARK_GREEN).italic(),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "[F] changer filtre   [Esc/Enter] retour".to_string(),
                Style::default().fg(DARK_GREEN),
            )),
        ])
        .alignment(Alignment::Center);
        f.render_widget(p, inner);
        return;
    }

    let mut by_day: BTreeMap<NaiveDate, Vec<&ScoreEntry>> = BTreeMap::new();
    for e in &filtered {
        by_day.entry(e.timestamp.date_naive()).or_default().push(e);
    }

    let today = Local::now().date_naive();
    let mut lines: Vec<Line> = Vec::new();

    for (day, mut day_entries) in by_day.into_iter().rev() {
        day_entries.sort_by(|a, b| b.score.cmp(&a.score));
        let day_label = day_label(day, today);
        lines.push(Line::from(Span::styled(
            format!("── {day_label} ── ({} runs)", day_entries.len()),
            Style::default().fg(GREEN).bold().add_modifier(Modifier::UNDERLINED),
        )));
        lines.push(Line::from(Span::styled(
            format!(
                "  {:>3}  {:<16}  {:<24}  {:<16}  {:<7}  {:>6}  {:>4}  {:>5}  {}",
                "#", "Nom", "Contact", "Mode", "Diff", "Score", "WPM", "Combo", "Heure"
            ),
            Style::default().fg(DARK_GREEN),
        )));
        for (i, e) in day_entries.iter().enumerate() {
            let rank = i + 1;
            let medal = match rank {
                1 => "🥇",
                2 => "🥈",
                3 => "🥉",
                _ => "  ",
            };
            let style = match rank {
                1 => Style::default().fg(Color::Rgb(255, 215, 0)).bold(),
                2 => Style::default().fg(Color::Rgb(192, 192, 192)).bold(),
                3 => Style::default().fg(Color::Rgb(205, 127, 50)).bold(),
                _ => Style::default().fg(GREEN),
            };
            lines.push(Line::from(Span::styled(
                format!(
                    " {medal}{:>3}  {:<16}  {:<24}  {:<16}  {:<7}  {:>6}  {:>4}  x{:>4}  {}",
                    rank,
                    truncate(&e.name, 16),
                    truncate(&e.contact, 24),
                    truncate(if e.mode.is_empty() { "—" } else { &e.mode }, 16),
                    e.difficulty,
                    e.score,
                    e.wpm,
                    e.max_combo,
                    e.timestamp.format("%H:%M"),
                ),
                style,
            )));
        }
        lines.push(Line::from(""));
    }
    lines.push(Line::from(Span::styled(
        "[Up/Down] scroll   [F] filtre mode   [P] vue joueurs   [Esc] retour".to_string(),
        Style::default().fg(DARK_GREEN),
    )));

    let total_lines = lines.len() as u16;
    let max_scroll = total_lines.saturating_sub(inner.height);
    let scroll = scroll.min(max_scroll);
    let p = Paragraph::new(lines).scroll((scroll, 0));
    f.render_widget(p, inner);
}
