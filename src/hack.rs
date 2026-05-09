use crate::config::{hack_time_limit, random_hack_command};
use crate::game::Difficulty;

pub struct HackState {
    pub diff: Difficulty,
    pub current: String,
    pub buffer: String,
    pub flash_error: f32,
    pub last_complete_pulse: f32,
    pub score: u32,
    pub combo: u32,
    pub max_combo: u32,
    pub commands_done: u32,
    pub elapsed: f32,
    pub time_limit: f32,
    pub keystrokes: u32,
    pub correct_keystrokes: u32,
    pub errors: u32,
}

impl HackState {
    pub fn new(diff: Difficulty) -> Self {
        Self {
            current: random_hack_command(diff),
            diff,
            buffer: String::new(),
            flash_error: 0.0,
            last_complete_pulse: 0.0,
            score: 0,
            combo: 0,
            max_combo: 0,
            commands_done: 0,
            elapsed: 0.0,
            time_limit: hack_time_limit(),
            keystrokes: 0,
            correct_keystrokes: 0,
            errors: 0,
        }
    }

    pub fn time_remaining(&self) -> f32 {
        (self.time_limit - self.elapsed).max(0.0)
    }

    pub fn is_over(&self) -> bool {
        self.elapsed >= self.time_limit
    }

    pub fn wpm(&self) -> u32 {
        if self.elapsed < 1.0 {
            return 0;
        }
        ((self.correct_keystrokes as f32 / 5.0) / (self.elapsed / 60.0)) as u32
    }

    pub fn tick(&mut self, dt: f32) {
        if self.is_over() {
            return;
        }
        self.elapsed += dt;
        if self.flash_error > 0.0 {
            self.flash_error = (self.flash_error - dt).max(0.0);
        }
        if self.last_complete_pulse > 0.0 {
            self.last_complete_pulse = (self.last_complete_pulse - dt).max(0.0);
        }
    }

    pub fn type_char(&mut self, c: char) {
        if self.is_over() {
            return;
        }
        self.keystrokes += 1;
        let next_idx = self.buffer.chars().count();
        let expected = self.current.chars().nth(next_idx);
        if expected == Some(c) {
            self.buffer.push(c);
            self.correct_keystrokes += 1;
            if self.buffer == self.current {
                self.complete_command();
            }
        } else {
            self.flash_error = 0.2;
            self.errors += 1;
            self.combo = 0;
            self.buffer.clear();
        }
    }

    pub fn backspace(&mut self) {
        self.buffer.pop();
    }

    pub fn submit(&mut self) {
        if self.buffer == self.current {
            self.complete_command();
        }
    }

    fn complete_command(&mut self) {
        self.combo += 1;
        self.max_combo = self.max_combo.max(self.combo);
        self.commands_done += 1;

        // Score pondéré par longueur :
        //   - 5 pts par caractère (la base)
        //   - bonus quadratique léger pour les commandes longues : (len-10)*2 si > 10
        //   - multiplicateur de combo : 1 + combo/5
        let len = self.current.chars().count() as u32;
        let length_bonus = len.saturating_sub(10) * 2;
        let mult = 1 + self.combo / 5;
        self.score += (len * 5 + length_bonus) * mult;

        self.buffer.clear();
        self.last_complete_pulse = 0.18;
        // Évite de retomber sur la même commande deux fois de suite, mais
        // borne le nombre de tentatives : si le pool ne contient qu'un seul
        // item (ou si tous les pools sont vides et qu'on retombe toujours
        // sur le même fallback), on accepte la répétition au bout de 5 essais
        // pour éviter une boucle infinie qui freezerait le jeu.
        let prev = std::mem::take(&mut self.current);
        for _ in 0..5 {
            let next = random_hack_command(self.diff);
            if next != prev {
                self.current = next;
                return;
            }
        }
        self.current = random_hack_command(self.diff);
    }
}
