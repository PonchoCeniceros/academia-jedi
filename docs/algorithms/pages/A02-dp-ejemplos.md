---
layout: section
---

# Ejemplos desarrollados
El mapa sobre problemas reales

---
layout: center
---

## Los Cuatro Casos

Entre estos cuatro problemas se cubren tres de los cuatro operadores de combinación:

<div class="flex flex-col gap-3 mt-3 max-w-5xl mx-auto text-left text-sm">

<div class="rounded px-4 py-3 border border-gray-700">
  <div class="grid gap-x-4 gap-y-2" style="grid-template-columns:1.6fr 1fr 0.7fr 1.4fr">
    <div class="opacity-50">Problema</div><div class="opacity-50">Estado (A)</div><div class="opacity-50">D</div><div class="opacity-50">Qué lo hace distinto</div>
    <div>70 · Climbing Stairs</div><div class="font-mono">n escalones</div><div class="font-mono font-bold">suma</div><div class="opacity-80">cuenta, no optimiza</div>
    <div>746 · Min Cost Climbing Stairs</div><div class="font-mono">escalón s</div><div class="font-mono font-bold">min</div><div class="opacity-80">el costo se acumula en el camino</div>
    <div>198 · House Robber</div><div class="font-mono">casa curr</div><div class="font-mono font-bold">max</div><div class="opacity-80">una decisión restringe a la siguiente</div>
    <div>322 · Coin Change</div><div class="font-mono">monto restante</div><div class="font-mono font-bold">min</div><div class="opacity-80">n decisiones, no 2; y hay casos imposibles</div>
  </div>
</div>

<div class="rounded px-4 py-3" style="background:#F27F3D;color:#000">
  Los cuatro comparten la misma estructura de solución: caso base → consulta al memo → una llamada por decisión → combinar → guardar. Lo único que cambia son las respuestas a A, B, C y D.
</div>

</div>

---
layout: center
---

## 70 · Climbing Stairs — combinación por `suma`

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
fn compute(n: i32, memo: &mut Memo) -> i32 {
    if n < 3 {
        return n;      // 1 escalón → 1 forma
    }                  // 2 escalones → 2 formas

    if let Some(&ans) = memo.get(&n) {
        return ans;
    }

    let n1 = compute(n - 1, memo);   // subir 1
    let n2 = compute(n - 2, memo);   // subir 2

    memo.insert(n, n1 + n2);
    n1 + n2
}
```

  </div>

<div class="rounded px-4 py-3 border border-gray-700 text-sm space-y-2">

<div class="grid gap-x-3 gap-y-1" style="grid-template-columns:auto 1fr">
  <div class="font-bold text-[#F26244]">A</div><div>escalones que faltan por subir: <code>n</code></div>
  <div class="font-bold text-[#F26244]">B</div><div>subir 1 → <code>n-1</code> · subir 2 → <code>n-2</code></div>
  <div class="font-bold text-[#F26244]">C</div><div>no optimiza: <b>cuenta formas</b></div>
  <div class="font-bold text-[#F26244]">D</div><div><b><code>n1 + n2</code></b></div>
</div>

Las formas que empiezan subiendo 1 escalón y las que empiezan subiendo 2 son **conjuntos disjuntos**: ninguna se repite entre ambos grupos, así que el total es su suma.

<div class="opacity-70">Es la recurrencia de Fibonacci, llegando por el lado del conteo.</div>

</div>

</div>

---
layout: center
---

## 746 · Min Cost Climbing Stairs — combinación por `min`

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
fn compute(s: i32, c: i32, cost: &[i32],
           memo: &mut HashMap<i32, i32>) -> i32 {
    if fuera(s, cost) {
        return 0;      // ya salí de la escalera
    }

    if let Some(&ans) = memo.get(&s) {
        return ans;
    }

    let ans = c + min(
        compute(s + 1, costo_de(s + 1), cost, memo),
        compute(s + 2, costo_de(s + 2), cost, memo),
    );

    memo.insert(s, ans);
    ans
}
```

  </div>

<div class="rounded px-4 py-3 border border-gray-700 text-sm space-y-2">

<div class="grid gap-x-3 gap-y-1" style="grid-template-columns:auto 1fr">
  <div class="font-bold text-[#F26244]">A</div><div>escalón actual <code>s</code> (arranca en <code>-1</code>: el piso)</div>
  <div class="font-bold text-[#F26244]">B</div><div>avanzar 1 → <code>s+1</code> · avanzar 2 → <code>s+2</code></div>
  <div class="font-bold text-[#F26244]">C</div><div>minimizar el costo total</div>
  <div class="font-bold text-[#F26244]">D</div><div><b><code>c + min(...)</code></b></div>
</div>

El costo del escalón actual (`c`) se **suma fuera** del `min`: se paga siempre, sin importar qué rama se elija. El `min` solo decide el resto del camino.

</div>

</div>

---
layout: center
---

## 746 · Una Sutileza del Estado

<div class="important-note">

La firma recibe **dos** parámetros que cambian (`s` y `c`), pero el memo se indexa **solo por `s`**. ¿Es un error?

</div>

<br>

No, y vale la pena entender por qué: `c` es el costo de pisar el escalón `s`, o sea **está determinado por `s`** (`c == cost[s]`). No es información independiente: es un dato derivable que se va pasando por conveniencia para no volver a leerlo del arreglo.

<div class="important-note">

**La regla general:** en la clave del memo va lo que hace *genuinamente* distinto a un estado. Un parámetro derivable del resto puede omitirse. Pero si `c` pudiera valer algo distinto para el mismo `s`, la clave estaría incompleta y el memo devolvería respuestas equivocadas.

Ante la duda, la prueba es: *¿dos llamadas con la misma clave pueden necesitar resultados diferentes?*

</div>

---
layout: center
---

## 198 · House Robber — combinación por `max`

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
fn solve(curr: usize, h: &[i32],
         memo: &mut Memo) -> i32 {
    if curr >= h.len() {
        return 0;   // no quedan casas
    }

    if let Some(&ans) = memo.get(&curr) {
        return ans;
    }

    // robar: obliga a saltar la siguiente
    let steal = h[curr]
        + solve(curr + 2, h, memo);
    // saltar: la siguiente queda libre
    let skip = solve(curr + 1, h, memo);

    let ans = max(steal, skip);
    memo.insert(curr, ans);
    ans
}
```

  </div>

<div class="rounded px-4 py-3 border border-gray-700 text-sm space-y-2">

<div class="grid gap-x-3 gap-y-1" style="grid-template-columns:auto 1fr">
  <div class="font-bold text-[#F26244]">A</div><div>casa en la que estoy: <code>curr</code></div>
  <div class="font-bold text-[#F26244]">B</div><div>robar → <code>curr+2</code> · saltar → <code>curr+1</code></div>
  <div class="font-bold text-[#F26244]">C</div><div>maximizar dinero</div>
  <div class="font-bold text-[#F26244]">D</div><div><b><code>max(steal, skip)</code></b></div>
</div>

Lo característico: **la decisión restringe la siguiente**. Robar no avanza un paso, avanza dos — la restricción del problema (no robar casas adyacentes) queda codificada en el salto, no en una condición aparte.

</div>

</div>

---
layout: center
---

## 322 · Coin Change — `min` con `n` decisiones

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
fn solve(mnt: i32, c: &[i32],
         memo: &mut Memo) -> Option<i32> {
    // rama inválida
    if mnt < 0  { return None; }
    // monto cubierto
    if mnt == 0 { return Some(0); }

    if let Some(&ans) = memo.get(&mnt) {
        return ans;
    }

    let mut ans = None;
    for &val in c.iter() {  // 1 por moneda
        let rem = mnt - val;
        if rem >= 0
            && let Some(q) = solve(rem, c, memo)
        {
            ans = match ans {
                Some(cur) => Some(min(cur, 1 + q)),
                None => Some(1 + q),
            };
        }
    }

    memo.insert(mnt, ans);
    ans
}
```

  </div>

<div class="rounded px-4 py-3 border border-gray-700 text-sm space-y-2">

<div class="grid gap-x-3 gap-y-1" style="grid-template-columns:auto 1fr">
  <div class="font-bold text-[#F26244]">A</div><div>monto restante: <code>mnt</code></div>
  <div class="font-bold text-[#F26244]">B</div><div><b>n decisiones</b>: restar cada <code>coins[i]</code></div>
  <div class="font-bold text-[#F26244]">C</div><div>minimizar cantidad de monedas</div>
  <div class="font-bold text-[#F26244]">D</div><div><b><code>min</code></b> sobre todas las ramas, con <code>1 + qty</code></div>
  <div class="font-bold text-[#F26244]">E</div><div><code>None</code> = monto inalcanzable</div>
</div>

Las decisiones no son un par fijo: son un **bucle**. Cuando B da una cantidad variable de opciones, el operador de D se aplica acumulando dentro del ciclo.

</div>

</div>

---
layout: center
---

## 322 · Por qué `Option` y no `i32::MAX`

<div class="grid grid-cols-2 gap-y-4 gap-x-8 items-center mt-4">

  <div class="[&_pre]:my-0">
    <span class="text-xs text-red-500 font-bold block mb-1">Con ∞ como centinela</span>

```rust
let mut ans = i32::MAX;
// ...
ans = min(ans, 1 + qty);
//              ^^^^^^^
// si qty == i32::MAX, esto desborda
```

  </div>

  <div class="[&_pre]:my-0">
    <span class="text-xs text-green-500 font-bold block mb-1">Con Option</span>

```rust
let mut ans = None;
// ...
if let Some(qty) = solve(rem, ...) {
    // solo se suma cuando hay
    // un resultado válido
}
```

  </div>

</div>

<div class="important-note">

El problema del centinela numérico es que **participa en la aritmética**: `1 + i32::MAX` desborda antes de que el `min` alcance a descartarlo. `Option` separa el caso imposible del dominio de los valores, y el compilador obliga a desempacarlo antes de operar.

</div>

---
layout: center
---

## Resumen · el Mapa Completo

<div class="flex flex-col gap-3 mt-3 max-w-5xl mx-auto text-left text-sm">

<div class="rounded px-4 py-3 border border-gray-700">
  <div class="grid gap-x-4 gap-y-2" style="grid-template-columns:auto 1.5fr 1.6fr">
    <div class="opacity-50">#</div><div class="opacity-50">Pregunta</div><div class="opacity-50">Se traduce en</div>
    <div class="font-bold text-[#F26244]">A</div><div>¿Cuál es mi estado?</div><div class="opacity-80">los parámetros y la clave del memo</div>
    <div class="font-bold text-[#F26244]">B</div><div>¿Qué decisiones puedo tomar?</div><div class="opacity-80">una llamada recursiva por opción (o un bucle)</div>
    <div class="font-bold text-[#F26244]">C</div><div>¿Qué optimizo?</div><div class="opacity-80">el significado del valor devuelto</div>
    <div class="font-bold text-[#F26244]">D</div><div>¿Cómo combino las decisiones?</div><div class="opacity-80"><code>max</code> · <code>min</code> · <code>suma</code> · <code>OR</code></div>
    <div class="font-bold text-[#F26244]">E</div><div>¿Qué represento si es imposible?</div><div class="opacity-80">el valor neutro del operador de D</div>
  </div>
</div>

<div class="rounded px-4 py-3" style="background:#F27F3D;color:#000">
  <b>Dos casos pendientes de desarrollar:</b> un estado de dos dimensiones <code style="background:#F29441;color:#000">(i,j)</code> con decisiones "derecha o abajo" (62 · Unique Paths), y una combinación por <code style="background:#F29441;color:#000">OR</code> para decidir si existe solución (139 · Word Break).
</div>

</div>
