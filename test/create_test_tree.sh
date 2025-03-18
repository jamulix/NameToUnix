#!/bin/bash
# create_test_tree.sh

# Erzeugt ein Verzeichnis mit dem Namen $TOPDIR und 20 Unterverzeichnissen 
# deren Namen aus zufälligen gewöhnichen und ungewöhnlichen Zeichen (s. chars) bestehen.
# Diese enthalten Dateien mit Basisnamen aus zufälligen gewöhnichen und ungewöhnlichen Zeichen
# und konventionellen Endungen (s. extensions). Der Inhalt ist ebenfalls zufällig.
#
# Dieser Dateibaum dient dann zum Testen des Rust-Programms NameToUnix :
# z. B.
# NameToUnix -n $TOPDIR  (Option -n ==> Änderungen nur anzeigen)
# oder
# NameToUnix $TOPDIR
#
#
# (c) 2025 Dieter Schlüter <dieter.schlueter@linix.de>

TOPDIR="./testverzeichnis"

# 1. Altes Testverzeichnis löschen
rm -rf "$TOPDIR"

# 2. Neues Verzeichnis erstellen
mkdir -p "$TOPDIR"
cd "$TOPDIR" || exit


# Liste gewöhnicher und ungewöhnlicher Zeichen und Emojis

chars=(
  "a" "b" "c" "d" "e" "f" "g" "h" "i" "j" "k" "l" "m"
  "n" "o" "p" "q" "r" "s" "t" "u" "v" "w" "x" "y" "z"
  "A" "B" "C" "D" "E" "F" "G" "H" "I" "J" "K" "L" "M"
  "N" "O" "P" "Q" "R" "S" "T" "U" "V" "W" "X" "Y" "Z"
  "0" "1" "2" "3" "4" "5" "6" "7" "8" "9"
  " " "  " "!" "@" "#" "$" "%" "^" "&" "*" "(" ")"
  "=" "+" "[" "]" "{" "}" "|" "\\" ";" ":" "'" "\""
  "," "<" ">" "?" "~" "\`" "¡" "™" "£" "¢" "∞" "§" "¶"
  "•" "ª" "º" "≠" "œ" "∑" "´" "®" "†" "¨" "ˆ" "ø" "π"
  "“" "‘" "«" "…" "å" "ß" "∂" "ƒ" "©" "˙" "∆" "˚" "¬"
  "Ω" "≈" "ç" "√" "∫" "˜" "µ" "≤" "≥" "÷" 
  "🦄" "👾" "💩" "🚀" "🎉" "😱" "🤯" "🌌" "💥" "🔧" "📁"
)

# Liste üblicher Dateiendungen
extensions=("txt" "jpg" "png" "pdf" "docx" "xlsx" "pptx" 
            "zip" "tar.gz" "sh" "py" "js" "html" "css" 
            "php" "json" "xml" "csv" "log" "conf")

# Zufälliger Datei/Verzeichnis Name Generator
generate_weird_name() {
    local length=$(( RANDOM % 15 + 5 ))
    for ((i=0; i<length; i++)); do
        local char_idx=$(( RANDOM % ${#chars[@]} ))
        echo -n "${chars[$char_idx]}"
    done
}

# 20 bizarre Verzeichnisse erstellen
for d in {1..20}; do
# for d in {1..2}; do
    dir_name="$(generate_weird_name)"
    mkdir -p "$dir_name"
    
    # 20 Dateien pro Verzeichnis
    for f in {1..20}; do
    # for f in {1..5}; do
        file_base="$(generate_weird_name)"
        ext_idx=$(( RANDOM % ${#extensions[@]} ))
        file_name="${file_base}.${extensions[$ext_idx]}"
        
        # Datei mit zufälligem Inhalt erstellen
        head -c $(( RANDOM % 1024 )) /dev/urandom > "$dir_name/$file_name" 2>/dev/null
    done
done

# Beispielausgabe
echo "Testumgebung erstellt:"
tree -L 2 | head -n 20
exit 0

