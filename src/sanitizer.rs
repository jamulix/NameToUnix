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
