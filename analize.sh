#!/bin/bash

# Check that a file is given as argument
if [ -z "$1" ]; then
    echo "Usage: $0 file.o"
    exit 1
fi

FILE="$1"

# Check that the file exists
if [ ! -f "$FILE" ]; then
    echo "Error: file '$FILE' not found"
    exit 1
fi

# echo "=============================="
# echo "📦 General info"
# echo "=============================="
# file "$FILE"

echo -e "\n=============================="
echo "🔣 Symbols (nm)"
echo "=============================="
nm "$FILE"

echo -e "\n=============================="
echo "🧠 Disassembly (objdump)"
echo "=============================="
objdump -d "$FILE"

# echo -e "\n=============================="
# echo "📚 Sections (readelf)"
# echo "=============================="
# readelf -S "$FILE"

# echo -e "\n=============================="
# echo "🔍 Headers ELF"
# echo "=============================="
# readelf -h "$FILE"
