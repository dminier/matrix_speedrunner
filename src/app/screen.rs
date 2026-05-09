//! Types décrivant l'état de l'UI : identité du joueur en cours, et l'enum
//! [`Screen`] qui modélise la state machine de l'application.

use crate::game::{Difficulty, GameMode, Session, Summary};
use crate::score::ScoreEntry;

#[derive(Clone)]
pub struct Identity {
    pub name: String,
    pub contact: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum IdField {
    Name,
    Contact,
}

pub enum Screen {
    Menu {
        /// 0..N modes, puis Scores, puis Quit.
        selected: usize,
    },
    EnterIdentity {
        mode: GameMode,
        name: String,
        contact: String,
        field: IdField,
        error: Option<String>,
    },
    DifficultySelect {
        mode: GameMode,
        identity: Identity,
        selected: usize,
    },
    Playing {
        identity: Identity,
        session: Session,
    },
    GameOver {
        identity: Identity,
        mode: GameMode,
        difficulty: Difficulty,
        summary: Summary,
        saved: bool,
    },
    Scores {
        entries: Vec<ScoreEntry>,
        scroll: u16,
        filter: Option<GameMode>,
    },
    Players {
        entries: Vec<ScoreEntry>,
        selected: usize,
        scroll: u16,
    },
    PlayerDetail {
        entries: Vec<ScoreEntry>,
        key: String,
    },
}

/// Validation légère : email (`@` + `.`, ≥ 5 chars) ou téléphone (≥ 8 chiffres).
pub fn valid_contact(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let is_email = s.contains('@') && s.contains('.') && s.len() >= 5;
    let digits = s.chars().filter(|c| c.is_ascii_digit()).count();
    let is_phone = digits >= 8;
    is_email || is_phone
}
