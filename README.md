# Filename Repair Tool for Linux
# "NameToUnix"

(german and english)

A powerful command line tool for cleaning up file names according to Linux conventions.
It works under Linux. The program is useful if many file names, e.g. after downloading and unpacking zip files from Windows file systems 
contain spaces or special characters. It saves an enormous amount of time by automatically replacing the offending characters. 

I have been using a similar program - a Perl script - for about 15 years. It has saved me many, many hours of mindless renaming work. Now I'm learning Rust and wanted to write a useful command line application.  ***NameToUnix*** is the result. 

This is my first program in Rust. (Please have mercy on me.)

Ein leistungsstarkes Kommandozeilen-Tool zum Bereinigen von Dateinamen gemäß Linux-Konventionen.
Es funktioniert unter Linux. Das Programm ist sinnvoll, wenn viele Dateinamen z. B. nach einem Download und Entpacken von Zip-Dateien aus Windows-Dateisystemen 
Leerzeichen oder Sonderzeichen enthalten. Es erspart enorm viel Zeit durch automatisches Ersetzen der störenden Zeichen. 

Ich benutze ein ähnliches Programm - ein Perl-Skript - seit ca. 15 Jahren. Es hat mir schon viele, viele Stunden stumpfsinniger Umbenennungs-Arbeit erspart. Nun bin ich dabei, Rust zu lernen und wollte eine sinnvolle Kommandozeilenanwendung schreiben.  ***NameToUnix*** ist dabei herausgekommen.

Dies ist mein erstes Programm in Rust. (Bitte seid gnädig.)

(c) 2025 Dieter Schlüter <dieter.schlueter@linix.de>

## Functions / Funktionen

- Replaces spaces and special characters in file and directory names with underscores
- Converts German umlauts to their ASCII counterparts (ä → ae, etc.)
- Supports recursive processing of directories
- Provides preview mode without actual changes
- Allows user-defined replacement rules via configuration file
- Supports exclusion patterns for specific file patterns/directory patterns

---

- Ersetzt Leerzeichen und Sonderzeichen in Datei- und Verzeichnisnamen durch Unterstriche
- Konvertiert deutsche Umlaute in ihre ASCII-Pendants (ä → ae, usw.)
- Unterstützt rekursive Verarbeitung von Verzeichnissen
- Bietet Vorschau-Modus ohne tatsächliche Änderungen
- Ermöglicht benutzerdefinierte Ersetzungsregeln über Konfigurationsdatei
- Unterstützt Ausschlussmuster für bestimmte Datei-Muster/Verzeichnis-Muster

## Installation

```bash
git clone https://github.com/jamulix/NameToUnix.git       # Download repository
cd NameToUnix                                             # Change to download directory
cargo build --release                                     # Build binary
sudo cp target/release/NameToUnix /usr/local/bin/         # copy binary to local bin directory
sudo mkdir -p /etc/NameToUnix/                            # Create global config directory for NameToUnix in /etc
sudo cp .NameToUnix.conf  /etc/NameToUnix/config.toml     # Copy config file to this global directory
mkdir -p ~/.config/NameToUnix/                            # Create a personal config directory for NameToUnix 
cp .NameToUnix.conf ~/.config/NameToUnix/config.toml      # Copy config file to this personal directory
```

Die ausführbare Datei wird dann unter `target/release/NameToUnix` erstellt. Du solltest sie mit 'sudo cp target/release/NameToUnix /usr/local/bin/' kopieren. Sie ist dann für alle User verfügbar. Denke daran, die Konfiguationsdatei (s. u.) ebenfalls zu kopieren. Sie kann für jeden User individuell angepasst werden, wenn sie im home-Verzeichnis des Users liegt.

The executable file is then created under `target/release/NameToUnix`. You should copy it with 'sudo cp target/release/NameToUnix /usr/local/bin/'. It is then available for all users. Remember to copy the configuration file (see below) as well. It can be customized for each user individually if it is located in the user's home directory.

## Usage

```bash
# Basic usage
NameToUnix /path/to/files

# Only preview the changes without actual renaming
NameToUnix -n /path/to/files

# Process multiple paths
NameToUnix /path1 /path2 /path3

# Exclude specific files
NameToUnix -e “*.tmp” -e “backup_*” /path/to/files

# Increase verbosity
NameToUnix -v /path/to/files

# Also rename the root directory
NameToUnix --modify-root /path/to/files

```
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

## Configuration File / Konfiguration

Erstelle eine Datei `.NameToUnix.conf` im persönlichen Arbeitsverzeichnis z. B. mit folgendem Inhalt 
(alternativ `~/.config/NameToUnix/config.toml`):

Create a file `.NameToUnix.conf` in your personal working directory, e.g. with the following content 
(alternatively `~/.config/NameToUnix/config.toml`):


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
Dies ist eine Beispielkonfiguration. Du kannst Die Datei nach Belieben anpassen.
The above is an example configuration. You can customize the file as you wish.

Erstelle eine Datei /etc/NameToUnix/config.toml im globalen Verzeichnis /etc z. B. mit folgendem Inhalt:

```toml
# /etc/NameToUnix/config.toml
# --------------------------------------------
# In dieser Datei können beliebige zusätzliche Schlüssel-Werte-Paare unter [replacements] hinterlegt werden,
# die im Dateistammnamen ersetzt werden. Zum Beispiel:
#
# [replacements]
# "foo" = "bar"
# "old" = "neu"
#
# Dadurch werden in den Dateinamen alle "foo" durch "bar" ersetzt, und "old" durch "neu".
# WICHTIG: Die hartcodierten Transformationen sind aber immer vorrangig und lassen sich auch nicht rückgängig machen.
# Die persönlichen Einstellungen überschreiben diese Einstellungen.  (Be careful! First test with NameToUnix -n <path>)

[replacements]
".." = "."
"_·_" = "_-_"
".-_" = "_-_"
# "" = ""          # bewirkt nichts / Dummy entry
```


### Verwendung von NameToUnix

Um die Verwendung von `NameToUnix` zu verstehen, kannst du die folgende Hilfe ausgeben:

```text
NameToUnix --help
```

Die Ausgabe sieht wie folgt aus:

```text
Ein Tool zum Anpassen von Verzeichnis- und Dateinamen an Linux-Konventionen

Usage: NameToUnix [OPTIONS] [PATHS]...

Arguments:
[PATHS]... Pfade (Dateien und Verzeichnisse) zum rekursiven Anpassen

Options:
-q, --quiet Ausgaben unterdrücken (keine Umbenennungsinfos auf stdout)
-n, --no-changes Nur anzeigen, aber keine realen Änderungen vornehmen
-f, --force Existierende Dateien überschreiben
-e, --exclude <PATTERN> Zu ignorierende Muster (-e "*.py", mehrere können angegeben werden)
-v, --verbose Ausführliche Debug-Informationen
    --modify-root Erlaubt, auch das Wurzelverzeichnis anzupassen
-h, --help Print help
-V, --version Print version

```

### Usage of NameToUnix

```text
A tool for adapting directories and file names to Linux conventions

Usage: NameToUnix [OPTIONS] [PATHS]...

Arguments:
[PATHS]... Paths (files and directories) for recursive customization

Options:
-q, --quiet Suppress output (no renaming info on stdout)
-n, --no-changes Only display, but do not make any real changes
-f, --force Overwrite existing files
-e, --exclude <PATTERN> Patterns to be ignored (-e “*.py”, several can be specified)
-v, --verbose Detailed debug information
    --modify-root Allows you to customize the root directory as well
-h, --help Print help
-V, --version Print version

```

## Test

Im Verzeichnis [***./test***](./test) gibt es ein bash-Skript [***create_test_tree.sh***](test/create_test_tree.sh), das lokal 21 Test-Verzeichnisse und 400 Dateien mit skurrilen Zufallsnamen erzeugt. Damit kannst Du ***NameToUnix*** ausprobieren:
 
***NameToUnix -n ./testverzeichnis*** (nur Anzeige der Änderungen)
oder
***NameToUnix ./testverzeichnis*** (Anzeige mit Umbenennen).


In the directory [***./test***](./test) there is a bash script [***create_test_tree.sh***](test/create_test_tree.sh), which locally creates 21 test directories and 400 files with bizarre random names. You can use this to try out ***NameToUnix***:
 
***NameToUnix -n ./testverzeichnis*** (display changes only)
or
***NameToUnix ./testverzeichnis*** (display with renaming).

## Lizenz / License

This project is licensed under the MIT license - see the [LICENSE](LICENSE) file for details.

Dieses Projekt steht unter der MIT-Lizenz - siehe die [LICENSE](LICENSE)-Datei für Details.

## Mitwirken / Contributions

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on the pull request process.

Beiträge sind willkommen! Bitte lies [CONTRIBUTING.md](CONTRIBUTING.md) für Details zum Prozess für Pull Requests.

