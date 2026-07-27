---
layout: section
---

# Result

---
layout: center
---

## El Problema: Operaciones que Fallan

1. `Option` modela la **ausencia** de un valor. Pero muchas operaciones no solo pueden "no dar nada": pueden **fallar por una razón concreta** (archivo inexistente, texto que no es un número, red caída).
2. En otros lenguajes eso se maneja con excepciones. Rust no tiene excepciones: el error viaja **en el tipo de retorno**, igual que `Option`.

<br>

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<div class="important-note">

`Result<T, E>` es otro enum de la librería estándar: `Ok(valor)` si salió bien, `Err(error)` si falló. Al ser parte del tipo, **el compilador no permite ignorar la posibilidad de error**.

</div>

---
layout: center
---

## Option vs. Result

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left">
  <div>

```rust
// ¿hay valor o no?
fn buscar(v: &[i32]) -> Option<usize>

// Some(3) / None
```

  </div>

<div>

```rust
// ¿salió bien o por qué falló?
fn leer(t: &str) -> Result<i32, ParseIntError>

// Ok(42) / Err(...)
```

</div>

</div>

<br/>

<div class="important-note">

Regla práctica: cuando la ausencia es **normal y esperada** (el elemento no está), corresponde `Option`. Cuando es un **fallo que amerita explicación**, corresponde `Result` — el `E` carga el motivo.

</div>

---
layout: center
---

## Manejar un Result

```rust
fn main() {
    let texto = "42";

    match texto.parse::<i32>() {
        Ok(n)  => println!("el número es {n}"),
        Err(e) => println!("no se pudo convertir: {e}"),
    }
}
```

* `match` obliga a cubrir ambos casos, igual que con `Option`.
* `if let Ok(n) = texto.parse::<i32>()` sirve cuando solo interesa el caso exitoso.
* `.unwrap()` extrae el `Ok` y hace *panic* si es `Err`; `.expect("mensaje")` es igual pero con un mensaje propio.

---
layout: center
---

## El Operador `?`

Escribir un `match` por cada operación que puede fallar es insostenible. El operador `?` lo resume: **si es `Ok`, extrae el valor; si es `Err`, retorna el error de inmediato.**

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
// con match: verboso
fn sumar(a: &str, b: &str)
    -> Result<i32, ParseIntError> {
    let x = match a.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    let y = match b.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}
```

  </div>

<div>

```rust
// con ?: la misma lógica
fn sumar(a: &str, b: &str)
    -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}
```

</div>

</div>

<div class="important-note">

`?` solo puede aparecer dentro de una función que **devuelva `Result`** (o `Option`): requiere un destino al que propagar el error. Por eso el retorno sigue siendo `Result` y el éxito se envuelve en `Ok(...)`.

</div>

---
layout: center
---

## Result Estaba a la Vista

Cada `.unwrap()` aplicado sobre algo que no era un `Option` operaba en realidad sobre un `Result`:

```rust
let re = Regex::new(r"[0-9]+").unwrap();   // Result<Regex, Error>
let n: i32 = "42".parse().unwrap();        // Result<i32, ParseIntError>
```

<div class="important-note">

`.unwrap()` es admisible en un trial o un prototipo, donde un *panic* no tiene consecuencias. En código real se prefiere `?` para propagar el error, o `match` para decidir qué hacer con él.

</div>

Y el patrón se repite: `Option` y `Result` son **enums con dos variantes** sobre los que se hace *pattern matching* — la misma "cebolla" que se pela con `match`, `if let` o `?`.
