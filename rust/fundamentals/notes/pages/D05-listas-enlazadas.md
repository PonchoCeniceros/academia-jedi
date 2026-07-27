---
layout: section
---

# Listas Enlazadas

---
layout: center
---

## El problema: tipos recursivos

1. Una lista enlazada es una secuencia de nodos donde cada nodo apunta al siguiente. La forma más directa de modelarlo sería:

```rust
struct Nodo {
    valor: i32,
    siguiente: Nodo,
}
```

2. Este código no compila. Rust necesita conocer el tamaño exacto de cada tipo en tiempo de compilación, pero `Nodo` contiene otro `Nodo`, que contiene otro `Nodo`... un tamaño infinito.

---
layout: center
---

## `Box<T>`: Indirección al Heap

`Box<T>` es un puntero inteligente que guarda un valor en el heap y deja en el stack solo un puntero de tamaño fijo hacia él.

```rust
struct Nodo {
    valor: i32,
    siguiente: Box<Nodo>,
}
```

<div class="important-note">

Al envolver el campo recursivo en un `Box`, el compilador ya no necesita conocer el tamaño de `Nodo` completo — solo el tamaño de un puntero.

</div>

Pero sigue faltando algo: ¿cómo se representa el final de la lista, donde no hay "siguiente" nodo?

---
layout: center
---

## `Option<Box<Nodo>>`: Representar el Final

La lista termina cuando un nodo no tiene siguiente. Esa ausencia se modela con `Option`, no con un valor especial como `null` (que Rust no tiene):

```rust
struct Nodo {
    valor: i32,
    siguiente: Option<Box<Nodo>>,
}

fn main() {
    let ultimo = Nodo { valor: 3, siguiente: None };
    let segundo = Nodo { valor: 2, siguiente: Some(Box::new(ultimo)) };
    let primero = Nodo { valor: 1, siguiente: Some(Box::new(segundo)) };
}
```

* `None` marca el final de la lista.
* `Some(Box::new(nodo))` envuelve el siguiente nodo, ya en el heap.

---
layout: center
---

## Recorrer sin robar la propiedad

Para leer la lista sin consumirla, se recorre con una **referencia** que avanza nodo a nodo — el mismo patrón de *borrowing* visto en Borrowing y Lifetimes:

```rust
fn imprimir(nodo: &Option<Box<Nodo>>) {
    let mut actual = nodo;
    while let Some(n) = actual {
        println!("{}", n.valor);
        actual = &n.siguiente;
    }
}
```

<br>

* `actual` es una referencia (`&Option<Box<Nodo>>`), no el dueño de los nodos.
* En cada vuelta, `actual` avanza al campo `siguiente` del nodo actual — nunca se mueve el nodo, solo el "dedo" que lo señala.

---
layout: center
---

## Mover en vez de prestar: `Option::take()`

Cuando sí se necesita tomar posesión de un nodo (por ejemplo, para invertir la lista), prestar no basta — hay que mover el valor y dejar algo válido en su lugar.

```rust
fn quitar_primero(lista: &mut Option<Box<Nodo>>) -> Option<Box<Nodo>> {
    lista.take()
}
```

<div class="important-note">

`Option::take()` extrae el valor de una `Option`, dejando `None` en su lugar. Así se puede mover un nodo sin dejar el campo original en un estado inválido.

</div>

* Antes de `take()`: `lista` posee el nodo.
* Después de `take()`: `lista` queda en `None`, y el nodo pasa a quien recibió el valor devuelto.
