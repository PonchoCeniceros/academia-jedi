#!/bin/bash

DIRECTORY="trials/rust"
BIN_DIR="${DIRECTORY}/src/bin"
mkdir -p "$BIN_DIR"

show_help() {
  echo "🦀 HOLOCRON RS - Your guide in the Jedi Academy (Rust)"
  echo "----------------------------------------------------------"
  echo "Usage:"
  echo "  ./holocron_rs.sh -m \"ID. Name\"  -> [M]editate: Create new Trial"
  echo "  ./holocron_rs.sh -t <ID>          -> [T]rain: Run Trial tests"
  echo "  ./holocron_rs.sh -r <ID>          -> [R]un: Execute Trial main"
  echo "  ./holocron_rs.sh -l               -> [L]og: List completed Trials"
  echo ""
  echo "Examples:"
  echo "  ./holocron_rs.sh -m \"1. Two Sum\""
  echo "  ./holocron_rs.sh -t 1"
  echo "  ./holocron_rs.sh -r 1"
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
  PACKAGE_NAME=$(echo "$PROBLEM_NAME" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9 ]//g' | sed 's/  */_/g' | sed 's/ /_/g')

  if [ -z "$NUMBER" ] || [ -z "$PROBLEM_NAME" ]; then
    echo "❌ Error: Invalid format. Use \"ID. Name\""
    exit 1
  fi

  BIN_NAME="${NUMBER}_${PACKAGE_NAME}"
  BIN_FILE="${BIN_DIR}/${BIN_NAME}.rs"

  if [ -f "$BIN_FILE" ]; then
    echo "⚠️  This Trial already exists in the Academy. Keep training, Padawan!"
  else
    cat <<EOF >"$BIN_FILE"
use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn ${PACKAGE_NAME}() {
        // Implement your solution here
        todo!()
    }
}

fn main() {
    let ans = Solution::${PACKAGE_NAME}();
    println!("{}", format!("{}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_${PACKAGE_NAME}() {
        let cases = [
            // (, ),
        ];

        for (input, expected) in cases {
            todo!();
            // assert_eq!(
            //     Solution::${PACKAGE_NAME}(),
            //     expected,
            //     "{}",
            //     format!("{:?}", input).red().italic().underline()
            // );
        }
    }






}
EOF

    echo "⚔️  Trial $NUMBER is ready for training in: $BIN_FILE"
  fi
  ;;

-t | --train)
  BIN_NAME=$(find "$BIN_DIR" -maxdepth 1 -name "${VALUE}_*.rs" | head -1 | xargs basename 2>/dev/null | sed 's/\.rs$//')
  if [ -n "$BIN_NAME" ]; then
    echo "🌌 Initiating Trial ${VALUE}..."
    cargo test --manifest-path "${DIRECTORY}/Cargo.toml" --bin "$BIN_NAME"
  else
    echo "❌ Error: Trial '${VALUE}' does not exist."
  fi
  ;;

-r | --run)
  BIN_NAME=$(find "$BIN_DIR" -maxdepth 1 -name "${VALUE}_*.rs" | head -1 | xargs basename 2>/dev/null | sed 's/\.rs$//')
  if [ -n "$BIN_NAME" ]; then
    echo "🚀 Running Trial ${VALUE}..."
    cargo run --manifest-path "${DIRECTORY}/Cargo.toml" --bin "$BIN_NAME"
  else
    echo "❌ Error: Trial '${VALUE}' does not exist."
  fi
  ;;

-l | --log)
  echo "📜 Trial Log — Jedi Academy:"
  echo "-----------------------------"
  for bin_file in "$BIN_DIR"/*.rs; do
    [ -f "$bin_file" ] && printf "  %s\n" "$(basename "$bin_file" .rs)"
  done
  ;;

*)
  show_help
  ;;
esac
