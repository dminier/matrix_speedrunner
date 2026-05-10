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
        /// Index dans la liste retournée par [`menu_items`] — source de vérité unique.
        selected: usize,
    },
    Rules {
        scroll: u16,
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

/// Item sélectionnable du menu principal. Source de vérité unique pour
/// l'ordre, les libellés et les descriptions affichés et le routing handler.
#[derive(Clone, Copy)]
pub enum MenuItem {
    Mode(GameMode),
    Rules,
    Scores,
    Quit,
}

impl MenuItem {
    pub fn label(self) -> &'static str {
        match self {
            MenuItem::Mode(m) => m.label(),
            MenuItem::Rules => "Rules",
            MenuItem::Scores => "Scores",
            MenuItem::Quit => "Quit",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            MenuItem::Mode(m) => m.description(),
            MenuItem::Rules => "Règles du jeu, contrôles et calcul du score.",
            MenuItem::Scores => "Tableau des scores du concours, par jour.",
            MenuItem::Quit => "Sortir de la Matrice.",
        }
    }
}

/// Construit la liste ordonnée des items du menu : modes de jeu, puis Rules,
/// Scores, Quit. Appelée par le rendu et par le handler clavier.
pub fn menu_items() -> Vec<MenuItem> {
    let mut v: Vec<MenuItem> = GameMode::ALL.iter().map(|&m| MenuItem::Mode(m)).collect();
    v.push(MenuItem::Rules);
    v.push(MenuItem::Scores);
    v.push(MenuItem::Quit);
    v
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
