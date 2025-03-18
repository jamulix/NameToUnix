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
