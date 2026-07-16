---
layout: section
---

# Cheatsheet
Hoja de referencia para la corrida mental

---
layout: center
---

## Ownership — ¿Se copia o se mueve?

<div class="flex flex-col gap-3 mt-3 max-w-5xl mx-auto text-left text-xs">

<div class="rounded px-4 py-3 border border-gray-700">
  <div class="grid gap-x-4 gap-y-2" style="grid-template-columns:1.7fr 0.8fr 1.3fr 0.9fr">
    <div class="opacity-50">Tipo de dato</div><div class="opacity-50">Dónde vive</div><div class="opacity-50">Al asignar / pasar / retornar</div><div class="opacity-50">¿Sigue usable?</div>
    <div class="font-mono">i32, u8, f64, bool, char</div><div>Stack</div><div><b>Copia</b> (Copy)</div><div>✅ Sí</div>
    <div class="font-mono">&amp;T, &amp;mut T</div><div>Stack</div><div><b>Copia</b> el puntero</div><div>✅ Sí *</div>
    <div class="font-mono">String, Vec&lt;T&gt;, Box&lt;T&gt;</div><div>Heap</div><div><b>Movimiento</b> (Move)</div><div>❌ No **</div>
  </div>
</div>

<div class="rounded px-4 py-3 border border-gray-700 space-y-1.5">
  <div class="font-bold text-[#F26244] mb-1">Las 3 reglas del ownership</div>
  <div>① Cada valor tiene un dueño (<i>owner</i>).</div>
  <div>② Solo un dueño a la vez.</div>
  <div>③ Cuando el dueño sale de scope, el valor se destruye (<i>drop</i>).</div>
  <div class="mt-2 opacity-90"><b>Gatillan transferencia:</b> <code>let y = x;</code> · <code>foo(x)</code> · <code>return x;</code></div>
</div>

<div class="opacity-60" style="font-size:0.6rem">* según las reglas de borrowing &nbsp;·&nbsp; ** salvo <code>.clone()</code></div>

</div>

---
layout: center
---

## Borrowing y Lifetimes — ¿Es válida esta referencia?

<div class="flex flex-col gap-3 mt-3 max-w-5xl mx-auto text-left text-xs">

<div class="rounded px-4 py-3 border border-gray-700">
  <div class="grid gap-x-4 gap-y-2" style="grid-template-columns:1fr 1.1fr 1.7fr">
    <div class="opacity-50">Referencia</div><div class="opacity-50">Permite</div><div class="opacity-50">¿Cuántas a la vez?</div>
    <div class="font-mono">&amp;T <span class="opacity-60">(compartida)</span></div><div>leer</div><div>✅ muchas simultáneas</div>
    <div class="font-mono">&amp;mut T <span class="opacity-60">(exclusiva)</span></div><div>leer y escribir</div><div>⚠️ una sola, y sin <code>&amp;</code> activas</div>
  </div>
</div>

<div class="rounded px-4 py-3" style="background:#F27F3D;color:#000">
  <b>Regla del borrow checker:</b> en un mismo instante puedes tener <b>una sola</b> <code style="background:#F29441;color:#000">&amp;mut T</code> <b>o bien cualquier cantidad de</b> <code style="background:#F29441;color:#000">&amp;T</code> — pero <b>nunca las dos a la vez</b>. Además, toda referencia debe ser siempre válida (sin <i>dangling</i>).
</div>

<div class="rounded px-4 py-3 border border-gray-700 space-y-2">

<div class="font-bold text-[#F26244]">Lifetimes</div>

**Ecuación:** $'a = \min\left(L(\text{entradas})\right)$; válido si $\max\left(L(r)\right) \le\ 'a$ — la referencia nunca vive más que su dueño.

</div>

</div>

---
layout: center
---

## Diagnóstico — el compilador se queja

<div class="flex flex-col gap-3 mt-3 max-w-5xl mx-auto text-left text-xs">

<div class="rounded px-4 py-3 border border-gray-700">
  <div class="grid gap-x-4 gap-y-2" style="grid-template-columns:1.6fr 1.4fr 1.4fr;font-size:0.68rem">
    <div class="opacity-50">Error del compilador</div><div class="opacity-50">Qué regla violaste</div><div class="opacity-50">Fix típico</div>
    <div class="font-mono">use of moved value</div><div>usaste un valor del heap <b>después</b> de moverlo</div><div><code>.clone()</code>, o pasar <code>&amp;x</code> en vez de <code>x</code></div>
    <div class="font-mono">cannot borrow as mutable … also borrowed as immutable</div><div>pediste <code>&amp;mut</code> con una <code>&amp;</code> todavía viva</div><div>termina de usar la ref inmutable antes</div>
    <div class="font-mono">cannot borrow as mutable more than once</div><div>dos <code>&amp;mut</code> al mismo tiempo</div><div>separa los scopes de cada préstamo</div>
    <div class="font-mono">x does not live long enough</div><div>la referencia sobrevive a su dueño</div><div>el dato debe vivir al menos tanto como la ref</div>
    <div class="font-mono">missing lifetime specifier</div><div>retornas <code>&amp;T</code> sin decir de cuál entrada</div><div>anota <code>'a</code> en firma y retorno</div>
  </div>
</div>

<div class="rounded px-4 py-3 border border-gray-700 space-y-1.5">
  <div class="font-bold text-[#F26244] mb-1">Corrida mental — pregúntate en orden</div>
  <div><b>①</b> ¿El dato vive en Heap? → asignarlo o pasarlo lo <b>mueve</b> (el original queda inutilizable). ¿En Stack? → se <b>copia</b>, ambos siguen vivos.</div>
  <div><b>②</b> ¿Hay una <code>&amp;mut</code> y una <code>&amp;</code> vivas a la vez sobre el mismo dato? → prohibido: ¿alguien escribe mientras otro lee?</div>
  <div><b>③</b> ¿La referencia que devuelvo o guardo vive más que su dueño? → <i>dangling</i>, prohibido.</div>
</div>

<div class="rounded px-4 py-3" style="background:#F27F3D;color:#000">
  En corto, casi todo error de memoria es una de tres: <b>¿lo moví y lo volví a usar?</b> · <b>¿mezclé lectura y escritura?</b> · <b>¿la referencia vive más que su dueño?</b>
</div>

</div>
