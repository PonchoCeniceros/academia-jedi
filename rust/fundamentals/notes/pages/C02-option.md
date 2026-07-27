---
layout: section
---

# Option

---
layout: center
---

## El Problema: Ausencia de Valor

Muchos lenguajes usan `null` para representar "no hay valor". Rust no tiene `null` — el mismo problema se resuelve con un enum de la librería estándar:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

<div class="important-note">

`Option<T>` no es un caso especial del lenguaje — es un enum como cualquier otro, solo que tan común que el compilador lo trae disponible sin necesidad de importarlo.

</div>

---
layout: center
---

## Usar Option

```rust
fn buscar(nums: &[i32], objetivo: i32) -> Option<usize> {
    for (i, &n) in nums.iter().enumerate() {
        if n == objetivo {
            return Some(i);
        }
    }
    None
}

fn main() {
    match buscar(&[3, 7, 1], 7) {
        Some(i) => println!("encontrado en {i}"),
        None => println!("no encontrado"),
    }
}
```

* `Some(i)` envuelve el valor cuando existe; `None` marca su ausencia.
* El compilador obliga a manejar ambos casos — no hay forma de "olvidarse" de comprobar si hay valor, a diferencia de un `null` que puede fallar en tiempo de ejecución.

---
layout: center
---

## `if let` y `while let` sobre Option

Cuando solo interesa un caso (usualmente `Some`), `match` es más verboso de lo necesario — para eso ya viste `if let` y `while let`:

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left">
  <div>

```rust
let dato: Option<i32> = Some(5);

if let Some(v) = dato {
    println!("hay un valor: {v}");
}
```

  </div>

<div>

```rust
let mut pila: Vec<Option<i32>> =
    vec![Some(1), Some(2), None];

while let Some(v) = pila.pop() {
    println!("{v:?}");
}
```

</div>

</div>

---
layout: center
---

## Combinators Comunes

<div class="important-note">

* **`.unwrap()`**: extrae el valor de `Some`, o hace *panic* si es `None` — úsalo solo cuando estás seguro de que hay valor.
* **`.is_some()` / `.is_none()`**: preguntan por el caso sin extraer el valor.
* **`.map(f)`**: transforma el valor interno si existe, sin tener que escribir un `match` manualmente.
* **`.take()`**: extrae el valor y deja `None` en su lugar.

</div>

```rust
let a: Option<i32> = Some(3);

let b = a.map(|v| v * 2); // Some(6)
let c = a.unwrap_or(0);   // 3 (o 0 si a fuera None)
```

---
layout: center
---

## Lo que Ya Estabas Usando

El patrón `Option<Box<Nodo>>` de la siguiente sección (Listas Enlazadas, Árboles) es exactamente esto: un enum con dos variantes, `Some` y `None`, sobre el que se hace *pattern matching* para decidir si seguir recorriendo la estructura o detenerse — y `Option::take()` es el mismo combinator que ya se mencionó aquí.
