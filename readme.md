<h1 align="left">
  <img src="https://github.com/PonchoCeniceros/academia-jedi/blob/main/.assets/jedi.png" width="90" align="absmiddle">
  &nbsp;
  Academia Jedi
</h1>

> *"La fuerza de un Jedi fluye de la Fuerza. Pero cuidado. La ira, el miedo, la agresión — el lado oscuro son."*
> — Maestro Yoda

Este es mi campo de entrenamiento personal en los caminos de la Fuerza. Aquí, cada problema es una **Prueba** — un test de disciplina, claridad y dominio del código. Un Padawan no atraviesa las Pruebas con prisa. Medita, refina y regresa hasta que la solución fluye tan naturalmente como la Fuerza misma.

![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Pytest](https://img.shields.io/badge/pytest-0A9EDC?style=for-the-badge&logo=pytest&logoColor=white)
![Rust](https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![LeetCode](https://img.shields.io/badge/-LeetCode-FFA116?style=for-the-badge&logo=LeetCode&logoColor=black)

---

## 🏛️ Estructura del Templo

Cada Prueba reside en `trials/`, identificada por su ID de LeetCode. El Templo está dividido en dos cámaras — una por cada camino de la Fuerza.

```text
.
├── trials/
│   ├── python/              # La Cámara Python (Pytest)
│   │   ├── 1.py             # Two Sum
│   │   ├── 20.py            # Valid Parentheses
│   │   └── ...
│   └── rust/                # La Cámara Rust (cargo test)
│       ├── Cargo.toml       # Un solo proyecto para gobernarlos a todos
│       └── src/bin/         # Un pergamino por Prueba
│           ├── 1_two_sum.rs
│           └── ...
├── utils/                   # Los Holocrones (conocimiento compartido)
├── holocron_py.sh           # El Holocrón — camino Python
├── holocron_rs.sh           # El Holocrón — camino Rust
└── README.md                # El Códex Jedi
```

---

## 🌌 Las Pruebas

Una Prueba solo se considera **dominada** cuando todos los tests pasan — y la solución es tan clara y simple como el lado luminoso exige.

### Camino Python
```python
@pytest.mark.parametrize("input, expected", [
    (["flower","flow","flight"], "fl"),
    (["dog","racecar","car"], ""),
])
def test_solution(input, expected):
    assert Solution().longestCommonPrefix(input) == expected
```

### Camino Rust
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

## ⚔️ Los Holocrones (Automatización)

Los scripts **Holocrón** guían a los Padawans a través de sus Pruebas — generando los pergaminos, iniciando los tests y llevando registro de las misiones completadas. Un verdadero Jedi no desperdicia energía en configuración. Se concentra en la Fuerza.

### Camino Python (`holocron_py.sh`)
```bash
# Comenzar una nueva Prueba
./holocron_py.sh -m "9. Palindrome Number"

# Iniciar el entrenamiento
./holocron_py.sh -t 9

# Consultar el Registro de Pruebas
./holocron_py.sh -l
```

### Camino Rust (`holocron_rs.sh`)
```bash
# Comenzar una nueva Prueba
./holocron_rs.sh -m "1. Two Sum"

# Iniciar el entrenamiento
./holocron_rs.sh -t 1

# Ejecutar la Prueba
./holocron_rs.sh -r 1

# Consultar el Registro de Pruebas
./holocron_rs.sh -l
```

Ambos Holocrones conjuran pergaminos con:
- ⚔️ **Estructura de tests** lista para ser empuñada.
- 📡 **Logging estandarizado** con `utils.log` (Python).
- 🦀 **Binario Cargo** forjado por Prueba (Rust).

---

## 🔮 El Código Jedi

> *No hay emoción, hay paz.*
> *No hay ignorancia, hay conocimiento.*
> *No hay pasión, hay serenidad.*

1. **Claridad sobre Velocidad** — El código debe ser legible antes de ser ingenioso. El lado oscuro promete atajos; el Jedi construye cimientos.
2. **Refinamiento Continuo** — Una Prueba no termina cuando el test pasa. Termina cuando ya no se puede eliminar nada más.
3. **Abraza el Proceso** — Cada test fallido es una lección. Cada test aprobado es un paso más hacia la maestría.

*Que la Fuerza te acompañe.* 🌌
