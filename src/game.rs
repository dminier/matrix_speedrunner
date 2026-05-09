use serde::{Deserialize, Serialize};

use crate::hack::HackState;
use crate::speedrunner::SpeedRunnerState;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Insane,
}

impl Difficulty {
    pub fn label(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Insane => "Insane",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    SpeedRunner,
    HackTimeAttack,
}

impl GameMode {
    pub const ALL: &'static [GameMode] = &[GameMode::SpeedRunner, GameMode::HackTimeAttack];

    pub fn label(self) -> &'static str {
        match self {
            GameMode::SpeedRunner => "Speed Runner",
            GameMode::HackTimeAttack => "Hack Time Attack",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            GameMode::SpeedRunner =>
                "Tape les commandes qui tombent avant qu'elles ne touchent le sol. 3 vies, score-attack.",
            GameMode::HackTimeAttack =>
                "Hack la Matrice : tape un maximum de commandes en 2 minutes. Score pondéré par longueur.",
        }
    }
}

pub struct Summary {
    pub score: u32,
    pub wpm: u32,
    pub max_combo: u32,
    pub elapsed_secs: u32,
    pub items_done: u32,
}

pub enum Session {
    SpeedRunner(SpeedRunnerState),
    Hack(HackState),
}

impl Session {
    pub fn new(mode: GameMode, diff: Difficulty) -> Self {
        match mode {
            GameMode::SpeedRunner => Session::SpeedRunner(SpeedRunnerState::new(diff)),
            GameMode::HackTimeAttack => Session::Hack(HackState::new(diff)),
        }
    }

    pub fn mode(&self) -> GameMode {
        match self {
            Session::SpeedRunner(_) => GameMode::SpeedRunner,
            Session::Hack(_) => GameMode::HackTimeAttack,
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        match self {
            Session::SpeedRunner(s) => s.diff,
            Session::Hack(s) => s.diff,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        match self {
            Session::SpeedRunner(s) => s.tick(dt),
            Session::Hack(s) => s.tick(dt),
        }
    }

    /// Synchronise la taille de l'aire de jeu avec le terminal courant.
    /// Pertinent uniquement pour Speed Runner (où le spawn dépend de la
    /// largeur). Hack Time Attack ignore cette information.
    pub fn set_area(&mut self, w: u16, h: u16) {
        if let Session::SpeedRunner(s) = self {
            s.set_area(w, h);
        }
    }

    pub fn type_char(&mut self, c: char) {
        match self {
            Session::SpeedRunner(s) => s.type_char(c),
            Session::Hack(s) => s.type_char(c),
        }
    }

    pub fn backspace(&mut self) {
        match self {
            Session::SpeedRunner(s) => s.backspace(),
            Session::Hack(s) => s.backspace(),
        }
    }

    pub fn submit(&mut self) {
        match self {
            Session::SpeedRunner(s) => s.submit(),
            Session::Hack(s) => s.submit(),
        }
    }

    pub fn is_over(&self) -> bool {
        match self {
            Session::SpeedRunner(s) => s.is_over(),
            Session::Hack(s) => s.is_over(),
        }
    }

    pub fn summary(&self) -> Summary {
        match self {
            Session::SpeedRunner(s) => Summary {
                score: s.score,
                wpm: s.wpm(),
                max_combo: s.max_combo,
                elapsed_secs: s.elapsed as u32,
                items_done: s.commands_completed,
            },
            Session::Hack(s) => Summary {
                score: s.score,
                wpm: s.wpm(),
                max_combo: s.max_combo,
                elapsed_secs: s.elapsed as u32,
                items_done: s.commands_done,
            },
        }
    }
}
