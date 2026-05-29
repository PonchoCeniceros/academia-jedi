#!/bin/bash

DIRECTORY="trials/python"
mkdir -p "$DIRECTORY"

show_help() {
  echo "🌌 HOLOCRON PY - Your guide in the Jedi Academy (Python)"
  echo "------------------------------------------------------------"
  echo "Usage:"
  echo "  ./holocron_py.sh -m \"ID. Name\"  -> [M]editate: Create new Trial"
  echo "  ./holocron_py.sh -t <ID>          -> [T]rain: Run Trial tests"
  echo "  ./holocron_py.sh -l               -> [L]og: List completed Trials"
  echo ""
  echo "Examples:"
  echo "  ./holocron_py.sh -m \"14. Longest Common Prefix\""
  echo "  ./holocron_py.sh -t 14"
  exit 1
}

if [ $# -eq 0 ]; then
  show_help
fi

OPTION=$1
VALUE=$2

case $OPTION in
-m | --meditate)
  INPUT=$VALUE
  NUMBER=$(echo "$INPUT" | grep -oE '^[0-9]+')
  PROBLEM_NAME=$(echo "$INPUT" | sed -E 's/^[0-9]+\.? *//')
  FUNCTION_NAME=$(echo "$PROBLEM_NAME" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9 ]//g' | sed 's/  */_/g' | sed 's/ /_/g')

  if [ -z "$NUMBER" ] || [ -z "$PROBLEM_NAME" ]; then
    echo "❌ Error: Invalid format. Use \"ID. Name\""
    exit 1
  fi

  FILE_PATH="${DIRECTORY}/${NUMBER}.py"

  if [ -f "$FILE_PATH" ]; then
    echo "⚠️  This Trial already exists in the Academy. Keep training, Padawan!"
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
        # Implement your solution here
        pass

# Unit tests

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
    echo "⚔️  Trial $NUMBER is ready for training in: $FILE_PATH"
  fi
  ;;

-t | --train)
  FILE_PATH="${DIRECTORY}/${VALUE}.py"
  if [ -f "$FILE_PATH" ]; then
    echo "🌌 Initiating Trial $VALUE..."
    pytest "$FILE_PATH" -v
  else
    echo "❌ Error: Trial $VALUE does not exist."
  fi
  ;;

-l | --log)
  echo "📜 Trial Log — Jedi Academy:"
  echo "-----------------------------"
  ls $DIRECTORY/*.py 2>/dev/null | sort -V | while read -r file; do
    ID=$(basename "$file" .py)
    NAME=$(grep -m 1 "#" "$file" | sed 's/#//' | xargs)
    printf "  %-4s | %s\n" "$ID" "$NAME"
  done
  ;;

*)
  show_help
  ;;
esac
