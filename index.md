# Willkommen bei meinem Projekt
NameToUnix - Rust Command line tool for cleaning up directory & file names according to Linux conventions. Recursively replacing offending characters or spaces.

Dieses Projekt bietet unter Linux eine einfache Lösung für das automatische Umbenennen von Verzeichnissen und Dateien nach dem Entpacken von gezippten Windows-Dateien mit Leerzeichen oder Sonderzeichen im Namen.

## Zielsetzung
Das Ziel ist es, diese unkonventionellen Dateinamen rekursiv und automatisch sinnvoll umzubenennen.

## Inhaltsverzeichnis

```text
NameToUnix/
├── Cargo.toml               # Projektmetadaten und Abhängigkeiten
├── Cargo.lock               # Exakte Versionen der Abhängigkeiten (automatisch generiert)
├── .gitignore               # Dateien, die von Git ignoriert werden sollen
├── .NameToUnix.conf         # Konfigurationsdatei (Übersetzungsregeln: 'foo' = 'bar')
├── README.md                # Projektdokumentation
├── CONTRIBUTING.md          # Contribute-Dokumentation
├── .github/                 # CI/CD und GitHub-spezifische Dateien
│   └── workflows/
│       └── build.yaml       # GitHub Actions Workflow für Build and Release
├── src/                     # Quellcode-Verzeichnis
│   ├── main.rs              # Haupteinstiegspunkt (bereits vorhanden)
│   ├── cli.rs               # CLI-Argumente und Parsing
│   ├── config.rs            # Konfigurationsverwaltung
│   └── sanitizer.rs         # Kernlogik zur Dateinamenbereinigung
├── index.md                 # diese Datei (german)
├── release.md               # Infos über dieses Release (german)
└── LICENSE                  # Lizenzinformationen
```

## Hinweise zur Nutzung
Um dieses Projekt zu nutzen, folge bitte den Anweisungen in der [README -Datei](README.md).

