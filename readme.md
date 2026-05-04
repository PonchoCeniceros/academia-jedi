# The Coding Dojo: Algorithmic Katas

Este repositorio es mi espacio personal de entrenamiento. Aquí no solo resuelvo problemas, sino que practico la **maestría del código** a través de *Katas*: ejercicios diseñados para perfeccionar la lógica, la sintaxis y el pensamiento algorítmico mediante la repetición y el refinamiento.

![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Pytest](https://img.shields.io/badge/pytest-0A9EDC?style=for-the-badge&logo=pytest&logoColor=white)
![Rust](https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![LeetCode](https://img.shields.io/badge/-LeetCode-FFA116?style=for-the-badge&logo=LeetCode&logoColor=black)
---

## Estructura del Dojo

He organizado el entorno para que sea minimalista y eficiente. Cada ejercicio (Kata) reside en la carpeta `katas/`, identificada por su ID de LeetCode.

```text
.
├── katas/                   # Katas en Python (Pytest)
│   ├── 1.py                 # Two Sum
│   ├── 20.py                # Valid Parentheses
│   ├── 26.py                # Remove Duplicates from Sorted Array
│   ├── 28.py                # Find the Index of the First Occurrence in a String
│   ├── 70.py                # Climbing Stairs
│   ├── ...
│   └── rust/                # Katas en Rust (cargo test)
│       └── two_sum/         # Proyecto Cargo independiente por Kata
│           ├── src/main.rs
│           └── Cargo.toml
├── utils/                   # Utilidades de apoyo (Loggers, etc.)
├── sensei.sh                # Automatización de Katas Python
├── sensei_rs.sh             # Automatización de Katas Rust
└── README.md                # El manifiesto del Dojo
```

---

## Laboratorio de Pruebas (Testing)

En este Dojo, una solución solo se considera "dominada" cuando supera todos los casos de prueba de forma elegante.

### Python con Pytest
Utilizo tests parametrizados para una validación exhaustiva:
```python
@pytest.mark.parametrize("input, expected", [
    (["flower","flow","flight"], "fl"),
    (["dog","racecar","car"], ""),
])
def test_solution(input, expected):
    assert Solution().longestCommonPrefix(input) == expected
```

### Rust con cargo test
Cada Kata es un proyecto Cargo independiente con tests unitarios integrados:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
```

---

## 🛠️ El "Sensei" (Automatización)

Para mantener el enfoque en la lógica y no en la configuración, utilizo scripts en Bash para generar el *boilerplate* de mis Katas.

### Python (`sensei.sh`)
```bash
# Generar nueva Kata
./sensei.sh -m "9. Palindrome Number"

# Ejecutar tests
./sensei.sh -e 9

# Listar Katas
./sensei.sh -l
```

### Rust (`sensei_rs.sh`)
```bash
# Generar nueva Kata
./sensei_rs.sh -m "1. Two Sum"

# Ejecutar tests
./sensei_rs.sh -e two_sum

# Listar Katas
./sensei_rs.sh -l
```

Ambos scripts generan esqueletos automáticos con:
- ✅ **Estructura de tests** lista para completar.
- ✅ **Logging estandarizado** con `utils.log` (Python).
- ✅ **Proyecto Cargo** configurado con `.gitignore` incluido (Rust).

---

## Filosofía de Entrenamiento

1. **Claridad sobre Velocidad:** El código debe ser legible antes que ingenioso.
2. **Refactorización Continua:** Una Kata no termina al pasar el test, sino cuando el código es lo más simple posible.

