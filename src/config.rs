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
