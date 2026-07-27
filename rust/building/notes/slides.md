---
theme: apple-basic
layout: intro
---

# Rust Aplicado
Notas de construcción

<div class="absolute bottom-10">
  <span class="font-700">
    Giovanny Alfonso Chávez Ceniceros
  </span>
</div>

---
layout: intro-image
---

<div class="absolute inset-0" style="background: linear-gradient(135deg, #F26244 0%, #F29441 100%);"></div>

<div class="absolute top-10 z-2">
  <span class="font-700 text-[#0D0500]">
    Giovanny Alfonso Chávez Ceniceros
  </span>
</div>

<div class="absolute bottom-10 z-2">
  <h1 class="text-[#0D0500]">Rust Aplicado</h1>
  <p class="text-[#0D0500]">Notas de construcción</p>
</div>

---
layout: center
---

# Contenido

<div class="grid grid-cols-3 gap-x-10 mt-6 text-left">

  <div>
    <h2 class="text-xl font-bold text-[#F26244] mb-4 border-b border-gray-700 pb-1">
      1. Módulos y Crates
    </h2>
    <ul class="space-y-3 list-none pl-0 text-sm">
      <li><span class="text-gray-500 font-mono">1.1</span> Módulos</li>
      <li><span class="text-gray-500 font-mono">1.2</span> Crates y Cargo</li>
    </ul>
  </div>

  <div>
    <h2 class="text-xl font-bold text-[#F27343] mb-4 border-b border-gray-700 pb-1">
      2. Arquitectura de Errores
    </h2>
    <ul class="space-y-3 list-none pl-0 text-sm">
      <li><span class="text-gray-500 font-mono">2.1</span> Errores propios</li>
      <li><span class="text-gray-500 font-mono">2.2</span> thiserror y anyhow</li>
    </ul>
  </div>

  <div>
    <h2 class="text-xl font-bold text-[#F28342] mb-4 border-b border-gray-700 pb-1">
      3. Async / Await
    </h2>
    <ul class="space-y-3 list-none pl-0 text-sm">
      <li><span class="text-gray-500 font-mono">3.1</span> Futures y .await</li>
      <li><span class="text-gray-500 font-mono">3.2</span> El runtime</li>
    </ul>
  </div>

</div>

<div class="mt-6 pt-3 border-t border-gray-800 text-sm text-gray-400 max-w-5xl mx-auto text-left">
  <span class="font-bold text-gray-300">Apéndices</span>
  &nbsp;&nbsp; <span class="font-mono text-gray-500">A</span> Tokio
  &nbsp;·&nbsp; <span class="font-mono text-gray-500">B</span> Axum
  &nbsp;·&nbsp; <span class="font-mono text-gray-500">C</span> Polars
</div>

---
layout: center
---

# Antes de Empezar

<div class="text-left max-w-4xl mx-auto">

Este deck asume lo cubierto en **Fundamentals**: ownership, borrowing, enums
(`Option`/`Result`), colecciones, structs, traits y generics.

<br>

<div class="important-note">

**División de responsabilidades.** Aquí viven los **conceptos**, que cambian
poco: cómo se organiza un proyecto, cómo se propagan los errores, cómo funciona
la concurrencia asíncrona.

Los **detalles de cada tecnología** (versiones, firmas concretas, recetas) viven
en el `README.md` de cada proyecto en `building/projects/`, junto al
`Cargo.toml` que fija su versión. Las APIs de las crates cambian seguido; estas
notas no deberían envejecer con ellas.

</div>

</div>

---
layout: intro-image
---

<div class="absolute inset-0" style="background: linear-gradient(135deg, #F26244 0%, #F29441 100%);"></div>

<div class="absolute bottom-10 z-2">
  <h1 class="text-[#0D0500]">Módulos y Crates</h1>
</div>

---
src: ./pages/A01-modulos.md
---
---
src: ./pages/A02-crates-cargo.md
---
