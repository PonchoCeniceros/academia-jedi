---
layout: section
---

# Iteradores

---
layout: center
---

## Tres Formas de Iterar

La forma de recorrer un `Vec` decide qué le pasa a sus elementos — y es exactamente la distinción de *ownership* y *borrowing* ya vista:

<div class="important-note">

* **`.iter()`** — presta cada elemento como `&T` (solo lectura). El `Vec` sobrevive intacto.
* **`.iter_mut()`** — presta cada elemento como `&mut T` (permite modificarlo en su lugar).
* **`.into_iter()`** — **consume** el `Vec`: entrega cada elemento por valor (`T`) y el `Vec` deja de existir.

</div>

```rust
let v = vec![1, 2, 3];

for x in v.iter() {      // x: &i32
    println!("{x}");
}
// v sigue disponible aquí
```

---
layout: center
---

## Adaptadores Perezosos

Un **adaptador** transforma un iterador en otro, pero **no ejecuta nada todavía** — solo describe qué hacer.

```rust
let v = vec![1, 2, 3, 4];

let pares = v.iter().filter(|&&n| n % 2 == 0);
let dobles = v.iter().map(|n| n * 2);
```

* **`.filter(pred)`** — deja pasar solo los elementos que cumplen el predicado.
* **`.map(f)`** — transforma cada elemento aplicando la función `f`.

<div class="important-note">

Estas líneas no recorren nada aún. Un iterador es **perezoso**: describe una receta que solo se ejecuta cuando un **consumidor** la pide.

</div>

---
layout: center
---

## Consumidores

Un **consumidor** recorre el iterador de verdad y produce un resultado final:

```rust
let v = vec![1, 2, 3, 4];

let total: i32 = v.iter().sum();        // 10
let cuantos = v.iter().count();          // 4

let dobles: Vec<i32> = v.iter()
    .map(|n| n * 2)
    .collect();                          // [2, 4, 6, 8]
```

* **`.sum()` / `.count()`** — reducen el iterador a un solo valor.
* **`.collect()`** — junta todos los elementos en una colección (aquí, un `Vec` nuevo).

---
layout: center
---

## Encadenar la Receta

El patrón real: encadenar adaptadores perezosos y cerrar con un consumidor. Nada se ejecuta hasta el `.collect()` final.

```rust
let v = vec![1, 2, 3, 4, 5, 6];

let resultado: Vec<i32> = v.iter()
    .filter(|&&n| n % 2 == 0) // solo pares:   2, 4, 6
    .map(|n| n * 10)          // por diez:      20, 40, 60
    .collect();               // junta en Vec: [20, 40, 60]
```

<div class="important-note">

Cuando el compilador no puede deducir el tipo de destino de `.collect()`, se anota con *turbofish*: `.collect::<Vec<i32>>()`. Es la misma información que el `: Vec<i32>` de la variable, escrita del lado del método.

</div>
