---
layout: section
---

# Vec

---
layout: center
---

## ¿Qué es un Vec?

1. Un `Vec<T>` es una **secuencia homogénea y de tamaño variable** — todos sus elementos son del mismo tipo `T`, pero puede crecer y encogerse en tiempo de ejecución.
2. A diferencia de un arreglo `[T; N]` (tamaño fijo, conocido en tiempo de compilación), un `Vec` no necesita saber cuántos elementos tendrá de antemano.

<br>

```rust
let arreglo: [i32; 3] = [1, 2, 3]; // tamaño fijo: siempre 3
let mut lista: Vec<i32> = Vec::new(); // vacío, crecerá
lista.push(1);
lista.push(2);
```

---
layout: center
---

## Dónde Vive un Vec

El contraste con Tipos de Memoria: un `Vec` reparte su información entre stack y heap.

<div class="important-note">

* En el **stack** vive el `Vec` en sí: tres datos de tamaño fijo — un **puntero** al buffer, la **longitud** (`len`) y la **capacidad** (`cap`).
* En el **heap** viven los **elementos** reales, en un buffer contiguo al que apunta ese puntero.

</div>

Por eso un `Vec` sigue las reglas de *ownership* de los datos en heap: asignarlo o pasarlo a una función lo **mueve** (no se copia el buffer completo).

---
layout: center
---

## Construcción y Operaciones

```rust
let mut v = Vec::new();     // vacío
let mut w = vec![10, 20];   // macro: con valores iniciales

v.push(1);          // agrega al final
v.push(2);
let ultimo = v.pop(); // quita el último y lo devuelve

let n = w.len();    // cantidad de elementos
```

<div class="important-note">

`.pop()` no devuelve un `i32` directo, sino un **`Option<T>`**: `Some(valor)` si había elementos, `None` si el `Vec` estaba vacío. Es el mismo `Option` de la sección anterior — el compilador te obliga a considerar el caso vacío.

</div>

---
layout: center
---

## Indexado Seguro vs. Directo

Hay dos formas de acceder a un elemento, y difieren en qué pasa si el índice no existe:

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left">
  <div>

```rust
let v = vec![10, 20, 30];

// acceso directo: panic si
// el índice no existe
let x = v[1]; // 20
let y = v[9]; // panic!
```

  </div>

<div>

```rust
let v = vec![10, 20, 30];

// acceso seguro: devuelve Option
let x = v.get(1); // Some(&20)
let y = v.get(9); // None
```

</div>

</div>

<br/>

> `.get(i)` devuelve `Option<&T>` en vez de arriesgar un *panic* — de nuevo, `Option` modelando la posible ausencia.

---
layout: center
---

## Slices: una Vista Prestada

Un **slice** `&[T]` es una referencia a una porción contigua de un `Vec` (o arreglo), sin tomar su propiedad — es *borrowing* aplicado a colecciones.

```rust
fn suma(nums: &[i32]) -> i32 {
    let mut total = 0;
    for &n in nums {
        total += n;
    }
    total
}

fn main() {
    let v = vec![1, 2, 3, 4];
    let total = suma(&v);        // &Vec<i32> se presta como &[i32]
    let parcial = suma(&v[1..3]); // solo una parte
}
```

* Recibir `&[T]` en vez de `&Vec<T>` hace la función más general: acepta tanto `Vec` como arreglos.
* El slice es un préstamo: `v` sigue siendo dueño de sus datos.
