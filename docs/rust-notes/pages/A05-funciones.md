---
layout: section
---

# Funciones

---
layout: center
---

## Punto de Entrada y Definición de Funciones

<br/>

```rust
fn bienvenida(nombre: &str) {   // función auxiliar
    println!("¡Hola, {}!", nombre);
}

fn main() {                     // punto de entrada obligatorio
    bienvenida("Rust");
}
```

<br>

* `fn` declara una función; el nombre sigue convención *snake_case*.
* `main` es el único identificador reservado como punto de arranque.
* Las funciones pueden definirse antes o después de `main`; el orden no importa.
* Se invocan escribiendo su nombre seguido de `()`.

---
layout: center
---

## Parámetros y Valores de Retorno


<br/>

```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b  // expresión sin `;` → valor de retorno implícito
}

fn main() {
    let resultado = sumar(3, 5);
    println!("{}", resultado); // 8
}
```

<br>

* Cada parámetro requiere nombre y tipo separados por `:` — Rust no los infiere.
* `->` seguido del tipo declara qué devuelve la función.
* La última **expresión** sin `;` es el retorno implícito; agregar `;` la convierte en enunciado y causa un error de compilación.
* `return valor;` permite retornos explícitos o anticipados.

---
layout: center
---

## Programa Completo


<br/>

<div class="grid grid-cols-2 gap-x-8 items-center mt-4">

  <div class="[&_pre]:my-0">

  ```rust
fn cuadrado(n: i32) -> i32 {
    n * n
}

fn mensaje(nombre: &str) -> String {
    format!("¡Hola, {}!", nombre)
}

fn main() {
    println!("{}", mensaje("Rust")); // ¡Hola, Rust!
    println!("4² = {}", cuadrado(4)); // 4² = 16
}
  ```

  </div>

  <div class="text-left text-sm space-y-4">

1. `main` orquesta el flujo general.
2. Las funciones auxiliares encapsulan tareas reutilizables.
3. Los valores fluyen mediante parámetros (entrada) y retornos (salida).

  </div>

</div>
