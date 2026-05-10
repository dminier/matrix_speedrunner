//! Couche de rendu — un sous-module par écran.
//!
//! Le point d'entrée unique est [`render`]. Il :
//!   1. resize l'effet de pluie si la taille du terminal a changé,
//!   2. dessine la pluie en arrière-plan sur toute la zone,
//!   3. délègue au sous-renderer correspondant à l'état courant de l'app.
//!
//! Chaque sous-module se concentre sur un seul écran et reste sous ~200 lignes.

mod common;
mod difficulty;
mod game_over;
mod identity;
mod menu;
mod play;
mod rules;
mod player_detail;
mod players;
mod scores;

use ratatui::Frame;

use crate::app::{App, Screen};
use crate::game::Session;
use crate::ui::common::{center_rect, clear_rect};

pub fn render(f: &mut Frame, app: &mut App) {
    let area = f.area();
    if app.rain.w != area.width || app.rain.h != area.height {
        app.rain.resize(area.width, area.height);
    }
    {
        let buf = f.buffer_mut();
        app.rain.render(area, buf);
    }

    // Synchronise la taille de l'aire de jeu avec le terminal AVANT que le
    // tick suivant ne tourne. Sans ça, les commandes du Speed Runner restent
    // sur une plage X figée à 80×24.
    if let Screen::Playing { session, .. } = &mut app.screen {
        session.set_area(area.width, area.height);
    }

    match &app.screen {
        Screen::Menu { selected } => {
            let r = center_rect(area, 70, 22);
            clear_rect(f, r);
            menu::render_menu(f, r, *selected);
        }
        Screen::Rules { scroll } => {
            let r = center_rect(area, 80, 24);
            clear_rect(f, r);
            rules::render_rules(f, r, *scroll);
        }
        Screen::EnterIdentity { mode, name, contact, field, error } => {
            let r = center_rect(area, 64, 16);
            clear_rect(f, r);
            identity::render_identity(f, r, *mode, name, contact, *field, error.as_deref());
        }
        Screen::DifficultySelect { mode, identity, selected } => {
            let r = center_rect(area, 56, 14);
            clear_rect(f, r);
            difficulty::render_difficulty(f, r, *mode, &identity.name, *selected);
        }
        Screen::Playing { session, identity } => match session {
            Session::SpeedRunner(s) => play::render_speedrunner(f, area, s, &identity.name),
            Session::Hack(s) => play::render_hack(f, area, s, &identity.name),
        },
        Screen::GameOver { identity, mode, difficulty, summary, saved } => {
            let r = center_rect(area, 60, 18);
            clear_rect(f, r);
            game_over::render_game_over(
                f, r, &identity.name, *mode, *difficulty, summary, *saved,
            );
        }
        Screen::Scores { entries, scroll, filter } => {
            let r = center_rect(area, area.width.saturating_sub(4), area.height.saturating_sub(2));
            clear_rect(f, r);
            scores::render_scores(f, r, entries, *scroll, *filter);
        }
        Screen::Players { entries, selected, scroll } => {
            let r = center_rect(area, area.width.saturating_sub(4), area.height.saturating_sub(2));
            clear_rect(f, r);
            players::render_players(f, r, entries, *selected, *scroll);
        }
        Screen::PlayerDetail { entries, key } => {
            let r = center_rect(area, area.width.saturating_sub(4), area.height.saturating_sub(2));
            clear_rect(f, r);
            player_detail::render_player_detail(f, r, entries, key);
        }
    }
}
