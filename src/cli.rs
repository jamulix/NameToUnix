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
