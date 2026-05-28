#!/bin/bash

DIRECTORY="katas/rust"
mkdir -p "$DIRECTORY"

show_help() {
  echo "🦀 SENSEI RS - Tu guía en el Coding Dojo (Rust)"
  echo "------------------------------------------------"
  echo "Uso:"
  echo "  ./sensei_rs.sh -m \"ID. Nombre\"  -> [M]editar: Crear nueva Kata"
  echo "  ./sensei_rs.sh -e <ID>            -> [E]ntrenar: Ejecutar tests de la Kata"
  echo "  ./sensei_rs.sh -r <ID>            -> [R]un: Ejecutar main de la Kata"
  echo "  ./sensei_rs.sh -l                 -> [L]istar: Ver tus Katas completadas"
  echo ""
  echo "Ejemplos:"
  echo "  ./sensei_rs.sh -m \"1. Two Sum\""
  echo "  ./sensei_rs.sh -e 1"
  echo "  ./sensei_rs.sh -r 1"
  exit 1
}

if [ $# -eq 0 ]; then
  show_help
fi

OPTION=$1
VALUE=$2

case $OPTION in
-m | --meditar)
  INPUT=$VALUE
  NUMBER=$(echo "$INPUT" | grep -oE '^[0-9]+')
  PROBLEM_NAME=$(echo "$INPUT" | sed -E 's/^[0-9]+\.? *//')
  PACKAGE_NAME=$(echo "$PROBLEM_NAME" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9 ]//g' | sed 's/  */_/g' | sed 's/ /_/g')

  if [ -z "$NUMBER" ] || [ -z "$PROBLEM_NAME" ]; then
    echo "❌ Error: Formato inválido. Usa \"ID. Nombre\""
    exit 1
  fi

  PROJECT_PATH="${DIRECTORY}/${NUMBER}_${PACKAGE_NAME}"

  if [ -d "$PROJECT_PATH" ]; then
    echo "⚠️  Esa Kata ya existe en tu Dojo. ¡Sigue entrenando!"
  else
    mkdir -p "${PROJECT_PATH}/src"

    cat <<EOF >"${PROJECT_PATH}/Cargo.toml"
[package]
name = "${PACKAGE_NAME}"
version = "0.1.0"
edition = "2024"

[dependencies]
colored = "3"
EOF

    cat <<EOF >"${PROJECT_PATH}/src/main.rs"
use colored::*;

struct Solution;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn ${PACKAGE_NAME}() {
        // Implementa tu solución aquí
        todo!()
    }
}

/**
 * Pruebas unitarias
 *
 */
fn main() {
  println!("{}", "Hello World!".black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_${PACKAGE_NAME}() {
        // assert_eq!(Solution::${PACKAGE_NAME}(), expected);
        todo!()
    }
}
EOF

    cat <<EOF >"${PROJECT_PATH}/.gitignore"
target/
EOF

    echo "🥋 Kata $NUMBER preparada para meditación en: $PROJECT_PATH"
  fi
  ;;

-e | --entrenar)
  PROJECT_PATH=$(find "$DIRECTORY" -maxdepth 1 -type d -name "${VALUE}_*" | head -1)
  if [ -n "$PROJECT_PATH" ]; then
    echo "⚔️  Entrenando Kata ${VALUE}..."
    cargo test --manifest-path "${PROJECT_PATH}/Cargo.toml"
  else
    echo "❌ Error: La Kata '${VALUE}' no existe."
  fi
  ;;

-r | --run)
  PROJECT_PATH=$(find "$DIRECTORY" -maxdepth 1 -type d -name "${VALUE}_*" | head -1)
  if [ -n "$PROJECT_PATH" ]; then
    echo "🚀 Ejecutando Kata ${VALUE}..."
    cargo run --manifest-path "${PROJECT_PATH}/Cargo.toml"
  else
    echo "❌ Error: La Kata '${VALUE}' no existe."
  fi
  ;;

-l | --listar)
  echo "📜 Registro de Katas Rust en el Dojo:"
  echo "--------------------------------------"
  for project in "$DIRECTORY"/*/; do
    if [ -f "${project}Cargo.toml" ]; then
      NAME=$(basename "$project")
      printf "  %s\n" "$NAME"
    fi
  done
  ;;

*)
  show_help
  ;;
esac
