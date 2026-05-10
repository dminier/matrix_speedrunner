//! Persistance des scores : un unique fichier `score.xlsx` (Office Excel),
//! placé **dans le même dossier que l'exécutable**. Pas d'autre fichier local.

use std::fs;
use std::path::PathBuf;

use calamine::{open_workbook_auto, Data, Reader};
use chrono::{DateTime, Local, TimeZone};
use rust_xlsxwriter::{Format, FormatBorder, Workbook};
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

/// Dossier qui contient l'exécutable courant (fallback : répertoire courant).
fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Chemin absolu du fichier de scores Excel.
pub fn scores_file() -> PathBuf {
    exe_dir().join("score.xlsx")
}

const HEADERS: [&str; 10] = [
    "timestamp",
    "name",
    "contact",
    "mode",
    "difficulty",
    "score",
    "wpm",
    "max_combo",
    "items_done",
    "duration_secs",
];

pub fn load() -> Vec<ScoreEntry> {
    let path = scores_file();
    if !path.exists() {
        return Vec::new();
    }
    let mut wb = match open_workbook_auto(&path) {
        Ok(wb) => wb,
        Err(e) => {
            eprintln!("[matrix_speedrunner] {} illisible : {e}", path.display());
            return Vec::new();
        }
    };
    let sheet_name = wb
        .sheet_names()
        .first()
        .cloned()
        .unwrap_or_else(|| "Scores".to_string());
    let range = match wb.worksheet_range(&sheet_name) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[matrix_speedrunner] feuille {sheet_name} : {e}");
            return Vec::new();
        }
    };
    let mut out = Vec::new();
    for row in range.rows().skip(1) {
        // Saute les lignes incomplètes ou en-tête mal placée.
        if row.len() < HEADERS.len() {
            continue;
        }
        let ts = parse_timestamp(&row[0]);
        let entry = ScoreEntry {
            timestamp: ts,
            name: cell_string(&row[1]),
            contact: cell_string(&row[2]),
            mode: cell_string(&row[3]),
            difficulty: cell_string(&row[4]),
            score: cell_u32(&row[5]),
            wpm: cell_u32(&row[6]),
            max_combo: cell_u32(&row[7]),
            items_done: cell_u32(&row[8]),
            duration_secs: cell_u32(&row[9]),
        };
        out.push(entry);
    }
    out
}

fn cell_string(c: &Data) -> String {
    match c {
        Data::String(s) => s.clone(),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(dt) => dt.to_string(),
        Data::DateTimeIso(s) | Data::DurationIso(s) => s.clone(),
        Data::Empty | Data::Error(_) => String::new(),
    }
}

fn cell_u32(c: &Data) -> u32 {
    match c {
        Data::Int(i) => (*i).max(0) as u32,
        Data::Float(f) => f.round().max(0.0) as u32,
        Data::String(s) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn parse_timestamp(c: &Data) -> DateTime<Local> {
    match c {
        Data::DateTimeIso(s) | Data::String(s) => DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Local))
            .unwrap_or_else(|_| Local::now()),
        Data::DateTime(dt) => {
            // Excel datetime sérialisé en jours depuis 1900-01-01.
            let secs = ((dt.as_f64() - 25569.0) * 86400.0) as i64;
            Local.timestamp_opt(secs, 0).single().unwrap_or_else(Local::now)
        }
        _ => Local::now(),
    }
}

/// Réécriture atomique : écrit dans un .tmp puis rename.
fn write_atomic_xlsx(path: &std::path::Path, entries: &[ScoreEntry]) -> std::io::Result<()> {
    let tmp = path.with_extension("xlsx.tmp");
    let mut wb = Workbook::new();
    let header_fmt = Format::new()
        .set_bold()
        .set_background_color("#003300")
        .set_font_color("#00FF41")
        .set_border(FormatBorder::Thin);
    let ws = wb
        .add_worksheet()
        .set_name("Scores")
        .map_err(|e| std::io::Error::other(e.to_string()))?;

    for (col, h) in HEADERS.iter().enumerate() {
        ws.write_string_with_format(0, col as u16, *h, &header_fmt)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
    }

    for (i, e) in entries.iter().enumerate() {
        let row = (i + 1) as u32;
        // Timestamp en chaîne RFC 3339 pour rester lisible et trier alphabétiquement.
        ws.write_string(row, 0, e.timestamp.to_rfc3339()).ok();
        ws.write_string(row, 1, &e.name).ok();
        ws.write_string(row, 2, &e.contact).ok();
        ws.write_string(row, 3, &e.mode).ok();
        ws.write_string(row, 4, &e.difficulty).ok();
        ws.write_number(row, 5, e.score as f64).ok();
        ws.write_number(row, 6, e.wpm as f64).ok();
        ws.write_number(row, 7, e.max_combo as f64).ok();
        ws.write_number(row, 8, e.items_done as f64).ok();
        ws.write_number(row, 9, e.duration_secs as f64).ok();
    }

    // Largeurs raisonnables.
    let widths = [25.0, 18.0, 24.0, 18.0, 12.0, 8.0, 8.0, 11.0, 12.0, 14.0];
    for (col, w) in widths.iter().enumerate() {
        ws.set_column_width(col as u16, *w).ok();
    }

    wb.save(&tmp)
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    fs::rename(&tmp, path)
}

pub fn save_all(entries: &[ScoreEntry]) -> std::io::Result<()> {
    let path = scores_file();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    write_atomic_xlsx(&path, entries)
}

/// Identité normalisée pour regrouper les runs d'un même joueur.
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
