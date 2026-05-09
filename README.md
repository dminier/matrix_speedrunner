# Matrix Speedrunner

Mini-jeu de salon en ligne de commande, thème **Matrix**. Format de runs courts
et compétitifs, pensé pour un stand : on saisit son nom, on joue, le score
horodaté est enregistré, et un classement par jour s'affiche.

```
  __  __    _  _____ ____  ___ __  __
 |  \/  |  / \|_   _|  _ \|_ _|\ \/ /
 | |\/| | / _ \ | | | |_) || |  \  /
 | |  | |/ ___ \| | |  _ < | |  /  \
 |_|  |_/_/   \_\_| |_| \_\___|/_/\_\
        S P E E D R U N N E R
```

## Sommaire

- [Modes de jeu](#modes-de-jeu)
- [Installation](#installation)
  - [Linux (binaire pré-compilé)](#linux-binaire-pré-compilé)
  - [Windows (binaire pré-compilé)](#windows-binaire-pré-compilé)
  - [Build depuis les sources](#build-depuis-les-sources)
- [Lancement](#lancement)
- [Contrôles](#contrôles)
- [Stockage des scores](#stockage-des-scores)
- [Construire des paquets de release](#construire-des-paquets-de-release)
- [Architecture du code](#architecture-du-code)
- [Crates utilisées](#crates-utilisées)
- [Licence](#licence)

## Modes de jeu

### 1. Speed Runner — *score-attack*

Des commandes shell tombent du haut de l'écran (dans une vraie pluie de glyphes
Matrix). Tu dois les retaper avant qu'elles touchent le sol. Tu as **3 vies**.
Plus tu enchaînes sans erreur, plus le combo monte et le multiplicateur de
score grimpe. La difficulté augmente avec le temps.

- Score = `longueur(cmd) × 10 × (1 + combo / 5)`
- Game over : 3 commandes ratées (touchent le sol).

### 2. Hack Time Attack — *2 minutes chrono*

Une commande de hacking apparaît, tu la tapes, elle disparaît, la suivante
apparaît. Tu as **120 secondes** pour en valider un maximum. Le score est
**pondéré par la longueur** des commandes : viser les plus complexes paie plus.

- Score par commande = `(len × 5 + max(0, len − 10) × 2) × (1 + combo / 5)`
- Auto-validation dès que le buffer matche la commande.
- Erreur de touche : reset du buffer + reset combo (pas de pénalité de score).

## Installation

### Pré-requis communs

- Un terminal avec **truecolor** (24-bit) pour le rendu vert phosphore (Windows
  Terminal, Alacritty, kitty, GNOME Terminal, iTerm2, etc.).
- Une police monospace contenant le **katakana demi-largeur** (`ｱｲｳｴｵ…`) :
  Cascadia Code, Consolas, JetBrains Mono, Fira Code, Menlo… toutes les polices
  modernes "developer" font le boulot.

### Linux (binaire pré-compilé)

Télécharger la dernière release depuis la page [Releases](../../releases) :

```bash
# Remplace VERSION par le tag de la release
curl -L -o matrix_speedrunner.tar.gz \
    https://github.com/<owner>/matrix_speedrunner/releases/download/VERSION/matrix_speedrunner-VERSION-linux-x86_64.tar.gz

# Vérifier le checksum (recommandé)
curl -L -o matrix_speedrunner.tar.gz.sha256 \
    https://github.com/<owner>/matrix_speedrunner/releases/download/VERSION/matrix_speedrunner-VERSION-linux-x86_64.tar.gz.sha256
sha256sum -c matrix_speedrunner.tar.gz.sha256

tar xzf matrix_speedrunner.tar.gz
cd matrix_speedrunner-*-linux-x86_64

# Test
./matrix_speedrunner

# Installation système (optionnel)
sudo install -m 755 matrix_speedrunner /usr/local/bin/
```

### Windows (binaire pré-compilé)

1. Télécharger `matrix_speedrunner-VERSION-windows-x86_64.zip` depuis la page
   [Releases](../../releases).
2. Extraire l'archive (clic droit → *Extraire tout*).
3. Ouvrir **Windows Terminal** (recommandé pour le rendu truecolor).
4. Lancer le `.exe` :

   ```powershell
   .\matrix_speedrunner.exe
   ```

   Ou double-cliquer sur `run.bat` inclus dans l'archive.

Vérifier l'intégrité (PowerShell) :

```powershell
Get-FileHash matrix_speedrunner-VERSION-windows-x86_64.zip -Algorithm SHA256
# Comparer avec le contenu de .sha256
```

> **Astuce** : sur l'ancien `cmd.exe` ou la console PowerShell standard, les
> couleurs RGB sont approximées en 16 couleurs ANSI. Pour le rendu propre,
> utilise [Windows Terminal](https://aka.ms/terminal).

### Build depuis les sources

#### 1. Installer Rust

[Rustup](https://rustup.rs/) (Linux/macOS/Windows) :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sur Windows, utilise plutôt l'installateur graphique depuis le site rustup.

#### 2. Cloner et build

```bash
git clone https://github.com/<owner>/matrix_speedrunner.git
cd matrix_speedrunner
cargo build --release
./target/release/matrix_speedrunner
```

Pour un cycle de dev rapide :

```bash
cargo run
```

## Lancement

```bash
matrix_speedrunner
```

Au démarrage :

1. Choisir un mode dans le menu (`Speed Runner` ou `Hack Time Attack`).
2. Saisir nom + email/téléphone (pour participer au classement).
3. Choisir une difficulté : `Easy` / `Normal` / `Insane`.
4. Jouer.

## Contrôles

### Menu principal
| Touche       | Action            |
| ------------ | ----------------- |
| `↑` / `↓`    | Navigation        |
| `Enter`      | Valider           |
| `Q` / `Esc`  | Quitter           |

### Saisie d'identité
| Touche             | Action                       |
| ------------------ | ---------------------------- |
| `Tab` / `↑` / `↓`  | Champ suivant / précédent    |
| `Backspace`        | Effacer le dernier caractère |
| `Enter`            | Valider                      |
| `Esc`              | Retour menu                  |

### En jeu
| Touche       | Action                          |
| ------------ | ------------------------------- |
| Lettres      | Taper la commande               |
| `Enter`      | Valider la commande tapée       |
| `Backspace`  | Effacer le dernier caractère    |
| `Esc`        | Abandonner et retourner au menu |

### Game Over / Time Out
| Touche          | Action                                            |
| --------------- | ------------------------------------------------- |
| `R` / `Enter`   | Rejouer (même mode, même difficulté, même joueur) |
| `D`             | Changer de difficulté                             |
| `S`             | Voir les scores                                   |
| `M` / `Esc`     | Retour menu                                       |
| `Q`             | Quitter le jeu                                    |

### Écran Scores
| Touche               | Action            |
| -------------------- | ----------------- |
| `↑` / `↓` / `j` / `k`| Scroll            |
| `PgUp` / `PgDn`      | Scroll rapide     |
| `F`                  | Filtrer par mode  |
| `Esc` / `Enter`      | Retour menu       |

## Stockage des scores

Les scores sont écrits dans un répertoire utilisateur standard :

| OS      | Chemin                                              |
| ------- | --------------------------------------------------- |
| Linux   | `~/.local/share/matrix_speedrunner/`                |
| macOS   | `~/Library/Application Support/matrix_speedrunner/` |
| Windows | `%APPDATA%\matrix_speedrunner\`                     |

Deux fichiers maintenus en parallèle :

- **`scores.json`** — source de vérité du jeu (rechargé au démarrage de
  l'écran Scores).
- **`scores.csv`** — export, **UTF-8 avec BOM + CRLF** (RFC 4180), ouvrable
  directement par Excel/LibreOffice/`pandas.read_csv`.

Colonnes du CSV :

```
timestamp,name,contact,mode,difficulty,score,wpm,max_combo,items_done,duration_secs
```

`timestamp` est en RFC 3339 (`2026-05-10T14:32:18+02:00`).

## Configuration

Tout est dans [`assets/config.toml`](assets/config.toml) : durées des parties
**et** pools de commandes pour chaque mode. Le fichier est embarqué dans le
binaire à la compilation, donc le jeu marche out-of-the-box sans aucune
config externe.

Pour **personnaliser sans recompiler**, copie le fichier ici :

| OS      | Chemin                                                        |
| ------- | ------------------------------------------------------------- |
| Linux   | `~/.config/matrix_speedrunner/config.toml`                    |
| macOS   | `~/Library/Application Support/matrix_speedrunner/config.toml` |
| Windows | `%APPDATA%\matrix_speedrunner\config.toml`                    |

Au démarrage, si ce fichier existe et parse correctement, ses valeurs
**remplacent** les défauts embarqués. Toutes les sections et clés sont
optionnelles — tu peux ne mettre que ce que tu veux changer. Si le fichier est
invalide, le jeu retombe silencieusement sur les valeurs par défaut.

Structure complète :

```toml
[speedrunner]
# Durée max d'une partie de Speed Runner (la partie peut aussi finir
# sur 3 vies perdues avant que le timer expire).
time_limit_secs = 180

[hack]
# Durée d'une partie de Hack Time Attack.
time_limit_secs = 120

[commands.speedrunner]
tier1  = ["ls", "pwd", ...]                            # commandes courtes
tier2  = ["git commit", ...]                           # intermédiaires
tier3  = ["kubectl rollout restart deploy api", ...]   # longues
easter = ["wake up neo", ...]                          # rares (~4-5 %)

[commands.hack]
tier1  = [...]
tier2  = [...]
tier3  = [...]
easter = [...]
```

**Cas d'usage typiques** :
- Stand court (15 min de queue) → baisse `speedrunner.time_limit_secs = 90` et `hack.time_limit_secs = 60` pour augmenter le débit.
- Concours long → monte les deux à 300+.
- Désactiver le timer Speed Runner (mode pur "vies") → mets `time_limit_secs = 99999`.

## Construire des paquets de release

Les scripts de packaging sont dans `scripts/` et produisent des archives prêtes
à publier sur la page Releases de GitHub.

### Linux (depuis Linux ou macOS)

```bash
./scripts/build-linux.sh
```

Produit :
- `dist/matrix_speedrunner-<version>-linux-x86_64.tar.gz`
- `dist/matrix_speedrunner-<version>-linux-x86_64.tar.gz.sha256`

### Windows

#### Option A — cross-compile depuis Linux/macOS

Pré-requis :

```bash
# Linux
sudo apt install mingw-w64 zip
# macOS
brew install mingw-w64

rustup target add x86_64-pc-windows-gnu
```

Puis :

```bash
./scripts/build-windows.sh
```

Produit :
- `dist/matrix_speedrunner-<version>-windows-x86_64.zip`
- `dist/matrix_speedrunner-<version>-windows-x86_64.zip.sha256`

#### Option B — build natif sur Windows

Pré-requis :
- Rust (rustup)
- Visual Studio Build Tools 2022 (workload *Outils de génération C++*)

Dans PowerShell :

```powershell
.\scripts\build-windows.ps1
```

Mêmes artefacts dans `dist\`.

### Build des deux en une commande

```bash
./scripts/build-all.sh
```

## Architecture du code

```
src/
├── main.rs         entry point + déclaration des modules
├── app.rs          state machine + event loop (~30 FPS)
├── game.rs         types partagés (Difficulty, GameMode, Session, Summary)
├── speedrunner.rs  state du mode Speed Runner
├── hack.rs         state du mode Hack Time Attack
├── commands.rs     pools de commandes (deux ensembles distincts par mode)
├── rain.rs         effet de pluie Matrix (widget cellule-par-cellule)
├── score.rs        persistance JSON + export CSV (BOM UTF-8 + CRLF)
└── ui.rs           rendu de tous les écrans, dispatch par mode
```

Le couplage entre les deux modes passe par l'enum `Session` dans `game.rs` :
chaque variante encapsule l'état spécifique du mode (`SpeedRunnerState` ou
`HackState`), et expose une API uniforme (`tick`, `type_char`, `submit`,
`is_over`, `summary`). Pas de trait object, pas de `Box<dyn>` — dispatch
statique, optimisé à la compilation.

## Crates utilisées

| Crate          | Version | Rôle                                       |
| -------------- | ------- | ------------------------------------------ |
| `ratatui`      | 0.29    | Framework TUI (immediate mode)             |
| `crossterm`    | 0.28    | Backend terminal cross-plateforme          |
| `color-eyre`   | 0.6     | Gestion d'erreurs avec backtrace           |
| `rand`         | 0.8     | Spawn aléatoire                            |
| `serde`        | 1       | Sérialisation `ScoreEntry`                 |
| `serde_json`   | 1       | Persistance JSON                           |
| `chrono`       | 0.4     | Horodatage local + format RFC 3339         |
| `dirs`         | 5       | Chemin standard du data/config dir par OS  |
| `toml`         | 0.8     | Parsing du fichier de pool de commandes    |

## Licence

[**WTFPL** v2](http://www.wtfpl.net/) — *Do What The Fuck You Want To Public License*.

> 0. You Just DO WHAT THE FUCK YOU WANT TO.

Voir le fichier [`LICENSE`](LICENSE). En clair : forke, modifie, copie-colle dans
ton propre projet, fais payer 100 €, fais tatouer le code source sur ton avant-bras —
aucune obligation, aucune contrainte, aucune mention à conserver. Si tu trouves
un bug, tu peux ouvrir une issue ou simplement le garder pour toi, c'est toi qui
vois. *Free as in fuck off.*

Crédits visuels au film *The Matrix* (Wachowski, 1999) — citations clin d'œil
uniquement, aucun asset graphique du film n'est inclus.
