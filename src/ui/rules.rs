//! Écran "Rules" : règles du jeu, contrôles, calcul du score.
//! Texte scrollable verticalement (Up/Down, PageUp/PageDown).

use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::ui::common::{DARK_GREEN, GREEN};

pub(super) fn render_rules(f: &mut Frame, area: Rect, scroll: u16) {
    let title_style = Style::default().fg(GREEN).bold();
    let body_style = Style::default().fg(GREEN);
    let dim = Style::default().fg(DARK_GREEN);
    let accent = Style::default().fg(Color::White).bold();

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("MODES DE JEU", title_style)));
    lines.push(Line::from(Span::styled("─".repeat(40), dim)));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::styled("Speed Runner ", accent),
        Span::styled("— score-attack, 3 vies.", body_style),
    ]));
    lines.push(Line::from(Span::styled(
        "Des commandes shell défilent à l'écran. Tape-les exactement.",
        body_style,
    )));
    lines.push(Line::from(Span::styled(
        "Une commande ratée = une vie en moins. Plus tu enchaînes,",
        body_style,
    )));
    lines.push(Line::from(Span::styled(
        "plus le combo grimpe.",
        body_style,
    )));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::styled("Hack Time Attack ", accent),
        Span::styled("— sprint de 2 minutes.", body_style),
    ]));
    lines.push(Line::from(Span::styled(
        "Tape un maximum de commandes correctes en 120 secondes.",
        body_style,
    )));
    lines.push(Line::from(Span::styled(
        "Le score est pondéré par la longueur des commandes : viser",
        body_style,
    )));
    lines.push(Line::from(Span::styled(
        "des chaînes longues tapées juste rapporte plus.",
        body_style,
    )));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled("DIFFICULTÉS", title_style)));
    lines.push(Line::from(Span::styled("─".repeat(40), dim)));
    lines.push(Line::from(vec![
        Span::styled("Easy   ", accent),
        Span::styled("commandes courtes, vitesse de chute lente.", body_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Normal ", accent),
        Span::styled("équilibre standard, recommandé.", body_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Insane ", accent),
        Span::styled("commandes longues et obscures, chute rapide.", body_style),
    ]));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled("CONTRÔLES", title_style)));
    lines.push(Line::from(Span::styled("─".repeat(40), dim)));
    lines.push(Line::from(vec![
        Span::styled("[Tape]      ", accent),
        Span::styled("saisis la commande à l'écran.", body_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("[Backspace] ", accent),
        Span::styled("efface le dernier caractère.", body_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("[Enter]     ", accent),
        Span::styled("valide une saisie (selon le mode).", body_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("[Esc / Q]   ", accent),
        Span::styled("quitter la partie ou revenir en arrière.", body_style),
    ]));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled("SCORING & WPM", title_style)));
    lines.push(Line::from(Span::styled("─".repeat(40), dim)));
    lines.push(Line::from(Span::styled(
        "Le WPM (mots par minute) est calculé sur 5 caractères = 1 mot,",
        body_style,
    )));
    lines.push(Line::from(Span::styled(
        "norme dactylographie standard. Le combo multiplie les points",
        body_style,
    )));
    lines.push(Line::from(Span::styled(
        "tant que tu enchaînes sans erreur. Une faute le réinitialise.",
        body_style,
    )));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled("CONCOURS", title_style)));
    lines.push(Line::from(Span::styled("─".repeat(40), dim)));
    lines.push(Line::from(Span::styled(
        "Chaque run termine par un enregistrement automatique du score",
        body_style,
    )));
    lines.push(Line::from(Span::styled(
        "(nom + contact + horodatage). Les classements quotidiens",
        body_style,
    )));
    lines.push(Line::from(Span::styled(
        "sont consultables depuis l'option Scores du menu principal.",
        body_style,
    )));
    lines.push(Line::from(""));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(GREEN))
        .title(" RULES ")
        .title_style(Style::default().fg(GREEN).bold());

    let footer = Line::from(Span::styled(
        "[Up/Down] scroll   [PgUp/PgDn] page   [Esc/Q] retour menu",
        dim,
    ))
    .alignment(Alignment::Center);

    let body = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    f.render_widget(body, area);

    // Footer overlay sur la dernière ligne intérieure de la rect.
    if area.height >= 2 {
        let footer_rect = Rect {
            x: area.x + 1,
            y: area.y + area.height - 2,
            width: area.width.saturating_sub(2),
            height: 1,
        };
        f.render_widget(Paragraph::new(footer), footer_rect);
    }
}
