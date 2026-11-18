# Estudio de programación competitiva

Soluciones a problemas de [LeetCode](https://leetcode.com) en **Python** y **Rust**, con pruebas para cada una, y material de estudio del lenguaje. Cada problema se identifica por su ID de LeetCode.

![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Pytest](https://img.shields.io/badge/pytest-0A9EDC?style=for-the-badge&logo=pytest&logoColor=white)
![Rust](https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![LeetCode](https://img.shields.io/badge/-LeetCode-FFA116?style=for-the-badge&logo=LeetCode&logoColor=black)

El avance está en [`roadmap.csv`](roadmap.csv): por problema, su complejidad, en qué lenguajes está resuelto y los conceptos clave.

## Cómo está organizado

El repo se organiza por **lenguaje**. En Rust hay dos pistas: *fundamentals* (dominar el lenguaje con algoritmos) y *building* (proyectos aplicados: APIs y análisis de datos, por crear).

```text
.
├── problema                 # punto de entrada: crear, probar y listar problemas
├── python/
│   ├── problems/            # una solución por archivo, con sus pruebas (pytest)
│   ├── utils/               # logging compartido
│   └── problema.sh
├── rust/
│   ├── fundamentals/
│   │   ├── notes/           # "Rust · Cuaderno de campo" (deck de Slidev)
│   │   └── problems/        # un solo paquete Cargo, un binario por problema
│   └── building/            # proyectos Cargo aplicados (por crear)
├── docs/
│   ├── algorithms/          # patrones de algoritmos, sin lenguaje (deck de Slidev)
│   └── whiteboards/         # bocetos
└── roadmap.csv              # metadatos y avance de cada problema
```

## Uso

Un solo comando, `./problema`, despacha al lenguaje indicado con `--py` o `--rs`. También funciona desde cualquier carpeta.

```bash
./problema --py -n "9. Palindrome Number"   # nuevo problema (crea el archivo con su plantilla de pruebas)
./problema --py -t 9                        # correr sus pruebas
./problema --rs -n "1. Two Sum"
./problema --rs -t 1
./problema --rs -r 1 [caso]                 # ejecutar main, con el índice de caso opcional (solo Rust)
./problema -l                               # listar los problemas de ambos lenguajes
```

Cada lenguaje tiene su propio script (`python/problema.sh`, `rust/fundamentals/problems/problema.sh`) con las mismas opciones. Para correr todas las pruebas:

```bash
.venv/bin/python -m pytest
cargo test --manifest-path rust/fundamentals/problems/Cargo.toml --no-fail-fast
```

### Forma de las pruebas

```python
@pytest.mark.parametrize("input, expected", [
    (["flower", "flow", "flight"], "fl"),
    (["dog", "racecar", "car"], ""),
])
def test_solution(input, expected):
    assert Solution().longestCommonPrefix(input) == expected
```

```rust
#[test]
fn test_two_sum() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
```

## Material de estudio

| Deck | Tema | Puerto |
| --- | --- | --- |
| `rust/fundamentals/notes` | **Rust · Cuaderno de campo**: el lenguaje y cómo construir con él | 3031 |
| `docs/algorithms` | **Algoritmos**: patrones, independiente del lenguaje | 3033 |

```bash
pnpm --dir rust/fundamentals/notes run dev
pnpm --dir docs/algorithms run dev
```

## Criterios

1. **Claridad sobre ingenio.** El código se lee antes de lucirse.
2. **Refinamiento continuo.** Un problema se da por terminado cuando ya no se le puede quitar nada.
3. **Proceso.** Cada prueba que falla enseña algo; la solución final debe salir natural.
