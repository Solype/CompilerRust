#!/bin/bash

# Vérifie qu’un fichier est passé en argument
if [ -z "$1" ]; then
    echo "Usage: $0 fichier.o"
    exit 1
fi

FILE="$1"

# Vérifie que le fichier existe
if [ ! -f "$FILE" ]; then
    echo "Erreur: fichier '$FILE' introuvable"
    exit 1
fi

echo "=============================="
echo "📦 Infos générales"
echo "=============================="
file "$FILE"

echo -e "\n=============================="
echo "🔣 Symboles (nm)"
echo "=============================="
nm "$FILE"

echo -e "\n=============================="
echo "🧠 Désassemblage (objdump)"
echo "=============================="
objdump -d "$FILE"

echo -e "\n=============================="
echo "📚 Sections (readelf)"
echo "=============================="
readelf -S "$FILE"

echo -e "\n=============================="
echo "🔍 Headers ELF"
echo "=============================="
readelf -h "$FILE"
