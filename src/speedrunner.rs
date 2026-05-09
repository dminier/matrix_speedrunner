use rand::Rng;

use crate::config::{random_speedrun_command, speedrunner_time_limit};
use crate::game::Difficulty;

pub struct FallingCommand {
    pub text: String,
    pub x: u16,
    pub y: f32,
}

pub struct SpeedRunnerState {
    pub diff: Difficulty,
    pub commands: Vec<FallingCommand>,
    pub buffer: String,
    pub flash_error: f32,
    pub score: u32,
    pub combo: u32,
    pub max_combo: u32,
    pub commands_completed: u32,
    pub lives: u32,
    pub elapsed: f32,
    pub time_limit: f32,
    pub spawn_timer: f32,
    pub keystrokes: u32,
    pub correct_keystrokes: u32,
    pub area_w: u16,
    pub area_h: u16,
}

fn base_fall_speed(d: Difficulty) -> f32 {
    match d {
        Difficulty::Easy => 1.5,
        Difficulty::Normal => 2.5,
        Difficulty::Insane => 3.5,
    }
}

fn base_spawn_interval(d: Difficulty) -> f32 {
    match d {
        Difficulty::Easy => 3.0,
        Difficulty::Normal => 2.0,
        Difficulty::Insane => 1.2,
    }
}

impl SpeedRunnerState {
    pub fn new(diff: Difficulty) -> Self {
        Self {
            diff,
            commands: Vec::new(),
            buffer: String::new(),
            flash_error: 0.0,
            score: 0,
            combo: 0,
            max_combo: 0,
            commands_completed: 0,
            lives: 3,
            elapsed: 0.0,
            time_limit: speedrunner_time_limit(),
            spawn_timer: 0.5,
            keystrokes: 0,
            correct_keystrokes: 0,
            area_w: 80,
            area_h: 24,
        }
    }

    pub fn is_over(&self) -> bool {
        self.lives == 0 || self.elapsed >= self.time_limit
    }

    pub fn time_remaining(&self) -> f32 {
        (self.time_limit - self.elapsed).max(0.0)
    }

    pub fn wpm(&self) -> u32 {
        if self.elapsed < 1.0 {
            return 0;
        }
        ((self.correct_keystrokes as f32 / 5.0) / (self.elapsed / 60.0)) as u32
    }

    fn difficulty_factor(&self) -> f32 {
        1.0 + (self.elapsed / 15.0) * 0.15
    }

    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
        if self.flash_error > 0.0 {
            self.flash_error = (self.flash_error - dt).max(0.0);
        }

        let speed = base_fall_speed(self.diff) * self.difficulty_factor();
        let max_y = self.area_h.saturating_sub(3) as f32;
        let mut missed = 0u32;
        self.commands.retain_mut(|c| {
            c.y += speed * dt;
            if c.y >= max_y {
                missed += 1;
                false
            } else {
                true
            }
        });
        if missed > 0 {
            self.lives = self.lives.saturating_sub(missed);
            self.combo = 0;
            self.buffer.clear();
        }

        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 {
            self.spawn();
            self.spawn_timer = base_spawn_interval(self.diff) / self.difficulty_factor();
        }
    }

    fn spawn(&mut self) {
        let mut rng = rand::thread_rng();
        let text = random_speedrun_command(self.diff, self.elapsed);
        let max_x = self.area_w.saturating_sub(text.len() as u16 + 2).max(1);
        let x = rng.gen_range(0..max_x);
        self.commands.push(FallingCommand { text, x, y: 1.0 });
    }

    pub fn type_char(&mut self, c: char) {
        self.keystrokes += 1;
        self.buffer.push(c);
        let ok = self.commands.iter().any(|cmd| cmd.text.starts_with(&self.buffer));
        if !ok {
            self.flash_error = 0.25;
            self.combo = 0;
            self.buffer.clear();
        } else {
            self.correct_keystrokes += 1;
        }
    }

    pub fn backspace(&mut self) {
        self.buffer.pop();
    }

    pub fn submit(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        let buf = self.buffer.clone();
        if let Some(idx) = self.commands.iter().position(|c| c.text == buf) {
            let removed = self.commands.remove(idx);
            self.combo += 1;
            self.commands_completed += 1;
            self.max_combo = self.max_combo.max(self.combo);
            let len = removed.text.len() as u32;
            let mult = 1 + self.combo / 5;
            self.score += len * 10 * mult;
            self.buffer.clear();
        } else {
            self.flash_error = 0.25;
            self.combo = 0;
            self.buffer.clear();
        }
    }
}
