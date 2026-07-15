---
layout: section
---

# Árboles Binarios

---
layout: center
---

## De listas a árboles

1. Una lista enlazada es estrictamente lineal: cada nodo tiene un único siguiente. Un **árbol binario** relaja esa restricción — cada nodo puede tener hasta dos hijos.

```rust
struct Nodo {
    valor: i32,
    izquierda: Option<Box<Nodo>>,
    derecha: Option<Box<Nodo>>,
}
```

2. Con `Box`, cada nodo sigue siendo dueño exclusivo de sus hijos, igual que en la lista enlazada. Esto funciona mientras el árbol se recorra y modifique desde un único punto de entrada.

---
layout: center
---

## Cuando un dueño no basta

Algunos escenarios rompen el supuesto de "un solo dueño":

* Compartir el mismo nodo desde **más de un lugar** (por ejemplo, una estructura que reorganiza o balancea sus propios subárboles).
* Necesitar **mutar** un nodo a través de una referencia compartida, algo que el borrow checker prohíbe en tiempo de compilación con referencias normales.

<div class="important-note">

`Box<T>` resuelve "¿quién es dueño?" con una respuesta única. Cuando la respuesta necesita ser "varios, y a veces hay que modificarlo", Rust ofrece `Rc<RefCell<T>>`.

</div>

---
layout: center
---

## `Rc<T>`: Conteo de Referencias

`Rc<T>` (*Reference Counted*) permite que varios dueños compartan el mismo valor en el heap, llevando la cuenta de cuántas referencias existen.

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a); // no copia el 5, incrementa el contador

println!("referencias: {}", Rc::strong_count(&a)); // 2
```

* `.clone()` sobre un `Rc` es barato: solo incrementa un contador, no duplica el dato.
* El valor se libera únicamente cuando el contador llega a cero (el último dueño sale de scope).
* `Rc<T>` por sí solo **no permite mutar** el valor compartido — todas las referencias son de solo lectura.

---
layout: center
---

## `RefCell<T>`: Mutabilidad en Tiempo de Ejecución

`RefCell<T>` mueve la verificación de las reglas de *borrowing* (una referencia mutable **o** varias inmutables) del tiempo de compilación al tiempo de ejecución.

```rust
use std::cell::RefCell;

let celda = RefCell::new(5);

*celda.borrow_mut() += 1;
println!("{}", celda.borrow()); // 6
```

<div class="important-note">

Si en tiempo de ejecución se viola la regla (dos préstamos mutables simultáneos, por ejemplo), el programa hace *panic* en vez de no compilar.

</div>

---
layout: center
---

## `Rc<RefCell<T>>`: Compartir y Mutar

Combinando ambos, un árbol puede tener nodos con **múltiples dueños** que además pueden **mutarse**:

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Nodo {
    valor: i32,
    izquierda: Option<Rc<RefCell<Nodo>>>,
    derecha: Option<Rc<RefCell<Nodo>>>,
}
```

* `Rc` responde "¿quién puede acceder a este nodo?" → varios.
* `RefCell` responde "¿puede alguien modificarlo?" → sí, verificado en ejecución.

---
layout: center
---

## Recorridos: Preorden, Inorden, Postorden

Los tres recorridos clásicos visitan los mismos tres elementos (nodo, izquierda, derecha), solo cambia el orden:

<div class="grid grid-cols-3 gap-x-6 mt-4 text-sm">

<div class="[&_pre]:my-0">

**Preorden**

```rust
fn preorden(n: &Nodo) {
    visitar(n);
    if let Some(i) = &n.izquierda {
        preorden(i);
    }
    if let Some(d) = &n.derecha {
        preorden(d);
    }
}
```

nodo → izq → der

</div>

<div class="[&_pre]:my-0">

**Inorden**

```rust
fn inorden(n: &Nodo) {
    if let Some(i) = &n.izquierda {
        inorden(i);
    }
    visitar(n);
    if let Some(d) = &n.derecha {
        inorden(d);
    }
}
```

izq → nodo → der

</div>

<div class="[&_pre]:my-0">

**Postorden**

```rust
fn postorden(n: &Nodo) {
    if let Some(i) = &n.izquierda {
        postorden(i);
    }
    if let Some(d) = &n.derecha {
        postorden(d);
    }
    visitar(n);
}
```

izq → der → nodo

</div>

</div>

---
layout: center
---

## Cierre: ¿Cuál Indirección Usar?

<div class="important-note">

* **`Box<T>`** — un único dueño, estructura estrictamente jerárquica (listas, árboles simples sin reestructuración).
* **`Rc<T>`** — múltiples dueños de solo lectura.
* **`Rc<RefCell<T>>`** — múltiples dueños que además necesitan mutar el valor compartido.

</div>

La elección no es estética: refleja cuántos dueños tiene realmente el dato y si necesita cambiar después de compartirse.
