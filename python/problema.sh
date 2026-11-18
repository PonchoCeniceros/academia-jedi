#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIRECTORY="${SCRIPT_DIR}/problems"
mkdir -p "$DIRECTORY"

show_help() {
  echo "problema (Python)"
  echo "------------------------------------------------------------"
  echo "Uso:"
  echo "  problema.sh -n \"ID. Nombre\"  -> [N]uevo: crea el archivo del problema"
  echo "  problema.sh -t <ID>            -> [T]est: corre las pruebas del problema"
  echo "  problema.sh -l                 -> [L]ista: problemas resueltos"
  echo ""
  echo "Ejemplos:"
  echo "  problema.sh -n \"14. Longest Common Prefix\""
  echo "  problema.sh -t 14"
  echo ""
  echo "Funciona desde cualquier carpeta: python/problema.sh -t 14"
  echo "(-m y --train siguen funcionando como alias de -n y -t)"
  exit 1
}

if [ $# -eq 0 ]; then
  show_help
fi

OPTION=$1
VALUE=$2

case $OPTION in
-n | -m | --new | --meditate)
  INPUT=$VALUE
  NUMBER=$(echo "$INPUT" | grep -oE '^[0-9]+')
  PROBLEM_NAME=$(echo "$INPUT" | sed -E 's/^[0-9]+\.? *//')
  FUNCTION_NAME=$(echo "$PROBLEM_NAME" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9 ]//g' | sed 's/  */_/g' | sed 's/ /_/g')

  if [ -z "$NUMBER" ] || [ -z "$PROBLEM_NAME" ]; then
    echo "Error: formato inválido. Usa \"ID. Nombre\""
    exit 1
  fi

  FILE_PATH="${DIRECTORY}/${NUMBER}.py"

  if [ -f "$FILE_PATH" ]; then
    echo "El problema $NUMBER ya existe: $FILE_PATH"
  else
    cat <<EOF >"$FILE_PATH"
import pytest
from utils.log import Log
from typing import List, Dict, Optional

#
# $PROBLEM_NAME
#
class Solution:
    def solve(self, *args, **kwargs):
        # Implementa tu solución aquí
        pass

# Pruebas unitarias

@pytest.mark.parametrize(
    "params, expected",
    [
        # (input, output),
    ],
)
def test_${FUNCTION_NAME}(params, expected):
    sol = Solution()
    # assert sol.solve(params) == expected
EOF
    echo "Problema $NUMBER creado en: $FILE_PATH"
  fi
  ;;

-t | --test | --train)
  FILE_PATH="${DIRECTORY}/${VALUE}.py"
  if [ -f "$FILE_PATH" ]; then
    echo "Probando el problema $VALUE..."
    # Preferimos el python del venv via `-m pytest`: invocar `pytest` directo
    # depende del shebang de .venv/bin/pytest, que se rompe si el repo cambia
    # de ruta en disco.
    VENV_PY="${SCRIPT_DIR}/../.venv/bin/python"
    if [ -x "$VENV_PY" ]; then
      "$VENV_PY" -m pytest "$FILE_PATH" -v
    else
      pytest "$FILE_PATH" -v
    fi
  else
    echo "Error: el problema $VALUE no existe."
  fi
  ;;

-l | --list | --log)
  echo "Problemas resueltos (Python):"
  echo "-----------------------------"
  ls $DIRECTORY/*.py 2>/dev/null | sort -V | while read -r file; do
    ID=$(basename "$file" .py)
    NAME=$(grep -m 1 -E '^# +[^ ]' "$file" | sed 's/^# *//')
    printf "  %-4s | %s\n" "$ID" "$NAME"
  done
  ;;

*)
  show_help
  ;;
esac
