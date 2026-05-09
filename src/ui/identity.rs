//! Écran de saisie nom + email/téléphone. Field cyclable, validation à la
//! soumission, message d'erreur affiché en rouge.

use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::IdField;
use crate::game::GameMode;
use crate::ui::common::{inner_of, label_line, DARK_GREEN, GREEN, RED};

pub(super) fn render_identity(
    f: &mut Frame,
    centered: Rect,
    mode: GameMode,
    name: &str,
    contact: &str,
    field: IdField,
    error: Option<&str>,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(GREEN))
        .title(format!(" {} — IDENTIFICATION ", mode.label()))
        .title_style(Style::default().fg(GREEN).bold());
    f.render_widget(block, centered);

    let inner = inner_of(centered, 1, 1);

    let name_active = field == IdField::Name;
    let contact_active = field == IdField::Contact;

    let lines = vec![
        Line::from(Span::styled(
            "Identifie-toi pour participer au concours :".to_string(),
            Style::default().fg(DARK_GREEN).italic(),
        )),
        Line::from(""),
        label_line("Nom / pseudo", name, name_active),
        Line::from(""),
        label_line("Email ou téléphone", contact, contact_active),
        Line::from(""),
        Line::from(Span::styled(
            error.unwrap_or("").to_string(),
            Style::default().fg(RED).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "[Tab/Up/Down] champ   [Enter] valider   [Esc] retour".to_string(),
            Style::default().fg(DARK_GREEN),
        )),
    ];
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}
