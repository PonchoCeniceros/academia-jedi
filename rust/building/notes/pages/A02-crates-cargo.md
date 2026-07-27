---
layout: section
---

# Crates y Cargo

---
layout: center
---

## Crate: la Unidad de Compilación

Un **crate** es lo que el compilador procesa como una sola unidad. Hay dos tipos:

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left text-sm">

<div class="rounded px-4 py-3 border border-gray-700 space-y-1.5">
  <div class="font-bold text-[#F26244] mb-1">Binario</div>
  <div>Produce un ejecutable. Su raíz es <code>src/main.rs</code> y debe tener <code>fn main()</code>.</div>
  <div class="opacity-70">Un paquete puede tener varios: cada archivo en <code>src/bin/</code> es un binario aparte.</div>
</div>

<div class="rounded px-4 py-3 border border-gray-700 space-y-1.5">
  <div class="font-bold text-[#F26244] mb-1">Librería</div>
  <div>No se ejecuta: se importa desde otros crates. Su raíz es <code>src/lib.rs</code>.</div>
  <div class="opacity-70">Es lo que se publica y lo que otros consumen como dependencia.</div>
</div>

</div>

<div class="important-note">

El paquete de los trials tiene ambos: `src/lib.rs` (que exporta el macro `s!`) y 34 binarios en `src/bin/`. Por eso funciona `use katas::s;` desde cada binario — están consumiendo la librería del mismo paquete.

</div>

---
layout: center
---

## Paquete vs. Crate

Es la distinción que más confusión genera:

<div class="text-sm">

| Concepto | Qué es | Cómo se identifica |
| :--- | :--- | :--- |
| **Paquete** (*package*) | lo que Cargo administra | tiene un `Cargo.toml` |
| **Crate** | unidad de compilación | tiene una raíz (`main.rs` o `lib.rs`) |
| **Módulo** | organización interna | `mod` dentro de un crate |

</div>

<div class="important-note">

Un paquete contiene **como máximo una librería** y **cualquier cantidad de binarios**. Hablar de "el crate `katas`" y "el paquete `katas`" no es lo mismo, aunque compartan nombre.

</div>

---
layout: center
---

## `Cargo.toml`: el Manifiesto

```toml
[package]
name = "mi-api"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }

[dev-dependencies]
reqwest = "0.12"          # solo para tests y benchmarks
```

* `[dependencies]` — lo que el crate necesita para compilar y ejecutarse.
* `[dev-dependencies]` — lo que solo hace falta al correr tests; no viaja al binario final.
* **Features** — funcionalidad opcional de una crate; se activa solo lo que se usa, para no pagar compilación de más.

---
layout: center
---

## Versiones y el Lockfile

<div class="text-sm">

| En `Cargo.toml` | Significa | Acepta |
| :--- | :--- | :--- |
| `"1"` | compatible con 1.x | `1.0.0` … `<2.0.0` |
| `"1.2"` | compatible desde 1.2 | `1.2.0` … `<2.0.0` |
| `"=1.2.3"` | exactamente esa | solo `1.2.3` |

</div>

<div class="important-note">

`Cargo.toml` declara **rangos aceptables**; `Cargo.lock` fija la **versión exacta** que se resolvió. El lockfile se commitea en binarios (garantiza builds reproducibles) y tradicionalmente se omite en librerías, donde conviene probar contra el rango completo.

</div>

---
layout: center
---

## Comandos de Cargo

<div class="text-sm">

| Comando | Qué hace |
| :--- | :--- |
| `cargo new nombre` | crea un paquete binario (`--lib` para librería) |
| `cargo add axum` | agrega una dependencia al manifiesto |
| `cargo check` | valida que compile **sin generar binario** — el más rápido |
| `cargo build` | compila (`--release` para optimizado) |
| `cargo run` | compila y ejecuta (`--bin nombre` si hay varios) |
| `cargo test` | corre los tests |
| `cargo clippy` | linter: sugiere mejoras idiomáticas |
| `cargo fmt` | formatea según el estilo estándar |

</div>

<div class="important-note">

`cargo check` es el compañero del ciclo de escritura: verifica tipos, ownership y borrows en una fracción del tiempo de `build`, porque se salta la generación de código.

</div>

---
layout: center
---

## Workspaces: Varios Paquetes Juntos

Cuando un proyecto crece en varios paquetes que comparten dependencias y un solo `target/`:

```toml
# Cargo.toml en la raíz del workspace
[workspace]
members = ["api", "core", "cli"]
resolver = "3"
```

<div class="important-note">

El paquete de los trials **no** es un workspace, aunque se le haya llamado así: es un paquete simple con muchos binarios. Un workspace real tendría varios `Cargo.toml`, uno por miembro.

</div>

Ventaja: un único `Cargo.lock` y un `target/` compartido, así las dependencias comunes se compilan una sola vez.
