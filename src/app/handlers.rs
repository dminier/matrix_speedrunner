//! Dispatch des événements clavier par écran. Chaque fonction `on_*` est
//! responsable d'un seul écran ; elle peut consommer ou produire une nouvelle
//! transition (retournée via `Option<Screen>`) que le caller applique.

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::screen::{menu_items, valid_contact, IdField, Identity, MenuItem, Screen};
use crate::game::{Difficulty, GameMode, Session};
use crate::score;

/// Résultat d'un handler : transition d'écran, demande de quitter, ou rien.
pub enum Action {
    None,
    Quit,
    Goto(Screen),
}

pub fn handle(screen: &mut Screen, key: KeyEvent) -> Action {
    match screen {
        Screen::Menu { selected } => on_menu(selected, key),
        Screen::Rules { scroll } => on_rules(scroll, key),
        Screen::EnterIdentity { mode, name, contact, field, error } => {
            on_identity(*mode, name, contact, field, error, key)
        }
        Screen::DifficultySelect { mode, identity, selected } => {
            on_difficulty(*mode, identity, selected, key)
        }
        Screen::Playing { session, .. } => on_playing(session, key),
        Screen::GameOver { identity, mode, difficulty, .. } => {
            on_game_over(identity, *mode, *difficulty, key)
        }
        Screen::Scores { entries, scroll, filter } => on_scores(entries, scroll, filter, key),
        Screen::Players { entries, selected, scroll } => on_players(entries, selected, scroll, key),
        Screen::PlayerDetail { entries, .. } => on_player_detail(entries, key),
    }
}

fn on_menu(selected: &mut usize, key: KeyEvent) -> Action {
    let items = menu_items();
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
        KeyCode::Up => {
            *selected = selected.saturating_sub(1);
            Action::None
        }
        KeyCode::Down => {
            *selected = (*selected + 1).min(items.len() - 1);
            Action::None
        }
        KeyCode::Enter => match items.get(*selected).copied() {
            Some(MenuItem::Mode(mode)) => Action::Goto(Screen::EnterIdentity {
                mode,
                name: String::new(),
                contact: String::new(),
                field: IdField::Name,
                error: None,
            }),
            Some(MenuItem::Rules) => Action::Goto(Screen::Rules { scroll: 0 }),
            Some(MenuItem::Scores) => Action::Goto(Screen::Scores {
                entries: score::load(),
                scroll: 0,
                filter: None,
            }),
            Some(MenuItem::Quit) | None => Action::Quit,
        },
        _ => Action::None,
    }
}

fn on_rules(scroll: &mut u16, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
            Action::Goto(Screen::Menu { selected: 0 })
        }
        KeyCode::Up => {
            *scroll = scroll.saturating_sub(1);
            Action::None
        }
        KeyCode::Down => {
            *scroll = scroll.saturating_add(1);
            Action::None
        }
        KeyCode::PageUp => {
            *scroll = scroll.saturating_sub(8);
            Action::None
        }
        KeyCode::PageDown => {
            *scroll = scroll.saturating_add(8);
            Action::None
        }
        KeyCode::Home => {
            *scroll = 0;
            Action::None
        }
        _ => Action::None,
    }
}

fn on_identity(
    mode: GameMode,
    name: &mut String,
    contact: &mut String,
    field: &mut IdField,
    error: &mut Option<String>,
    key: KeyEvent,
) -> Action {
    match key.code {
        KeyCode::Esc => Action::Goto(Screen::Menu { selected: 0 }),
        KeyCode::Tab => {
            *field = if *field == IdField::Name { IdField::Contact } else { IdField::Name };
            Action::None
        }
        KeyCode::Up => { *field = IdField::Name; Action::None }
        KeyCode::Down => { *field = IdField::Contact; Action::None }
        KeyCode::Backspace => {
            let target = match field { IdField::Name => name, IdField::Contact => contact };
            target.pop();
            Action::None
        }
        KeyCode::Char(c) => {
            let target = match field { IdField::Name => name, IdField::Contact => contact };
            if target.chars().count() < 60 {
                target.push(c);
            }
            Action::None
        }
        KeyCode::Enter => {
            let n = name.trim();
            let c = contact.trim();
            if n.is_empty() {
                *error = Some("Le nom ne peut pas être vide.".into());
                *field = IdField::Name;
                Action::None
            } else if !valid_contact(c) {
                *error = Some("Email ou numéro de téléphone requis.".into());
                *field = IdField::Contact;
                Action::None
            } else {
                let identity = Identity { name: n.to_string(), contact: c.to_string() };
                Action::Goto(Screen::DifficultySelect { mode, identity, selected: 1 })
            }
        }
        _ => Action::None,
    }
}

fn on_difficulty(
    mode: GameMode,
    identity: &Identity,
    selected: &mut usize,
    key: KeyEvent,
) -> Action {
    match key.code {
        KeyCode::Esc => Action::Goto(Screen::Menu { selected: 0 }),
        KeyCode::Up => { *selected = selected.saturating_sub(1); Action::None }
        KeyCode::Down => { *selected = (*selected + 1).min(2); Action::None }
        KeyCode::Enter => {
            let diff = match *selected {
                0 => Difficulty::Easy,
                1 => Difficulty::Normal,
                _ => Difficulty::Insane,
            };
            Action::Goto(Screen::Playing {
                identity: identity.clone(),
                session: Session::new(mode, diff),
            })
        }
        _ => Action::None,
    }
}

fn on_playing(session: &mut Session, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::Goto(Screen::Menu { selected: 0 }),
        KeyCode::Enter => { session.submit(); Action::None }
        KeyCode::Backspace => { session.backspace(); Action::None }
        KeyCode::Char(c) => { session.type_char(c); Action::None }
        _ => Action::None,
    }
}

fn on_game_over(
    identity: &Identity,
    mode: GameMode,
    difficulty: Difficulty,
    key: KeyEvent,
) -> Action {
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Esc | KeyCode::Char('m') => Action::Goto(Screen::Menu { selected: 0 }),
        KeyCode::Char('s') => Action::Goto(Screen::Scores {
            entries: score::load(),
            scroll: 0,
            filter: None,
        }),
        KeyCode::Char('d') => Action::Goto(Screen::DifficultySelect {
            mode,
            identity: identity.clone(),
            selected: match difficulty {
                Difficulty::Easy => 0,
                Difficulty::Normal => 1,
                Difficulty::Insane => 2,
            },
        }),
        KeyCode::Char('r') | KeyCode::Enter => Action::Goto(Screen::Playing {
            identity: identity.clone(),
            session: Session::new(mode, difficulty),
        }),
        _ => Action::None,
    }
}

fn on_scores(
    entries: &mut Vec<crate::score::ScoreEntry>,
    scroll: &mut u16,
    filter: &mut Option<GameMode>,
    key: KeyEvent,
) -> Action {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Action::Goto(Screen::Menu { selected: 0 }),
        KeyCode::Char('p') => Action::Goto(Screen::Players {
            entries: entries.clone(),
            selected: 0,
            scroll: 0,
        }),
        KeyCode::Char('f') => {
            *filter = match *filter {
                None => Some(GameMode::SpeedRunner),
                Some(GameMode::SpeedRunner) => Some(GameMode::HackTimeAttack),
                Some(GameMode::HackTimeAttack) => None,
            };
            *scroll = 0;
            Action::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let max = entries.len() as u16;
            *scroll = (*scroll + 1).min(max.saturating_sub(1));
            Action::None
        }
        KeyCode::Up | KeyCode::Char('k') => { *scroll = scroll.saturating_sub(1); Action::None }
        KeyCode::PageDown => { *scroll = scroll.saturating_add(10); Action::None }
        KeyCode::PageUp => { *scroll = scroll.saturating_sub(10); Action::None }
        _ => Action::None,
    }
}

fn on_players(
    entries: &mut Vec<crate::score::ScoreEntry>,
    selected: &mut usize,
    scroll: &mut u16,
    key: KeyEvent,
) -> Action {
    let players = score::aggregate_players(entries);
    let count = players.len();
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Esc => Action::Goto(Screen::Scores {
            entries: entries.clone(),
            scroll: 0,
            filter: None,
        }),
        KeyCode::Down | KeyCode::Char('j') => {
            if count > 0 {
                *selected = (*selected + 1).min(count - 1);
                if *selected as u16 >= scroll.saturating_add(10) {
                    *scroll = scroll.saturating_add(1);
                }
            }
            Action::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            *selected = selected.saturating_sub(1);
            if (*selected as u16) < *scroll {
                *scroll = *selected as u16;
            }
            Action::None
        }
        KeyCode::PageDown => {
            *selected = (*selected + 10).min(count.saturating_sub(1));
            *scroll = scroll.saturating_add(10);
            Action::None
        }
        KeyCode::PageUp => {
            *selected = selected.saturating_sub(10);
            *scroll = scroll.saturating_sub(10);
            Action::None
        }
        KeyCode::Enter => {
            if let Some(p) = players.get(*selected) {
                Action::Goto(Screen::PlayerDetail {
                    entries: entries.clone(),
                    key: p.key.clone(),
                })
            } else {
                Action::None
            }
        }
        _ => Action::None,
    }
}

fn on_player_detail(entries: &mut Vec<crate::score::ScoreEntry>, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Esc | KeyCode::Backspace | KeyCode::Enter => Action::Goto(Screen::Players {
            entries: entries.clone(),
            selected: 0,
            scroll: 0,
        }),
        _ => Action::None,
    }
}
