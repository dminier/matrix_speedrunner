//! Application principale : event loop ratatui à ~30 FPS, dispatch des
//! événements clavier vers les handlers par écran ([`handlers`]) et sauvegarde
//! automatique du score quand une partie se termine.

mod handlers;
mod screen;

use std::time::{Duration, Instant};

use chrono::Local;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::rain::Rain;
use crate::score::{self, ScoreEntry};
use crate::ui;

pub use screen::{IdField, Screen};

use handlers::Action;

const FRAME_MS: u64 = 33;

pub struct App {
    pub screen: Screen,
    pub rain: Rain,
    pub last_tick: Instant,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Menu { selected: 0 },
            rain: Rain::new(80, 24),
            last_tick: Instant::now(),
            running: true,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|f| ui::render(f, &mut self))?;

            if event::poll(Duration::from_millis(FRAME_MS))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match handlers::handle(&mut self.screen, key) {
                            Action::None => {}
                            Action::Quit => self.running = false,
                            Action::Goto(s) => self.screen = s,
                        }
                    }
                }
            }
            self.tick();
        }
        Ok(())
    }

    /// Avance la pluie et la session en cours. Si la session se termine,
    /// persiste le score et bascule vers l'écran [`Screen::GameOver`].
    fn tick(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = now;

        self.rain.tick(dt);
        if let Screen::Playing { session, identity } = &mut self.screen {
            session.tick(dt);
            if session.is_over() {
                let mode = session.mode();
                let difficulty = session.difficulty();
                let summary = session.summary();
                let entry = ScoreEntry {
                    name: identity.name.clone(),
                    contact: identity.contact.clone(),
                    mode: mode.label().to_string(),
                    difficulty: difficulty.label().to_string(),
                    score: summary.score,
                    wpm: summary.wpm,
                    max_combo: summary.max_combo,
                    duration_secs: summary.elapsed_secs,
                    items_done: summary.items_done,
                    timestamp: Local::now(),
                };
                let saved = score::append(entry).is_ok();
                self.screen = Screen::GameOver {
                    identity: identity.clone(),
                    mode,
                    difficulty,
                    summary,
                    saved,
                };
            }
        }
    }
}
