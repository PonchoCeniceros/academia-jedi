---
layout: section
---

# Tipos de Dato

---
layout: center
---

<div class="text-sm">

| Categoría | Tipo | Descripción | Ejemplo / Sintaxis |
| :--- | :--- | :--- | :--- |
| **Numéricos** | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | Enteros con signo (complemento a 2). | `let x: i32 = -42;`<br>`let ptr: isize = -1;` |
| | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | Enteros sin signo. | `let x: u8 = 255;`<br>`let idx: usize = 0;` |
| | `f32`, `f64` | Números de punto flotante según estándar IEEE 754. | `let x: f64 = 3.1416;`<br>`let y: f32 = 2.5;` |

</div>

---
layout: center
---

<div class="text-sm">

| Categoría | Tipo | Descripción | Ejemplo / Sintaxis |
| :--- | :--- | :--- | :--- |
| **Texto** | `char` | Representa un carácter puro de cualquier alfabeto, emoji o símbolo. | `let c: char = 'z';`<br>`let emoji = '🦀';` |
| | `str` | Cadena de texto fija. Se utiliza casi siempre como referencia inline (`&str`). | `let s: &str = "hola";` |
| | `String` | Cadena de texto dinámica, mutable, de tamaño variable. | `let s = String::from("hola");`<br>`let s2 = "mundo".to_string();` |

</div>

---
layout: center
---


<div class="text-sm">

| Categoría | Tipo | Descripción | Ejemplo / Sintaxis |
| :--- | :--- | :--- | :--- |
| **Lógicos y Vacíos**| `bool` | Tipo booleano clásico de 1 byte de tamaño. | `let activo: bool = true;`<br>`let bandera = false;` |
| | `()` | Equivale al concepto de `void` de otros lenguajes. | `let vacio: () = ();`<br>`fn log() -> () {}` |

</div>

---
layout: center
---

<div class="text-sm">

| Categoría | Tipo | Descripción | Ejemplo / Sintaxis |
| :--- | :--- | :--- | :--- |
| **Colecciones** | `[T; N]` | Arreglo (*Array*). Colección homogénea de longitud fija conocida en tiempo de compilación (`N`). | `let arr: [i32; 3] = [1, 2, 3];` |
| | `(T, U, ...)` | Tupla. Colección heterogénea de longitud fija. | `let t: (i32, &str, bool) = (42, "hola", true);` |
| | `[T]` | Secuencia contigua de tamaño dinámico de tipos `T`. Se opera mediante `&[T]`. | `let sub_arr: &[i32] = &arr[1..3];` |

</div>

---
layout: center
---

<div class="text-sm">

| Categoría | Tipo | Descripción | Ejemplo / Sintaxis |
| :--- | :--- | :--- | :--- |
| **Personalizados** | `struct` | Estructuras. Permiten agrupar múltiples valores relacionados de distintos tipos bajo un mismo nombre. | `struct Usuario { ... }` |

</div>

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left">
  <div>

  ```rust
struct Person {
    name: String,
    age: u8,
    is_dev: bool,
}
```

  </div>
  <div>

```rust
fn main() {
    let developer = Person {
        nane: String::from("Gio"),
        age: 25,
        is_dev: true,
    };
}
```
  </div>
</div>

<br/>

> A diferencia de las tuplas, cada dato dentro de una `struct` tiene un **nombre claro (campo)**. Esto te permite modelar entidades del mundo real con un formato fijo que el compilador de Rust valida estrictamente.
