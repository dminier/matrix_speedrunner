//! Fiche d'un joueur : header avec stats + courbe de progression
//! (ratatui::Chart à 2 datasets) + table des 5 derniers runs.

use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph};
use ratatui::Frame;

use crate::game::GameMode;
use crate::score::{self, ScoreEntry};
use crate::ui::common::{chip, inner_of, truncate, BRIGHT_GREEN, DARK_GREEN, GREEN};

pub(super) fn render_player_detail(f: &mut Frame, centered: Rect, entries: &[ScoreEntry], key: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(GREEN))
        .title(" FICHE JOUEUR ")
        .title_style(Style::default().fg(GREEN).bold());
    f.render_widget(block, centered);
    let inner = inner_of(centered, 2, 1);

    let runs = score::runs_for_player(entries, key);
    if runs.is_empty() {
        let p = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "Aucun run pour ce joueur.".to_string(),
                Style::default().fg(DARK_GREEN).italic(),
            )),
        ])
        .alignment(Alignment::Center);
        f.render_widget(p, inner);
        return;
    }

    // Layout : header | chart | recent runs | footer
    let [header, chart_area, list_area, footer] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(10),
        Constraint::Length(8),
        Constraint::Length(1),
    ])
    .areas(inner);

    // ---- Header : identité + stats ---------------------------------
    let first = runs.first().unwrap();
    let best_score = runs.iter().map(|r| r.score).max().unwrap_or(0);
    let total_score: u64 = runs.iter().map(|r| r.score as u64).sum();
    let avg_score = (total_score as f64 / runs.len() as f64) as u32;
    let max_combo = runs.iter().map(|r| r.max_combo).max().unwrap_or(0);
    let max_wpm = runs.iter().map(|r| r.wpm).max().unwrap_or(0);

    let sr_runs = runs.iter().filter(|r| r.mode == GameMode::SpeedRunner.label()).count();
    let hk_runs = runs.iter().filter(|r| r.mode == GameMode::HackTimeAttack.label()).count();

    let header_lines = vec![
        Line::from(vec![
            Span::styled(
                format!(" {} ", first.name),
                Style::default().fg(Color::Black).bg(GREEN).bold(),
            ),
            Span::raw("  "),
            Span::styled(
                first.contact.to_string(),
                Style::default().fg(DARK_GREEN),
            ),
        ]),
        Line::from(vec![
            chip(" Runs ", &format!(" {} ", runs.len())),
            Span::raw("  "),
            chip(" Best ", &format!(" {} ", best_score)),
            Span::raw("  "),
            chip(" Avg ", &format!(" {} ", avg_score)),
            Span::raw("  "),
            chip(" Combo max ", &format!(" x{} ", max_combo)),
            Span::raw("  "),
            chip(" WPM max ", &format!(" {} ", max_wpm)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("Speed Runner : {sr_runs} runs"),
                Style::default().fg(GREEN),
            ),
            Span::raw("    "),
            Span::styled(
                format!("Hack Time Attack : {hk_runs} runs"),
                Style::default().fg(BRIGHT_GREEN),
            ),
        ]),
        Line::from(""),
    ];
    f.render_widget(Paragraph::new(header_lines), header);

    // ---- Chart : score par run, deux datasets ---------------------
    let sr_label = GameMode::SpeedRunner.label();
    let hk_label = GameMode::HackTimeAttack.label();
    let sr_data: Vec<(f64, f64)> = runs
        .iter()
        .enumerate()
        .filter(|(_, r)| r.mode == sr_label)
        .map(|(i, r)| ((i + 1) as f64, r.score as f64))
        .collect();
    let hk_data: Vec<(f64, f64)> = runs
        .iter()
        .enumerate()
        .filter(|(_, r)| r.mode == hk_label)
        .map(|(i, r)| ((i + 1) as f64, r.score as f64))
        .collect();

    let max_x = runs.len() as f64;
    let max_y = (best_score as f64 * 1.15).max(100.0);

    let datasets = vec![
        Dataset::default()
            .name(sr_label)
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(GREEN))
            .data(&sr_data),
        Dataset::default()
            .name(hk_label)
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(BRIGHT_GREEN))
            .data(&hk_data),
    ];

    let mid_x = (max_x / 2.0).round();
    let mid_y = (max_y / 2.0).round();
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(DARK_GREEN))
                .title(" Progression — score par run ")
                .title_style(Style::default().fg(GREEN).bold()),
        )
        .x_axis(
            Axis::default()
                .title("run #")
                .style(Style::default().fg(DARK_GREEN))
                .bounds([0.5, max_x + 0.5])
                .labels::<Vec<Span>>(vec![
                    Span::raw("1"),
                    Span::raw(format!("{}", mid_x as u32)),
                    Span::raw(format!("{}", runs.len())),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("score")
                .style(Style::default().fg(DARK_GREEN))
                .bounds([0.0, max_y])
                .labels::<Vec<Span>>(vec![
                    Span::raw("0"),
                    Span::raw(format!("{}", mid_y as u32)),
                    Span::raw(format!("{}", max_y as u32)),
                ]),
        );
    f.render_widget(chart, chart_area);

    // ---- Liste des derniers runs (5 plus récents) ------------------
    let mut last5: Vec<&&ScoreEntry> = runs.iter().rev().take(5).collect();
    last5.reverse();
    let mut list_lines = vec![Line::from(Span::styled(
        format!(
            "  {:<16}  {:<10}  {:<7}  {:>6}  {:>4}  {:>5}",
            "Quand", "Mode", "Diff", "Score", "WPM", "Combo"
        ),
        Style::default().fg(DARK_GREEN).bold(),
    ))];
    for r in last5 {
        list_lines.push(Line::from(Span::styled(
            format!(
                "  {:<16}  {:<10}  {:<7}  {:>6}  {:>4}  x{:>4}",
                r.timestamp.format("%d/%m %H:%M").to_string(),
                truncate(&r.mode, 10),
                r.difficulty,
                r.score,
                r.wpm,
                r.max_combo
            ),
            Style::default().fg(GREEN),
        )));
    }
    f.render_widget(
        Paragraph::new(list_lines).block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(DARK_GREEN))
                .title(" Derniers runs ")
                .title_style(Style::default().fg(DARK_GREEN)),
        ),
        list_area,
    );

    // ---- Footer ----------------------------------------------------
    f.render_widget(
        Paragraph::new(Span::styled(
            "[Esc/Enter] retour liste joueurs   [Q] quitter".to_string(),
            Style::default().fg(DARK_GREEN),
        ))
        .alignment(Alignment::Center),
        footer,
    );
}
