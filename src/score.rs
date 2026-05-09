use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreEntry {
    pub name: String,
    pub contact: String,
    #[serde(default)]
    pub mode: String,
    pub difficulty: String,
    pub score: u32,
    pub wpm: u32,
    pub max_combo: u32,
    pub duration_secs: u32,
    #[serde(default)]
    pub items_done: u32,
    pub timestamp: DateTime<Local>,
}

pub fn data_dir() -> PathBuf {
    if let Some(d) = dirs::data_dir() {
        d.join("matrix_speedrunner")
    } else {
        PathBuf::from(".")
    }
}

pub fn scores_file() -> PathBuf {
    data_dir().join("scores.json")
}

pub fn scores_csv_file() -> PathBuf {
    data_dir().join("scores.csv")
}

// CRLF (RFC 4180) pour qu'Excel sous Windows découpe correctement les lignes.
const CSV_HEADER: &str =
    "timestamp,name,contact,mode,difficulty,score,wpm,max_combo,items_done,duration_secs\r\n";
// BOM UTF-8 : sans lui, Excel français interprète le fichier en Windows-1252
// et casse les accents (é è ç) et les caractères non-ASCII.
const UTF8_BOM: &str = "\u{FEFF}";

fn csv_escape(s: &str) -> String {
    // Selon RFC 4180 : on quote dès qu'il y a un séparateur, un guillemet,
    // un retour ligne ou un espace en bord. Les " internes sont doublés.
    let needs_quote = s
        .chars()
        .any(|c| c == ',' || c == '"' || c == '\n' || c == '\r' || c == ';');
    if needs_quote {
        let escaped = s.replace('"', "\"\"");
        format!("\"{escaped}\"")
    } else {
        s.to_string()
    }
}

fn entry_to_csv_row(e: &ScoreEntry) -> String {
    format!(
        "{},{},{},{},{},{},{},{},{},{}\r\n",
        // RFC3339 — Excel/LibreOffice/Pandas le parsent nativement.
        e.timestamp.to_rfc3339(),
        csv_escape(&e.name),
        csv_escape(&e.contact),
        csv_escape(&e.mode),
        csv_escape(&e.difficulty),
        e.score,
        e.wpm,
        e.max_combo,
        e.items_done,
        e.duration_secs,
    )
}

fn save_csv(entries: &[ScoreEntry]) -> std::io::Result<()> {
    let dir = data_dir();
    fs::create_dir_all(&dir)?;
    let mut out = String::with_capacity(UTF8_BOM.len() + CSV_HEADER.len() + entries.len() * 80);
    out.push_str(UTF8_BOM);
    out.push_str(CSV_HEADER);
    for e in entries {
        out.push_str(&entry_to_csv_row(e));
    }
    fs::write(scores_csv_file(), out)
}

pub fn load() -> Vec<ScoreEntry> {
    let Ok(s) = fs::read_to_string(scores_file()) else {
        return Vec::new();
    };
    serde_json::from_str(&s).unwrap_or_default()
}

pub fn save_all(entries: &[ScoreEntry]) -> std::io::Result<()> {
    let dir = data_dir();
    fs::create_dir_all(&dir)?;
    let s = serde_json::to_string_pretty(entries).expect("serialize scores");
    fs::write(scores_file(), s)?;
    // CSV est régénéré entièrement à chaque sauvegarde : c'est moins efficace
    // qu'un append, mais ça garde le fichier toujours cohérent même si
    // l'utilisateur édite manuellement le JSON.
    save_csv(entries)
}

/// Identité normalisée pour regrouper les runs d'un même joueur.
/// Le **contact** (email ou téléphone) est la clé : un même participant peut
/// saisir des variations de pseudo entre deux runs, mais son contact reste
/// stable. On normalise par trim + lowercase + on retire les espaces internes
/// (utile pour les numéros de tel saisis avec des espaces ou points).
pub fn identity_key(_name: &str, contact: &str) -> String {
    contact
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '.' && *c != '-')
        .collect()
}

pub struct PlayerSummary {
    pub key: String,
    pub name: String,
    pub contact: String,
    pub runs: u32,
    pub best_score: u32,
    pub last_played: chrono::DateTime<chrono::Local>,
}

/// Agrège les ScoreEntry par identité (clé = contact normalisé).
/// Trié par meilleur score décroissant. Le nom retenu pour l'affichage est
/// celui du run le plus récent — un joueur qui change de pseudo verra son
/// dernier pseudo affiché.
pub fn aggregate_players(entries: &[ScoreEntry]) -> Vec<PlayerSummary> {
    use std::collections::HashMap;
    let mut by_key: HashMap<String, PlayerSummary> = HashMap::new();
    for e in entries {
        let k = identity_key(&e.name, &e.contact);
        by_key
            .entry(k.clone())
            .and_modify(|p| {
                p.runs += 1;
                if e.score > p.best_score {
                    p.best_score = e.score;
                }
                if e.timestamp > p.last_played {
                    p.last_played = e.timestamp;
                    // le nom et le contact "officiels" sont ceux du dernier run
                    p.name = e.name.clone();
                    p.contact = e.contact.clone();
                }
            })
            .or_insert(PlayerSummary {
                key: k,
                name: e.name.clone(),
                contact: e.contact.clone(),
                runs: 1,
                best_score: e.score,
                last_played: e.timestamp,
            });
    }
    let mut v: Vec<_> = by_key.into_values().collect();
    v.sort_by(|a, b| b.best_score.cmp(&a.best_score));
    v
}

/// Tous les runs d'un joueur (clé = contact normalisé),
/// triés du plus ancien au plus récent.
pub fn runs_for_player<'a>(entries: &'a [ScoreEntry], key: &str) -> Vec<&'a ScoreEntry> {
    let mut v: Vec<_> = entries
        .iter()
        .filter(|e| identity_key(&e.name, &e.contact) == key)
        .collect();
    v.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    v
}

pub fn append(entry: ScoreEntry) -> std::io::Result<()> {
    let mut all = load();
    all.push(entry);
    save_all(&all)
}
