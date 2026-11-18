#!/bin/bash

DIRECTORY="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="${DIRECTORY}/src/bin"
mkdir -p "$BIN_DIR"

show_help() {
  echo "problema (Rust)"
  echo "----------------------------------------------------------"
  echo "Uso:"
  echo "  problema.sh -n \"ID. Nombre\"  -> [N]uevo: crea el binario del problema"
  echo "  problema.sh -t <ID>            -> [T]est: corre las pruebas del problema"
  echo "  problema.sh -r <ID> [caso]     -> [R]un: ejecuta main (índice de caso opcional)"
  echo "  problema.sh -l                 -> [L]ista: problemas resueltos"
  echo ""
  echo "Ejemplos:"
  echo "  problema.sh -n \"1. Two Sum\""
  echo "  problema.sh -t 1"
  echo "  problema.sh -r 1"
  echo ""
  echo "Funciona desde cualquier carpeta: rust/fundamentals/problems/problema.sh -t 1"
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
  PACKAGE_NAME=$(echo "$PROBLEM_NAME" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9 ]//g' | sed 's/  */_/g' | sed 's/ /_/g')

  if [ -z "$NUMBER" ] || [ -z "$PROBLEM_NAME" ]; then
    echo "Error: formato inválido. Usa \"ID. Nombre\""
    exit 1
  fi

  BIN_NAME="${NUMBER}_${PACKAGE_NAME}"
  BIN_FILE="${BIN_DIR}/${BIN_NAME}.rs"

  if [ -f "$BIN_FILE" ]; then
    echo "El problema $NUMBER ya existe: $BIN_FILE"
  else
    cat <<EOF >"$BIN_FILE"
use colored::*;
use katas::s;

struct Solution;

/**
 * Solución a implementar
 */
impl Solution {
    pub fn ${PACKAGE_NAME}() {
        todo!()
    }
}

/**
 * Casos de prueba
 *
 * casos borde:
 * E1. ...
 */
fn get_test_cases() -> Vec<(/* input, expected */)> {
    vec![
      // /**/ (, ), // E
    ]
}

/**
 * Ejecución a discresión
 */
fn main() {
    const DEFAULT: usize = 0;

    let suite = get_test_cases();
    let idx: usize = std::env::var("CASO")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT);

    let (input, expected) = &suite[idx];
    let answer = Solution::${PACKAGE_NAME}(/* input */);
    println!(
        "answer: {} | expected: {}",
        format!("{}", answer).green().italic().underline(),
        format!("{}", expected).blue().italic().underline()
    );
}

/**
 * Ejecucion de la suite completa de pruebas
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_${PACKAGE_NAME}() {
      let suite = get_test_cases();

      for (input, expected) in suite {
        // assert_eq!(
        //   Solution::${PACKAGE_NAME}(/* input */),
        //   expected,
        //   "{}",
        //   format!("{:?}", input).red().italic().underline()
        // );
      }
    }
}
EOF

    echo "Problema $NUMBER creado en: $BIN_FILE"
  fi
  ;;

-t | --test | --train)
  BIN_NAME=$(find "$BIN_DIR" -maxdepth 1 -name "${VALUE}_*.rs" | head -1 | xargs basename 2>/dev/null | sed 's/\.rs$//')
  if [ -n "$BIN_NAME" ]; then
    echo "Probando el problema ${VALUE}..."
    cargo test --manifest-path "${DIRECTORY}/Cargo.toml" --bin "$BIN_NAME"
  else
    echo "Error: el problema '${VALUE}' no existe."
  fi
  ;;

-r | --run)
  CASE_INDEX=$3
  BIN_NAME=$(find "$BIN_DIR" -maxdepth 1 -name "${VALUE}_*.rs" | head -1 | xargs basename 2>/dev/null | sed 's/\.rs$//')
  if [ -n "$BIN_NAME" ]; then
    if [ -n "$CASE_INDEX" ]; then
      echo "Ejecutando el problema ${VALUE} (caso #${CASE_INDEX})..."
      CASO="$CASE_INDEX" cargo run --quiet --manifest-path "${DIRECTORY}/Cargo.toml" --bin "$BIN_NAME"
    else
      echo "Ejecutando el problema ${VALUE}..."
      cargo run --quiet --manifest-path "${DIRECTORY}/Cargo.toml" --bin "$BIN_NAME"
    fi
  else
    echo "Error: el problema '${VALUE}' no existe."
  fi
  ;;

-l | --list | --log)
  echo "Problemas resueltos (Rust):"
  echo "-----------------------------"
  for bin_file in "$BIN_DIR"/*.rs; do
    [ -f "$bin_file" ] && printf "  %s\n" "$(basename "$bin_file" .rs)"
  done
  ;;

*)
  show_help
  ;;
esac
