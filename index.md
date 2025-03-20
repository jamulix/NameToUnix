# Willkommen bei meinem Projekt
NameToUnix - Rust Command line tool for cleaning up directory & file names according to Linux conventions. Recursively replacing offending characters or spaces.

Dieses Projekt bietet unter Linux eine einfache Lösung für das automatische Umbenennen von Verzeichnissen und Dateien nach dem Entpacken von gezippten Windows-Dateien mit Leerzeichen oder Sonderzeichen im Namen.

## Zielsetzung
Das Ziel ist es, diese unkonventionellen Dateinamen rekursiv und automatisch sinnvoll umzubenennen.

## Inhaltsverzeichnis

```text
NameToUnix/
├── .github/                 # CI/CD und GitHub-spezifische Dateien
│   └── workflows/
│       └── build.yaml       # GitHub Actions Workflow für Build and Release
├── test/                    # enthält test-Verzeichnis Skript
│   └── create_test_tree.sh  # Bash-Skript erzeugt ein skurriles Testverzeichnis
├── src/                     # Quellcode-Verzeichnis
│   ├── main.rs              # Haupteinstiegspunkt (bereits vorhanden)
│   ├── cli.rs               # CLI-Argumente und Parsing
│   ├── config.rs            # Konfigurationsverwaltung
│   └── sanitizer.rs         # Kernlogik zur Dateinamenbereinigung
├── .NameToUnix.conf         # Konfigurationsdatei (Übersetzungsregeln: 'foo' = 'bar')
├── CONTRIBUTING.md          # Contribute-Dokumentation
├── Cargo.toml               # Projektmetadaten und Abhängigkeiten
├── LICENSE                  # Lizenzinformationen
├── README.md                # Projektdokumentation
├── index.md                 # diese Datei (german)
└── release.md               # Infos über dieses Release (german)
```

## Hinweise zur Nutzung
Um dieses Projekt zu nutzen, folge bitte den Anweisungen in der [README -Datei](README.md).

