//! Configuration du jeu : durées par mode + pools de commandes.
//!
//! Une seule lecture au premier accès via [`OnceLock`]. Ordre de résolution :
//!   1. fichier utilisateur `<config_dir>/matrix_speedrunner/config.toml`
//!   2. fichier embarqué dans le binaire (`assets/config.toml`).
//!
//! En cas de fichier utilisateur invalide, fallback silencieux sur les
//! valeurs embarquées : un fichier de config cassé ne doit pas planter le jeu.

use std::path::PathBuf;
use std::sync::OnceLock;

use rand::seq::SliceRandom;
use rand::Rng;
use serde::Deserialize;

use crate::game::Difficulty;

const DEFAULT_TOML: &str = include_str!("../assets/config.toml");

#[derive(Debug, Default, Deserialize)]
struct Config {
    #[serde(default)]
    speedrunner: SpeedrunnerSettings,
    #[serde(default)]
    hack: HackSettings,
    #[serde(default)]
    commands: CommandsConfig,
}

#[derive(Debug, Deserialize)]
struct SpeedrunnerSettings {
    #[serde(default = "default_speedrunner_time")]
    time_limit_secs: f32,
}

#[derive(Debug, Deserialize)]
struct HackSettings {
    #[serde(default = "default_hack_time")]
    time_limit_secs: f32,
}

fn default_speedrunner_time() -> f32 { 180.0 }
fn default_hack_time() -> f32 { 120.0 }

impl Default for SpeedrunnerSettings {
    fn default() -> Self {
        Self { time_limit_secs: default_speedrunner_time() }
    }
}
impl Default for HackSettings {
    fn default() -> Self {
        Self { time_limit_secs: default_hack_time() }
    }
}

#[derive(Debug, Default, Deserialize)]
struct CommandsConfig {
    #[serde(default)]
    speedrunner: ModePools,
    #[serde(default)]
    hack: ModePools,
}

#[derive(Debug, Default, Deserialize)]
struct ModePools {
    #[serde(default)] tier1: Vec<String>,
    #[serde(default)] tier2: Vec<String>,
    #[serde(default)] tier3: Vec<String>,
    #[serde(default)] easter: Vec<String>,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn user_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("matrix_speedrunner").join("config.toml"))
}

fn load() -> Config {
    if let Some(p) = user_config_path() {
        if let Ok(s) = std::fs::read_to_string(&p) {
            if let Ok(cfg) = toml::from_str(&s) {
                return cfg;
            }
        }
    }
    toml::from_str(DEFAULT_TOML).expect("bundled config.toml is invalid")
}

fn config() -> &'static Config {
    CONFIG.get_or_init(load)
}

// ===== Durées exposées aux modes ======================================

pub fn speedrunner_time_limit() -> f32 {
    config().speedrunner.time_limit_secs.max(1.0)
}

pub fn hack_time_limit() -> f32 {
    config().hack.time_limit_secs.max(1.0)
}

// ===== Pools de commandes =============================================

fn pick<R: Rng>(rng: &mut R, pool: &[String], fallback: &str) -> String {
    pool.choose(rng)
        .cloned()
        .unwrap_or_else(|| fallback.to_string())
}

fn fallback_from(pools: [&[String]; 3], default: &str) -> Option<String> {
    let mut rng = rand::thread_rng();
    for p in pools {
        if !p.is_empty() {
            return Some(pick(&mut rng, p, default));
        }
    }
    None
}

pub fn random_speedrun_command(diff: Difficulty, elapsed: f32) -> String {
    let cfg = &config().commands.speedrunner;
    let mut rng = rand::thread_rng();

    if !cfg.easter.is_empty() && rng.gen_bool(0.04) {
        return pick(&mut rng, &cfg.easter, "wake up neo");
    }

    let pool: &[String] = match diff {
        Difficulty::Easy => &cfg.tier1,
        Difficulty::Normal => {
            if elapsed < 30.0 || rng.gen_bool(0.4) { &cfg.tier1 } else { &cfg.tier2 }
        }
        Difficulty::Insane => {
            let r: f32 = rng.gen();
            if r < 0.2 { &cfg.tier1 }
            else if r < 0.65 { &cfg.tier2 }
            else { &cfg.tier3 }
        }
    };

    if !pool.is_empty() {
        pick(&mut rng, pool, "ls")
    } else {
        fallback_from([&cfg.tier1, &cfg.tier2, &cfg.tier3], "ls").unwrap_or_else(|| "ls".to_string())
    }
}

pub fn random_hack_command(diff: Difficulty) -> String {
    let cfg = &config().commands.hack;
    let mut rng = rand::thread_rng();

    if !cfg.easter.is_empty() && rng.gen_bool(0.05) {
        return pick(&mut rng, &cfg.easter, "i know kung fu");
    }

    let pool: &[String] = match diff {
        Difficulty::Easy => &cfg.tier1,
        Difficulty::Normal => {
            if rng.gen_bool(0.5) { &cfg.tier1 } else { &cfg.tier2 }
        }
        Difficulty::Insane => {
            let r: f32 = rng.gen();
            if r < 0.2 { &cfg.tier1 }
            else if r < 0.6 { &cfg.tier2 }
            else { &cfg.tier3 }
        }
    };

    if !pool.is_empty() {
        pick(&mut rng, pool, "ls /etc")
    } else {
        fallback_from([&cfg.tier1, &cfg.tier2, &cfg.tier3], "ls /etc")
            .unwrap_or_else(|| "ls /etc".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_toml_parses() {
        let cfg: Config = toml::from_str(DEFAULT_TOML).expect("toml ok");
        assert!(!cfg.commands.speedrunner.tier1.is_empty());
        assert!(!cfg.commands.hack.tier1.is_empty());
        assert!(cfg.speedrunner.time_limit_secs > 0.0);
        assert!(cfg.hack.time_limit_secs > 0.0);
    }

    #[test]
    fn defaults_apply_when_keys_missing() {
        let cfg: Config = toml::from_str("").expect("empty toml ok");
        assert_eq!(cfg.speedrunner.time_limit_secs, 180.0);
        assert_eq!(cfg.hack.time_limit_secs, 120.0);
    }
}
