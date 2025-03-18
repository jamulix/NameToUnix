# Dateinamen-Reparatur-Tool "NameToUnix"

Ein leistungsstarkes Kommandozeilen-Tool zum Bereinigen von Dateinamen gemäß Linux-Konventionen.
Es funktioniert unter Linux (und sollte auch unter MacOS laufen, dort habe ich es aber nicht getestet). 
Das Programm ist sinnvoll, wenn viele Dateinamen z. B. nach einem Download und Entpacken von Zip-Dateien aus Windows-Dateisystemen 
Leerzeichen oder Sonderzeichen enthalten. Es erspart enorm viel Zeit durch automatisches Ersetzen der störenden Zeichen. 

(c) 2025 Dieter Schlüter <dieter.schlueter@linix.de>

## Funktionen

- Ersetzt Leerzeichen und Sonderzeichen in Datei- und Verzeichnisnamen durch Unterstriche
- Konvertiert deutsche Umlaute in ihre ASCII-Pendants (ä → ae, usw.)
- Unterstützt rekursive Verarbeitung von Verzeichnissen
- Bietet Vorschau-Modus ohne tatsächliche Änderungen
- Ermöglicht benutzerdefinierte Ersetzungsregeln über Konfigurationsdatei
- Unterstützt Ausschlussmuster für bestimmte Datei-Muster/Verzeichnis-Muster

## Installation

### Über Cargo

```bash
cargo install NameToUnix
```

### Manueller Build

```bash
git clone https://github.com/jamulix/NameToUnix.git
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
(alternativ: ~/.config/NameToUnix/config.toml)

```toml
[replacements]
"foo" = "bar"
"old" = "new"
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

## Test

Im Verzeichnis [***./test***](./test) gibt es ein bash-Skript [***create_test_tree.sh***](test/create_test_tree.sh), das lokal 21 Test-Verzeichnisse und 400 Dateien mit skurrilen Zufallsnamen erzeugt. Damit kannst Du ***NameToUnix*** ausprobieren:
 
***NameToUnix -n ./testverzeichnis*** (nur Anzeige der Änderungen)
oder
***NameToUnix ./testverzeichnis*** (Anzeige mit Umbenennen).

## Lizenz

Dieses Projekt steht unter der MIT-Lizenz - siehe die [LICENSE](LICENSE)-Datei für Details.

## Mitwirken

Beiträge sind willkommen! Bitte lies [CONTRIBUTING.md](CONTRIBUTING.md) für Details zum Prozess für Pull Requests.

