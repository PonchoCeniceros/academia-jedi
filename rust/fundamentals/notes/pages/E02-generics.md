---
layout: section
---

# Generics

---
layout: center
---

## El Problema: Repetir por Cada Tipo

Sin genéricos habría que escribir la misma función una vez por tipo:

```rust
fn mayor_i32(a: i32, b: i32) -> i32 { if a > b { a } else { b } }
fn mayor_f64(a: f64, b: f64) -> f64 { if a > b { a } else { b } }
```

<br>

Un **genérico** abstrae el tipo con un parámetro (`T`), y el compilador genera la versión concreta para cada uso:

```rust
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

---
layout: center
---

## Trait Bounds: Donde se Juntan Traits y Generics

`<T: PartialOrd>` es un ***trait bound***: no acepta *cualquier* tipo, sino cualquiera **que cumpla ese trait**.

<div class="important-note">

Se lee: *"`T` es cualquier tipo, **siempre que** se pueda comparar con `>`"*. Sin el bound el compilador rechaza `a > b`, porque no todo tipo sabe compararse.

</div>

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left">
  <div>

```rust
// un bound
fn mayor<T: PartialOrd>(a: T, b: T) -> T

// varios bounds
fn mostrar<T: Debug + Clone>(x: T)
```

  </div>

<div>

```rust
// sintaxis `where`: equivalente, y
// más legible con muchos bounds
fn procesar<T, U>(a: T, b: U)
where
    T: Debug + Clone,
    U: PartialOrd,
{ }
```

</div>

</div>

---
layout: center
---

## Structs Genéricos

El parámetro de tipo también sirve para definir estructuras que funcionan con cualquier contenido:

```rust
struct Par<T> {
    primero: T,
    segundo: T,
}

impl<T: PartialOrd> Par<T> {
    fn mayor(&self) -> &T {
        if self.primero > self.segundo { &self.primero } else { &self.segundo }
    }
}
```

* `struct Par<T>` declara el parámetro; `impl<T: ...> Par<T>` lo vuelve a declarar para el bloque de métodos.
* El bound puede vivir solo en el `impl`: así el struct admite cualquier `T`, pero `mayor()` existe únicamente si `T` es comparable.

---
layout: center
---

## Genéricos en las Secciones Anteriores

Todas estas estructuras, ya vistas, son genéricas:

<div class="text-sm">

| Tipo | Parámetros | Bound que exige |
| :--- | :--- | :--- |
| `Vec<T>` | el tipo de los elementos | ninguno |
| `Option<T>` / `Result<T, E>` | el valor y el error | ninguno |
| `Box<T>` / `Rc<T>` / `RefCell<T>` | el dato apuntado | ninguno |
| `HashMap<K, V>` | llave y valor | `K: Hash + Eq` |
| `BinaryHeap<T>` | los elementos | `T: Ord` |

</div>

<div class="important-note">

**Costo cero:** el compilador genera una versión concreta de cada genérico por cada tipo usado (*monomorfización*). Un `Vec<i32>` es tan rápido como uno escrito a mano para `i32` — la abstracción no cuesta nada en tiempo de ejecución.

</div>
