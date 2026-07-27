---
theme: apple-basic
layout: intro
---

# Rust
Notas del estudio

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
  <h1 class="text-[#0D0500]">Rust</h1>
  <p class="text-[#0D0500]">Notas de estudio</p>
</div>

---
layout: center
---

# Contenido

<div class="grid grid-cols-2 gap-x-16 gap-y-8 mt-6 text-left">

  <div>
    <h2 class="text-xl font-bold text-[#F26244] mb-4 border-b border-gray-700 pb-1">
      1. Sintaxis y Semántica
    </h2>
    <ul class="space-y-3 list-none pl-0 text-sm">
      <li><span class="text-gray-500 font-mono">1.1</span> Tokens</li>
      <li><span class="text-gray-500 font-mono">1.2</span> Tipos de Dato</li>
      <li><span class="text-gray-500 font-mono">1.3</span> Estructuras de Control</li>
      <li><span class="text-gray-500 font-mono">1.4</span> Tipos de Memoria</li>
      <li><span class="text-gray-500 font-mono">1.5</span> Funciones</li>
    </ul>
  </div>

  <div>
    <h2 class="text-xl font-bold text-[#F27343] mb-4 border-b border-gray-700 pb-1">
      2. Memory Safety
    </h2>
    <ul class="space-y-3 list-none pl-0 text-sm">
      <li><span class="text-gray-500 font-mono">2.1</span> Ownership</li>
      <li><span class="text-gray-500 font-mono">2.2</span> Borrowing y Lifetimes</li>
    </ul>
  </div>

</div>

---
layout: center
---

# Contenido (cont.)

<div class="grid grid-cols-2 gap-x-16 gap-y-8 mt-6 text-left">

  <div>
    <h2 class="text-xl font-bold text-[#F28342] mb-4 border-b border-gray-700 pb-1">
      3. Enums y Option
    </h2>
    <ul class="space-y-3 list-none pl-0 text-sm">
      <li><span class="text-gray-500 font-mono">3.1</span> Enums</li>
      <li><span class="text-gray-500 font-mono">3.2</span> Option</li>
    </ul>
  </div>

  <div>
    <h2 class="text-xl font-bold text-[#F29441] mb-4 border-b border-gray-700 pb-1">
      4. Estructuras de Datos
    </h2>
    <ul class="space-y-3 list-none pl-0 text-sm">
      <li><span class="text-gray-500 font-mono">4.1</span> Vec</li>
      <li><span class="text-gray-500 font-mono">4.2</span> Iteradores</li>
      <li><span class="text-gray-500 font-mono">4.3</span> HashMap y HashSet</li>
      <li><span class="text-gray-500 font-mono">4.4</span> Structs</li>
      <li><span class="text-gray-500 font-mono">4.5</span> Listas Enlazadas</li>
      <li><span class="text-gray-500 font-mono">4.6</span> Árboles</li>
      <li><span class="text-gray-500 font-mono">4.7</span> Big-O (resumen)</li>
    </ul>
  </div>

</div>

<div class="mt-6 pt-3 border-t border-gray-800 text-sm text-gray-400 max-w-5xl mx-auto text-left">
  <span class="font-bold text-gray-300">Apéndices</span>
  &nbsp;&nbsp; <span class="font-mono text-gray-500">A</span> Cheatsheet
  &nbsp;·&nbsp; <span class="font-mono text-gray-500">B</span> Aplicación
</div>

---
layout: intro-image
---

<div class="absolute inset-0" style="background: linear-gradient(135deg, #F26244 0%, #F29441 100%);"></div>

<div class="absolute bottom-10 z-2">
  <h1 class="text-[#0D0500]">Sintaxis y Semántica</h1>
</div>

---
src: ./pages/A01-tokens.md
---
---
src: ./pages/A02-tipos-dato.md
---
---
src: ./pages/A03-estructuras-control.md
---
---
src: ./pages/A04-tipos-memoria.md
---
---
src: ./pages/A05-funciones.md
---
---
layout: intro-image
---

<div class="absolute inset-0" style="background: linear-gradient(135deg, #F26244 0%, #F29441 100%);"></div>

<div class="absolute bottom-10 z-2">
  <h1 class="text-[#0D0500]">Memory Safety</h1>
</div>

---
src: ./pages/B01-ownership.md
---
---
src: ./pages/B02-borrowing-lifetimes.md
---

---
layout: intro-image
---

<div class="absolute inset-0" style="background: linear-gradient(135deg, #F26244 0%, #F29441 100%);"></div>

<div class="absolute bottom-10 z-2">
  <h1 class="text-[#0D0500]">Enums y Option</h1>
</div>

---
src: ./pages/C01-enums.md
---
---
src: ./pages/C02-option.md
---

---
layout: intro-image
---

<div class="absolute inset-0" style="background: linear-gradient(135deg, #F26244 0%, #F29441 100%);"></div>

<div class="absolute bottom-10 z-2">
  <h1 class="text-[#0D0500]">Estructuras de Datos</h1>
</div>

---
src: ./pages/D01-vec.md
---
---
src: ./pages/D02-iteradores.md
---
---
src: ./pages/D03-hashmap-hashset.md
---
---
src: ./pages/D04-structs.md
---
---
src: ./pages/D05-listas-enlazadas.md
---
---
src: ./pages/D06-arboles.md
---
---
src: ./pages/D07-big-o.md
---

---
layout: intro-image
---

<div class="absolute inset-0" style="background: linear-gradient(135deg, #F26244 0%, #F29441 100%);"></div>

<div class="absolute bottom-10 z-2">
  <h1 class="text-[#0D0500]">Apéndices</h1>
</div>

---
src: ./pages/E01-cheatsheet.md
---
---
src: ./pages/E02-aplicacion.md
---

