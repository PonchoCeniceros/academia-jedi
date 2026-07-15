---
layout: section
---

# Enums

---
layout: center
---

## ¿Qué es un enum?

1. Un **struct** agrupa varios valores que existen **todos a la vez** (un `Punto` siempre tiene `x` **y** `y`).
2. Un **enum** representa un valor que puede ser **una entre varias variantes posibles** — nunca dos al mismo tiempo.

<br>

```rust
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}

fn main() {
    let rumbo = Direccion::Norte;
}
```

---
layout: center
---

## Variantes con Datos

Cada variante puede llevar sus propios datos asociados, como si fuera un mini-struct:

```rust
enum Forma {
    Circulo(f64),         // radio
    Rectangulo(f64, f64), // ancho, alto
}

fn area(f: &Forma) -> f64 {
    match f {
        Forma::Circulo(r) => std::f64::consts::PI * r * r,
        Forma::Rectangulo(a, h) => a * h,
    }
}
```

* `match` (ya visto en Estructuras de Control) obliga a manejar cada variante — si falta una, el código no compila.
