---
layout: section
---

# Aplicación
El cheatsheet en tus propios trials

---
layout: center
---

## Aplicación · Move + `.clone()` — Reverse Linked List (206)

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
// reverse_list CONSUME la lista
pub fn reverse_list(head: Node) -> Node {
    /* ... */
}

// en el test, para no perder la entrada:
let head = Solution::reverse_list(input.clone());
let ans  = Solution::get_vec(head);
```

  </div>

<div class="rounded px-4 py-3 border border-gray-700 text-sm space-y-2">

<div class="font-bold text-[#F26244]">Corrida mental</div>

`Node = Option<Box<ListNode>>` vive en el **heap** → pasarlo a `reverse_list` lo **mueve**.

Sin `.clone()`, `input` quedaría inutilizable después de esa línea. La copia conserva el original para el resto del test.

<div class="opacity-70">Cheatsheet: <b>Heap → Movimiento</b> · ¿usable? No, salvo <code style="background:#F29441;color:#000">.clone()</code></div>

</div>

</div>

---
layout: center
---

## Aplicación · `&mut T` exclusiva — Remove Duplicates (26)

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    nums.retain(|&x| set.insert(x));
    nums.len() as i32
}

let mut arr = vec![1, 1, 2];
Solution::remove_duplicates(&mut arr);
// arr quedó modificado in-place
```

  </div>

<div class="rounded px-4 py-3 border border-gray-700 text-sm space-y-2">

<div class="font-bold text-[#F26244]">Corrida mental</div>

`&mut Vec<i32>` es un préstamo **exclusivo**: mientras dura, nadie más puede leer ni escribir `arr`.

Permite mutar in-place con `retain` sin mover el `Vec` ni devolver uno nuevo.

<div class="opacity-70">Cheatsheet: <b>&mut T → una sola</b>, y sin <code style="background:#F29441;color:#000">&</code> activas</div>

</div>

</div>

---
layout: center
---

## Aplicación · `&T` compartida — Longest Common Prefix (14)

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
for word in strs.iter() {   // word: &String
    if len > word.len() {
        len = word.len();
    }
}
// strs sigue disponible: se recorre
// otra vez más abajo
```

  </div>

<div class="rounded px-4 py-3 border border-gray-700 text-sm space-y-2">

<div class="font-bold text-[#F26244]">Corrida mental</div>

`.iter()` entrega `&String`: préstamos **compartidos**, puedes tener muchos a la vez.

`strs` no se mueve — sigue usable tras el bucle (de hecho se vuelve a iterar después).

<div class="opacity-70">Cheatsheet: <b>&T → muchas lecturas</b> simultáneas</div>

</div>

</div>

---
layout: center
---

## Aplicación · Copy de escalares — Container With Most Water (11)

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
let mut i = 0_usize;
let mut j = height.len() - 1;

while i < j {
    let a = min(height[i], height[j]); // i32
    if height[i] < height[j] { i += 1; }
    else { j -= 1; }
}
```

  </div>

<div class="rounded px-4 py-3 border border-gray-700 text-sm space-y-2">

<div class="font-bold text-[#F26244]">Corrida mental</div>

`i`, `j` (`usize`) y los `height[i]` (`i32`) viven en el **stack** → se **copian**.

Por eso puedes reasignar `i += 1` y comparar valores libremente, sin mover ni prestar nada.

<div class="opacity-70">Cheatsheet: <b>Stack → Copia</b> · ambos siguen vivos</div>

</div>

</div>

---
layout: center
---

## Aplicación · `&mut` "puntero láser" — Reverse Linked List (206)

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left items-center">
  <div>

```rust
let mut head: Node = None;
let mut iter: &mut Node = &mut head;

for &val in values.iter() {
    *iter = Some(Box::new(
        ListNode { val, next: None }
    ));
    iter = &mut iter.as_mut().unwrap().next;
}
head
```

  </div>

<div class="rounded px-4 py-3 border border-gray-700 text-sm space-y-2">

<div class="font-bold text-[#F26244]">Corrida mental</div>

`iter: &mut Node` es un préstamo **exclusivo** que se va reapuntando al `next` de cada nodo.

Solo puede existir ese único `&mut` sobre la cadena a la vez — el borrow checker lo garantiza en compilación.

<div class="opacity-70">Cheatsheet: <b>&mut T → una sola</b> · enlaza con Listas Enlazadas</div>

</div>

</div>
