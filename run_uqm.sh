#!/bin/bash

# Script para ejecutar The Ur-Quan Masters
# Compilado exitosamente en Ubuntu WSL2

echo "=== The Ur-Quan Masters - Execution Script ==="
echo "Proyecto compilado exitosamente con:"
echo "- SDL2, PNG, Ogg Vorbis, zlib"
echo "- OpenGL renderer"
echo "- Audio PulseAudio"
echo ""

cd "$(dirname "$0")/sc2"

echo "Iniciando UQM desde: $(pwd)"
echo "Ejecutando: ./uqm-debug -w -r 800x600 --safe"
echo ""

# Ejecutar el juego en modo ventana con resolución 800x600 y modo seguro
./uqm-debug -w -r 800x600 --safe

echo ""
echo "=== Ejecución terminada ==="