## Release 0.1.0   2025/03/18

Lass mich die wichtigsten Dateien genauer beschreiben:

## 1. README.md


# NameToUnix

Ein leistungsstarkes Kommandozeilen-Tool zum Bereinigen von Dateinamen gemäß Linux-Konventionen. Es ist sinnvoll, 
wenn viele Dateinamen z. B. nach einem Download und Entpacken von Zip-Dateien aus Windows-Dateisystemen 
Leerzeichen enthalten. Es erspart enorm viel Zeit durch automatisches Umbenennen, d.h. Ersetzen der störenden Zeichen. 

## Funktionen

- Ersetzt Leerzeichen und Sonderzeichen durch Unterstriche
- Konvertiert deutsche Umlaute in ihre ASCII-Pendants (ä → ae, usw.)
- Unterstützt rekursive Verarbeitung von Verzeichnissen
- Bietet Vorschau-Modus ohne tatsächliche Änderungen
- Ermöglicht benutzerdefinierte Ersetzungsregeln über Konfigurationsdatei
- Unterstützt Ausschlussmuster für bestimmte Dateien/Verzeichnisse

## Installation

### Über Cargo

```bash
cargo install NameToUnix
```

### Manueller Build

```bash
git clone https://github.com/username/NameToUnix.git
cd NameToUnix
cargo build --release
```

Die ausführbare Datei wird dann unter `target/release/NameToUnix` erstellt.

## Verwendung

```bash
# Grundlegende Verwendung
NameToUnix /pfad/zu/dateien

# Nur Vorschau der Änderungen ohne tatsächliche Umbenennung
NameToUnix -n /pfad/zu/dateien

# Mehrere Pfade verarbeiten
NameToUnix /pfad1 /pfad2 /pfad3

# Bestimmte Dateien ausschließen
NameToUnix -e "*.tmp" -e "backup_*" /pfad/zu/dateien

# Verbosity erhöhen
NameToUnix -v /pfad/zu/dateien

# Auch das Wurzelverzeichnis umbenennen
NameToUnix --modify-root /pfad/zu/dateien
```

## Konfiguration

Erstelle eine Datei `.NameToUnix.conf` im Arbeitsverzeichnis mit folgendem Inhalt:

```toml
[replacements]
"foo" = "bar"
"alt" = "neu"
".." = "."
"_·_" = "_-_"
"Ä" = "Ae"
"Ö" = "Oe"
"Ü" = "Ue"
"ä" = "ae"
"ö" = "oe"
"ü" = "ue"
"ß" = "ss"
```

## Lizenz

Dieses Projekt steht unter der MIT-Lizenz - siehe die [LICENSE](LICENSE)-Datei für Details.

## Mitwirken

Beiträge sind willkommen! Bitte lies [CONTRIBUTING.md](CONTRIBUTING.md) für Details zum Prozess für Pull Requests.


## 2. .gitignore

```text
/target
**/*.rs.bk
Cargo.lock
.idea/
.vscode/
*.swp
*.swo
.DS_Store
.NameToUnix.conf
/test-files/

```

## 3. LICENSE

```text
MIT License

Copyright (c) 2025 Dieter Schlüter

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

```

## 4. src/cli.rs

```rust
use clap::{ArgGroup, Parser};
use std::path::PathBuf;

/// Command-line interface für das Umbenennungsprogramm
#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
#[clap(group(
    ArgGroup::new("mode")
        .args(&["no_changes", "force"])
        .multiple(false)
))]
pub struct Cli {
    /// Pfade (Dateien und Verzeichnisse) zum rekursiven Anpassen
    pub paths: Vec<PathBuf>,

    /// Ausgaben unterdrücken (keine Umbenennungsinfos auf stdout)
    #[clap(short, long)]
    pub quiet: bool,

    /// Nur anzeigen, aber keine realen Änderungen vornehmen
    #[clap(short, long)]
    pub no_changes: bool,

    /// Existierende Dateien überschreiben
    #[clap(short, long)]
    pub force: bool,

    /// Zu ignorierende Muster (-e "*.py", mehrere können angegeben werden)
    #[clap(short = 'e', long, value_name = "PATTERN")]
    pub exclude: Vec<String>,

    /// Ausführliche Debug-Informationen
    #[clap(short = 'v', long)]
    pub verbose: bool,

    /// Erlaubt, auch das Wurzelverzeichnis anzupassen
    #[clap(long)]
    pub modify_root: bool,
}
```

## 5. src/config.rs

```rust
use anyhow::{Context, Result};
use log::{debug, info};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Repräsentiert die Konfiguration, die sich aus einer TOML-Datei laden lässt.
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Config {
    /// Individuelle Ersetzungen, z. B. "foo" => "bar"
    pub replacements: HashMap<String, String>,
}

impl Config {
    /// Liest eine TOML-Konfigurationsdatei ein, wenn sie existiert.
    fn load_internal(path: &Path, verbose: bool) -> Result<Self> {
        if path.exists() {
            if verbose {
                debug!("Lade Konfigurationsdatei: {}", path.display());
            }
            let content = fs::read_to_string(path).with_context(|| {
                format!("Konnte Konfigurationsdatei nicht lesen: {}", path.display())
            })?;
            let loaded: Self = toml::from_str(&content).with_context(|| {
                format!(
                    "Konnte Konfigurationsdatei nicht parsen: {}",
                    path.display()
                )
            })?;
            if verbose && !loaded.replacements.is_empty() {
                debug!("Eingelesene Replacements:");
                for (k, v) in &loaded.replacements {
                    debug!("  {:?} => {:?}", k, v);
                }
            }
            Ok(loaded)
        } else {
            Err(anyhow::anyhow!("Datei existiert nicht: {}", path.display()))
        }
    }

    /// Öffentliche Methode zum Laden einer Konfiguration aus einem Pfad
    pub fn load(path: &str, verbose: bool) -> Result<Self> {
        let cfg_path = Path::new(path);
        match Self::load_internal(cfg_path, verbose) {
            Ok(config) => Ok(config),
            Err(_) => {
                if verbose {
                    info!(
                        "Keine Konfigurationsdatei '{}' gefunden. Verwende Standardwerte.",
                        path
                    );
                }
                Ok(Self::default())
            }
        }
    }
    /// Sucht nach Konfigurationsdateien in verschiedenen Orten und kombiniert sie
    pub fn from_default_locations(verbose: bool) -> Result<Self> {
        // Prioritätenreihenfolge (später überschreibt früher):
        // 1. Standard etc-Verzeichnis (/etc/NameToUnix/config.toml)
        // 2. Benutzerverzeichnis (~/.config/NameToUnix/config.toml)
        // 3. Arbeitsverzeichnis (./.NameToUnix.conf)
        // Die Ladereihenfolge ist so gewählt, dass lokale Einstellungen Vorrang vor
        // Benutzereinstellungen haben und diese wiederum Vorrang vor Systemeinstellungen.

        // Wir beginnen mit einer leeren Konfiguration
        let mut config = Self::default();
        let mut config_found = false;

        // Standard etc-Verzeichnis
        let etc_config = Path::new("/etc/NameToUnix/config.toml");
        if etc_config.exists() {
            if verbose {
                debug!("Lade System-Konfiguration: {}", etc_config.display());
            }
            if let Ok(etc_conf) = Self::load(etc_config.to_str().unwrap_or_default(), verbose) {
                // Füge die Werte zur Konfiguration hinzu
                config.replacements.extend(etc_conf.replacements);
                config_found = true;
            }
        } else if verbose {
            debug!(
                "System-Konfiguration nicht gefunden: {}",
                etc_config.display()
            );
        }

        // Benutzerverzeichnis
        if let Some(home) = dirs::home_dir() {
            let user_config = home.join(".config/NameToUnix/config.toml");
            if user_config.exists() {
                if verbose {
                    debug!("Lade Benutzer-Konfiguration: {}", user_config.display());
                }
                if let Ok(user_conf) = Self::load(user_config.to_str().unwrap_or_default(), verbose)
                {
                    // Überschreibe/ergänze mit Benutzerkonfiguration
                    config.replacements.extend(user_conf.replacements);
                    config_found = true;
                }
            } else if verbose {
                debug!(
                    "Benutzer-Konfiguration nicht gefunden: {}",
                    user_config.display()
                );
            }
        }

        // Arbeitsverzeichnis (höchste Priorität)
        let local_config = Path::new(".NameToUnix.conf");
        if local_config.exists() {
            if verbose {
                debug!("Lade lokale Konfiguration: {}", local_config.display());
            }
            if let Ok(local_conf) = Self::load(local_config.to_str().unwrap_or_default(), verbose) {
                // Überschreibe/ergänze mit lokaler Konfiguration
                config.replacements.extend(local_conf.replacements);
                config_found = true;
            }
        } else if verbose {
            debug!(
                "Lokale Konfiguration nicht gefunden: {}",
                local_config.display()
            );
        }

        // Gib die kombinierte Konfiguration zurück
        if config_found {
            if verbose && !config.replacements.is_empty() {
                info!(
                    "Kombinierte Konfiguration enthält {} Ersetzungen",
                    config.replacements.len()
                );
            }
        } else {
            if verbose {
                info!("Keine Konfigurationsdateien gefunden. Verwende nur die Standardwerte.");
            }
        }

        Ok(config)
    }
}
```

## 6. src/sanitizer.rs

```rust
use crate::config::Config;
use emojis;
use glob::Pattern;
use log::{debug, warn};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::ffi::OsStr;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;
use walkdir::DirEntry;

// Regex-Patterns als statische Variablen für bessere Performance
static RE_INVALID: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\w.\-]").unwrap());
static RE_ADJACENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"_\.|\._").unwrap());
static RE_MULTI: Lazy<Regex> = Lazy::new(|| Regex::new(r"[_\.]{2,}").unwrap());

/// Bereinigt den übergebenen Dateinamen oder Verzeichnisnamen.
pub fn clean_filename(name: &OsStr, config: &Config, verbose: bool) -> Option<String> {
    let original = name.to_string_lossy();

    // Stamm und Extension trennen
    let (mut base, mut ext) = match original.rsplit_once('.') {
        Some((b, e)) => (b.to_string(), format!(".{e}")),
        None => (original.to_string(), String::new()),
    };

    // Platzhalter (C++, c++, C#, c#) anlegen
    base = preserve_special_identifiers(&base);
    ext = preserve_special_identifiers(&ext);

    // 1) Konfig-Replacements zuerst
    for (k, v) in &config.replacements {
        base = base.replace(k, v);
    }

    // 2) Dann erst hart-codierte Ersetzungen anwenden
    base = apply_hardcoded_replacements(&base);

    // 3) Emojis und hochgestellte Zeichen ersetzen
    base = replace_emojis_and_superscript(&base);

    // 4) Entfernen/Ersetzen aller übrigen ungültigen Zeichen
    base = RE_INVALID.replace_all(&base, "_").to_string();

    // Ungültige Kombinationen aus Punkt und Unterstrich
    base = RE_ADJACENT.replace_all(&base, ".").to_string();

    // Mehrfache Punkte/Unterstriche auf einen reduzieren
    base = RE_MULTI
        .replace_all(
            &base,
            |caps: &Captures| {
                if caps[0].contains('.') {
                    "."
                } else {
                    "_"
                }
            },
        )
        .to_string();

    // Führender Punkt soll bleiben, führende Unterstriche sollen verschwinden
    base = trim_leading_underscores_preserve_leading_dot(&base);

    // Überflüssige Unterstriche und Punkte am Ende beseitigen
    base = base.trim_end_matches('_').to_string();
    base = base.trim_end_matches('.').to_string();

    // Falls komplett geleert, "unnamed"
    if base.is_empty() {
        base = "unnamed".to_string();
    }

    // Endgültigen Dateinamen zusammenbauen
    let mut result = format!("{}{}", base, ext);

    // Platzhalter zurückverwandeln
    result = restore_special_identifiers(&result);

    // Falls --verbose und sich der Name geändert hat
    if verbose && result != original {
        debug!("Transformiert: '{}' -> '{}'", original, result);
    }

    // Keine Änderung -> None zurückgeben
    if result == *original {
        None
    } else {
        Some(result)
    }
}

/// Schützt spezielle Identifikatoren vor der Umwandlung
fn preserve_special_identifiers(input: &str) -> String {
    input
        .replace("C++", "CPLUSPLUS")
        .replace("c++", "cplusplus")
        .replace("C#", "CSHARP")
        .replace("c#", "csharp")
}

/// Stellt spezielle Identifikatoren wieder her
fn restore_special_identifiers(input: &str) -> String {
    input
        .replace("CPLUSPLUS", "C++")
        .replace("cplusplus", "c++")
        .replace("CSHARP", "C#")
        .replace("csharp", "c#")
}

/// Fasst alle fest eingebauten Ersetzungen zusammen.
fn apply_hardcoded_replacements(input: &str) -> String {
    input
        .replace('\'', "") // Apostroph entfernen
        .replace("ˆ", "_")
}

/// Entfernt am Anfang nur Unterstriche, einen führenden Punkt (.) bewahrt es.
fn trim_leading_underscores_preserve_leading_dot(s: &str) -> String {
    let mut chars = s.chars().peekable();
    let mut result = String::new();

    if let Some('.') = chars.peek() {
        // Nimm den Punkt
        result.push('.');
        chars.next();

        // Entferne anschließend führende Unterstriche hinter dem Punkt
        while let Some('_') = chars.peek() {
            chars.next();
        }
    } else {
        // Entferne führende Unterstriche
        while let Some('_') = chars.peek() {
            chars.next();
        }
    }

    // Restliche Zeichen anfügen
    result.extend(chars);
    result
}

/// Ersetzt Emojis und hochgestellte Zeichen (z. B. ²³⁴) durch '_'.
fn replace_emojis_and_superscript(input: &str) -> String {
    input
        .graphemes(true)
        .map(|g| {
            if emojis::get(g).is_some() {
                "_".to_string()
            } else if is_superscript(g) {
                "_".to_string()
            } else {
                g.to_string()
            }
        })
        .collect()
}

/// Prüft, ob alle Zeichen ein Superscript sind (z. B. ²³⁴).
fn is_superscript(g: &str) -> bool {
    g.chars().all(|c| {
        c == '\u{00AA}'
            || c == '\u{00BA}'
            || (c >= '\u{00B2}' && c <= '\u{00B3}')
            || c == '\u{00B9}'
            || (c >= '\u{2070}' && c <= '\u{209F}')
    })
}

/// Prüft, ob der Pfad aufgrund der Ausschlussmuster ignoriert werden soll.
pub fn is_excluded(entry: &DirEntry, patterns: &[Pattern]) -> bool {
    let path = entry.path();
    patterns.iter().any(|p| p.matches_path(path))
}

/// Überprüft, ob eine Umbenennungsoperation sicher ist
pub fn is_safe_rename(src: &Path, dst: &Path, force: bool) -> bool {
    if src.exists() && dst.exists() && !force {
        warn!(
            "Ziel existiert bereits und --force nicht gesetzt: {}",
            dst.display()
        );
        return false;
    }

    // Prüfen auf zusätzliche Sicherheitsrisiken
    // z.B. Systemdateien, schreibgeschützte Verzeichnisse, etc.

    true
}
```

## 7. src/main.rs

```rust
// Verwende nun die Module
mod cli;
mod config;
mod sanitizer;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use config::Config;
use glob::Pattern;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info};
use sanitizer::{clean_filename, is_excluded, is_safe_rename};
use std::fs;
use walkdir::WalkDir;

/// Startpunkt des Programms
fn main() -> Result<()> {
    // Initialisiere Logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Argumente parsen
    let args = Cli::parse();

    // Optional Konfigurationsdatei laden
    let config = Config::from_default_locations(args.verbose)?;
    // let config = Config::load(".NameToUnix.conf", args.verbose)?;

    // Ausschlussmuster (Glob-Patterns) vorbereiten
    let exclude_patterns = args
        .exclude
        .iter()
        .map(|pattern| {
            Pattern::new(pattern)
                .with_context(|| format!("Ungültiges Ausschlussmuster: {}", pattern))
        })
        .collect::<Result<Vec<_>>>()?;

    if args.verbose && !exclude_patterns.is_empty() {
        debug!("Folgende Exclude-Pattern werden genutzt:");
        for p in &exclude_patterns {
            debug!("  {}", p.as_str());
        }
    }

    // Für alle angegebenen Pfade
    for path in &args.paths {
        // Alle Einträge sammeln, damit zuerst die tiefsten umbenannt werden
        let mut entries = Vec::new();
        for entry_result in WalkDir::new(path)
            .into_iter()
            .filter_entry(|e| !is_excluded(e, &exclude_patterns))
        {
            if let Ok(entry) = entry_result {
                entries.push(entry);
            } else if let Err(e) = entry_result {
                error!("Fehler beim Durchlaufen von {}: {}", path.display(), e);
            }
        }

        // Aufsteigend nach Tiefe sortieren, dann umkehren => tiefste Einträge zuerst
        entries.sort_by_key(|e| e.depth());
        entries.reverse();

        // Fortschrittsbalken bei größeren Dateimengen
        let progress_bar = if !args.quiet && entries.len() > 50 {
            let bar = ProgressBar::new(entries.len() as u64);
            bar.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
                .progress_chars("#>-"));
            Some(bar)
        } else {
            None
        };

        // Umbenennen in absteigender Tiefe
        for entry in entries {
            if let Some(bar) = &progress_bar {
                bar.inc(1);
            }

            let old_path = entry.path();

            // Ebenentiefe 0 -> überspringen wir als Verzeichnis, außer --modify_root ist gesetzt
            if entry.depth() == 0 {
                if entry.file_type().is_dir() && !args.modify_root {
                    if args.verbose {
                        debug!("Skip root directory: {}", old_path.display());
                    }
                    continue;
                }
            }

            // Dateiname (oder Verzeichnisname) ermitteln
            let filename = old_path.file_name().ok_or_else(|| {
                anyhow::anyhow!(
                    "Konnte Dateinamen nicht ermitteln für: {}",
                    old_path.display()
                )
            })?;

            // Verarbeiten und ggf. umbenennen
            if let Some(new_name) = clean_filename(filename, &config, args.verbose) {
                let new_path = old_path.with_file_name(&new_name);

                if !args.quiet {
                    info!("{} -> {}", old_path.display(), new_path.display());
                }

                if !args.no_changes {
                    if is_safe_rename(old_path, &new_path, args.force) {
                        fs::rename(old_path, &new_path).with_context(|| {
                            format!(
                                "Fehler beim Umbenennen: {} -> {}",
                                old_path.display(),
                                new_path.display()
                            )
                        })?;
                    }
                }
            }
        }

        if let Some(bar) = &progress_bar {
            bar.finish_with_message("Umbenennung abgeschlossen");
        }
    }

    Ok(())
}
```


## 8. ./github/workflows/build.yaml -- GitHub Actions Workflow für CI/CD

```yaml
name: Build and Test

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable]

    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
        restore-keys: ${{ runner.os }}-cargo-
    
    - name: Format check
      uses: actions-rs/cargo@v1
      with:
        command: fmt
        args: --all -- --check
    
    - name: Clippy check
      uses: actions-rs/cargo@v1
      with:
        command: clippy
        args: -- -D warnings
    
    - name: Build
      uses: actions-rs/cargo@v1
      with:
        command: build
        args: --verbose
    
    - name: Run tests
      uses: actions-rs/cargo@v1
      with:
        command: test
        args: --verbose
```

## Zusammenfassung

Diese Dateien bilden zusammen ein vollständiges, gut strukturiertes Rust-Projekt für den `NameToUnix`. 
Die wichtigsten Aspekte sind:

1. **Modularisierung**: Der Code ist in logische Module aufgeteilt (cli.rs, config.rs, sanitizer.rs)
2. **Verbesserte Fehlerbehandlung**: Verwendung von `anyhow` für bessere Fehlermeldungen
3. **Dokumentation**: Ausführliche README.md mit Anwendungsbeispielen
4. **Tests**: Test-Skript zur Überprüfung der Funktionalität
5. **CI/CD**: Aktionen für automatisierte Tests und Builds
6. **Konfiguration**: Erweiterte Konfigurationsmöglichkeiten
7. **Benutzerfreundlichkeit**: Fortschrittsbalken für große Dateimengen

Diese Struktur folgt den Rust-Best-Practices und macht das Projekt wartbar, erweiterbar und benutzerfreundlich.

