---
layout: section
---

# Traits

---
layout: center
---

## ¿Qué es un trait?

1. Un **trait** define un **contrato de comportamiento**: un conjunto de métodos que un tipo promete implementar.
2. No dice *qué* es el tipo, sino *qué sabe hacer*. Varios tipos distintos pueden cumplir el mismo trait.

<br>

```rust
trait Descriptible {
    fn describir(&self) -> String;
}

struct Punto { x: f64, y: f64 }
struct Usuario { nombre: String }

impl Descriptible for Punto {
    fn describir(&self) -> String { format!("({}, {})", self.x, self.y) }
}

impl Descriptible for Usuario {
    fn describir(&self) -> String { format!("usuario {}", self.nombre) }
}
```

---
layout: center
---

## Métodos por Default

Un trait puede traer implementaciones ya hechas. El tipo que lo implementa puede **usarlas tal cual o sobrescribirlas**.

```rust
trait Descriptible {
    fn describir(&self) -> String;

    // default: se apoya en describir()
    fn presentarse(&self) -> String {
        format!("Soy {}", self.describir())
    }
}
```

<div class="important-note">

Así se evita repetir la misma lógica en cada implementación: el trait define lo mínimo obligatorio (`describir`) y deriva el resto (`presentarse`) a partir de ello.

</div>

---
layout: center
---

## `derive` es Auto-Implementar Traits

Ya venías usando traits sin nombrarlos: `#[derive(...)]` le pide al compilador que **escriba la implementación por ti**.

<div class="text-sm">

| Derive | Qué habilita |
| :--- | :--- |
| `Debug` | imprimir con `{:?}` |
| `Clone` | el método `.clone()` |
| `PartialEq` / `Eq` | comparar con `==` |
| `PartialOrd` / `Ord` | `<`, `>`, y **ordenar** |
| `Hash` | ser **llave** de un `HashMap` / `HashSet` |

</div>

<div class="important-note">

Por eso un tipo necesita `Hash + Eq` para ser llave de un `HashMap`, y `Ord` para usarse en `sort()` o en un `BinaryHeap`: esas estructuras **exigen** esos traits.

</div>

---
layout: center
---

## Los Traits que Ya Usabas

<div class="text-sm">

| Lo que escribías | El trait detrás |
| :--- | :--- |
| `for x in v.iter()` · `.map()` · `.filter()` | **`Iterator`** — cada adaptador vive en este trait |
| `println!("{:?}", v)` | **`Debug`** |
| `v.sort()` | **`Ord`** sobre los elementos |
| closures como `\|x\| x * 2` | **`Fn`** / `FnMut` / `FnOnce` |
| `String::from("x")` | **`From`** |

</div>

<div class="important-note">

Los closures también son traits: `Fn` (solo lee lo que captura), `FnMut` (lo muta) y `FnOnce` (lo consume). Por eso `.map()` acepta cualquier closure: su firma pide un tipo que cumpla uno de esos traits, no un tipo concreto.

</div>
