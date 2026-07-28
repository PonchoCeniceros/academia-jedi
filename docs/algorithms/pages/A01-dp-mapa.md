---
layout: section
---

# Programación Dinámica
El mapa mental

---
layout: center
---

## La Premisa

<div class="important-note">

La función **no resuelve el problema completo**. Resuelve el **mejor resultado posible a partir de un estado**.

</div>

<br>

Ese cambio de encuadre es lo que hace tratable un problema de optimización recursivo: en lugar de preguntar *"¿cuál es la respuesta?"*, se pregunta *"¿cuál es la respuesta **desde aquí**?"*.

El problema original se vuelve un caso particular: la respuesta desde el estado inicial.

---
layout: center
---

## Las Cuatro Preguntas

<div class="flex flex-col gap-3 mt-3 max-w-5xl mx-auto text-left text-sm">

<div class="rounded px-4 py-3 border border-gray-700">
  <div class="grid gap-x-4 gap-y-2" style="grid-template-columns:auto 1.1fr 1.4fr">
    <div class="opacity-50">Pregunta</div><div class="opacity-50">Qué determina</div><div class="opacity-50">Ejemplos</div>
    <div class="font-bold text-[#F26244]">A. ¿Cuál es mi estado?</div><div>los parámetros de la función y la clave del memo</div><div class="opacity-80">el escalón <code>i</code> · la casa <code>i</code> · la celda <code>(i,j)</code> · el monto restante</div>
    <div class="font-bold text-[#F26244]">B. ¿Qué decisiones puedo tomar?</div><div>las llamadas recursivas</div><div class="opacity-80">subir 1 o 2 escalones · robar o no · derecha o abajo</div>
    <div class="font-bold text-[#F26244]">C. ¿Qué optimizo?</div><div>el significado del valor devuelto</div><div class="opacity-80">maximizar dinero · minimizar costo · maximizar longitud · contar</div>
    <div class="font-bold text-[#F26244]">D. ¿Cómo combino las decisiones?</div><div>el operador que une las ramas</div><div class="opacity-80">ver la tabla siguiente</div>
  </div>
</div>

<div class="rounded px-4 py-3" style="background:#F27F3D;color:#000">
  Respondidas las cuatro, la función prácticamente se escribe sola: <b>A</b> da la firma, <b>B</b> da el cuerpo, <b>C</b> da el tipo de retorno y <b>D</b> da la línea que lo une todo.
</div>

</div>

---
layout: center
---

## D · El Operador de Combinación

El objetivo (C) determina cómo se unen los resultados de las decisiones (B):

<div class="flex flex-col gap-3 mt-3 max-w-5xl mx-auto text-left text-sm">

<div class="rounded px-4 py-3 border border-gray-700">
  <div class="grid gap-x-4 gap-y-2" style="grid-template-columns:1.3fr 0.8fr 1.5fr">
    <div class="opacity-50">Si el objetivo es…</div><div class="opacity-50">Operador</div><div class="opacity-50">Se lee como</div>
    <div>el mejor beneficio</div><div class="font-mono font-bold">max</div><div class="opacity-80">la mejor de las ramas</div>
    <div>el menor costo</div><div class="font-mono font-bold">min</div><div class="opacity-80">la rama más barata</div>
    <div>contar formas</div><div class="font-mono font-bold">suma</div><div class="opacity-80">todas las ramas se acumulan</div>
    <div>saber si existe</div><div class="font-mono font-bold">OR</div><div class="opacity-80">basta con que una rama funcione</div>
  </div>
</div>

<div class="rounded px-4 py-3" style="background:#F27F3D;color:#000">
  <b>Contar es distinto de optimizar.</b> Con <code style="background:#F29441;color:#000">max</code> o <code style="background:#F29441;color:#000">min</code> se elige <i>una</i> rama y se descartan las demás; con <code style="background:#F29441;color:#000">suma</code> se conservan <b>todas</b>, porque son caminos disjuntos que se suman.
</div>

</div>

---
layout: center
---

## E · ¿Qué Representa lo Imposible?

Una quinta pregunta que se desprende de la D: cuando una decisión no conduce a ninguna solución válida, hay que devolver algo que **no contamine la combinación**. Ese algo es la identidad del operador:

<div class="flex flex-col gap-3 mt-3 max-w-5xl mx-auto text-left text-sm">

<div class="rounded px-4 py-3 border border-gray-700">
  <div class="grid gap-x-4 gap-y-2" style="grid-template-columns:0.8fr 1fr 1.8fr">
    <div class="opacity-50">Operador</div><div class="opacity-50">Valor neutro</div><div class="opacity-50">Por qué</div>
    <div class="font-mono">suma</div><div class="font-mono">0</div><div class="opacity-80">sumar cero no agrega formas</div>
    <div class="font-mono">min</div><div class="font-mono">None</div><div class="opacity-80">un centinela explícito evita el overflow de usar ∞</div>
    <div class="font-mono">max</div><div class="font-mono">None</div><div class="opacity-80">mismo motivo, por el lado opuesto</div>
    <div class="font-mono">OR</div><div class="font-mono">false</div><div class="opacity-80">una rama fallida no invalida a las demás</div>
  </div>
</div>

<div class="rounded px-4 py-3" style="background:#F27F3D;color:#000">
  Usar <code style="background:#F29441;color:#000">i32::MAX</code> como "infinito" en un <code style="background:#F29441;color:#000">min</code> es la trampa clásica: en cuanto se le suma 1 para contar un paso más, desborda. Un <code style="background:#F29441;color:#000">Option</code> hace explícito el caso imposible y el compilador obliga a manejarlo.
</div>

</div>

---
layout: center
---

## Del Mapa al Código

Las respuestas caen en lugares fijos de la función:

```rust
//  A → la firma: el estado son los parámetros que varían
fn resolver(estado: usize, datos: &[i32], memo: &mut HashMap<usize, i32>) -> i32 {

    //  el caso base: el estado donde ya no hay decisiones que tomar
    if /* estado terminal */ { return /* E: valor neutro */; }

    //  memoización: la clave debe ser exactamente el estado de A
    if let Some(&ans) = memo.get(&estado) { return ans; }

    //  B → una llamada recursiva por decisión
    let opcion_1 = /* ... */ resolver(estado + 1, datos, memo);
    let opcion_2 = /* ... */ resolver(estado + 2, datos, memo);

    //  D → el operador que las combina (C decide cuál)
    let ans = max(opcion_1, opcion_2);

    memo.insert(estado, ans);
    ans
}
```

---
layout: center
---

## La Clave del Memo es el Estado

<div class="important-note">

La clave del memo debe capturar **todo** lo que hace distinto a un estado. Si dos llamadas con el mismo estado pueden dar respuestas diferentes, la respuesta A estaba incompleta y la memoización devolverá resultados incorrectos.

</div>

<br>

* Si la función recibe parámetros que **no** forman parte del estado (por ejemplo, un dato derivable o constante), no deben entrar en la clave.
* Si el estado real es un par `(i, j)`, la clave tiene que ser el par — memoizar solo por `i` mezcla estados distintos.
* Una firma con más parámetros que la clave del memo es una señal a revisar: puede estar bien, o puede ser un error latente.

---
layout: center
---

## Después de que Funciona

El mapa produce una solución **recursiva con memoización** (*top-down*). Es la forma más directa de traducir el razonamiento a código, y suele ser suficiente. Dos pasos opcionales después:

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left text-sm">

<div class="rounded px-4 py-3 border border-gray-700 space-y-1.5">
  <div class="font-bold text-[#F26244] mb-1">Aplanar a iterativo (bottom-up)</div>
  <div>Recorrer los estados en orden y llenar una tabla, en vez de recursión.</div>
  <div class="opacity-70">Elimina el riesgo de desbordar la pila con entradas grandes.</div>
</div>

<div class="rounded px-4 py-3 border border-gray-700 space-y-1.5">
  <div class="font-bold text-[#F26244] mb-1">Reducir el espacio</div>
  <div>Si cada estado solo depende de los <i>k</i> anteriores, basta con <i>k</i> variables en vez de la tabla completa.</div>
  <div class="opacity-70">Es lo que lleva una solución de O(n) espacio a O(1).</div>
</div>

</div>

<div class="important-note">

Ninguno de los dos cambia el razonamiento: A, B, C y D siguen siendo los mismos. Solo cambia la mecánica de cómo se recorren y almacenan los estados.

</div>
