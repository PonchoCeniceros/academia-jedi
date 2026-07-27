---
layout: section
---

# Ownership

---
layout: center
---

1. Todos los programas tienen que gestionar la forma en que utilizan la memoria de la computadora mientras se ejecutan.
* Algunos lenguajes tienen un recolector de basura (*garbage collection*) que busca regularmente la memoria que ya no se utiliza a medida que el programa corre.
* En otros lenguajes, el programador debe reservar y liberar la memoria de forma explícita.

2. Rust utiliza un tercer enfoque: la memoria se gestiona a través de un conjunto de reglas que el compilador verifica. Si se viola cualquiera de estas reglas, el programa no compilará.

---
layout: center
---

El sistema de **Ownership** es un conjunto de reglas que gobiernan cómo un programa en Rust gestiona la memoria:

<div class="important-note">

1. Cada valor tiene un propietario (*owner*).
2. Solo puede haber un propietario a la vez.
3. Cuando el propietario sale del ámbito (*scope*), el valor se destruye (*dropped*).

</div>

---
layout: center
---

El *Ownership* (propiedad) define quién es responsable de la información en memoria, lo que naturalmente conlleva la capacidad de transferir esta responsabilidad.
La propiedad de la información se transfiere a través de:

* **Asociaciones** (ej. `y = x;`)
* **Asignaciones** (ej. `let y = x;`)
* **Paso de parámetros** (enviar variables a una función)
* **Retornos** (devolver valores desde una función)

---
layout: center
---

El comportamiento de los datos ante estos fenómenos variará dependiendo de dónde se almacene la información:

* **Si el dato vive en el Stack:** El valor se **duplica automáticamente**. Ambas variables se quedan con una copia independiente y siguen vivas.
* **Si el dato vive en el Heap:** Se realiza una **Transferencia (Move)**. La propiedad se mueve al nuevo destino y la variable original se destruye de inmediato.

---
layout: center
---

## Asociaciones y Asignaciones

<div class="grid grid-cols-2 gap-y-6 gap-x-8 items-center mt-4">

  <div class="[&_pre]:my-0">

  ```rust
  let x = 10;
  let y = x;
  ```

  </div>

  <div class="flex justify-center">
    <img src="/images/owner/a.png" class="h-32 object-cover" />
  </div>

  <div class="[&_pre]:my-0">

  ```rust
  let x = String::from("hola");
  let y = x;
  ```

  </div>

  <div class="flex justify-center">
    <img src="/images/owner/b.png" class="h-32 object-cover" />
  </div>

</div>

<br>

> En Rust, las "asociaciones" tras una declaración inicial y las "asignaciones" propiamente dichas a variables mutables siguen exactamente la misma semántica de copia o movimiento).


---
layout: center
---

## Paso de Parámetros

<div class="grid grid-cols-2 gap-y-6 gap-x-8 items-center mt-4">

  <div class="[&_pre]:my-0">

  ```rust
  fn foo(x: i32) {
    // ...
  }

  fn main() {
      let x = 27;
      foo(x);
      println!("{x}");
  }
  ```

  </div>

  <div class="flex justify-center">
    <img src="/images/owner/c.png" class="h-42 object-cover" />
  </div>

  <div class="[&_pre]:my-0">

  ```rust
  fn foo(x: String) {
    // ...
  }

  fn main() {
      let x = String::from("hola");
      foo(x);
      println!("{x}");
  }
  ```

  </div>

  <div class="flex justify-center">
    <img src="/images/owner/d.png" class="h-42 object-cover" />
  </div>

</div>

---
layout: center
---

## Retornos

<div class="grid grid-cols-2 gap-y-6 gap-x-8 items-center mt-4">

  <div class="[&_pre]:my-0">

  ```rust
  fn foo(x: i32) {
    x
  }

  fn main() {
      let x = 27;
      let y = foo(x);
      println!("{x}");
  }
  ```

  </div>

  <div class="flex justify-center">
    <img src="/images/owner/e.png" class="h-42 object-cover" />
  </div>

  <div class="[&_pre]:my-0">

  ```rust
  fn foo(x: String) {
    x
  }

  fn main() {
      let x = String::from("hola");
      let y = foo(x);
      println!("{y}");
  }
  ```

  </div>

  <div class="flex justify-center">
    <img src="/images/owner/f.png" class="h-42 object-cover" />
  </div>

</div>

---
layout: center
---

## Structs

<div class="grid grid-cols-2 gap-x-6 mt-4 w-full text-left">
  <div>

  ```rust
struct User {
    name: String,
    age: u8,
}


fn main() {
    let u = User {
        name: String::from("Giovanny"),
        age: 25,
    };

    let v = u;
}
```

  </div>
  <div class="flex justify-center">
    <img src="/images/owner/g.png" class="h-64 object-cover" />
  </div>
</div>

---
layout: center
---

## El Costo del Ownership Estricto

El sistema de *ownership* garantiza seguridad, pero introduce una limitación práctica: **pasar datos del Heap a una función los consume**.
Devolver el valor en cada función para recuperarlo es tedioso y no escala.
Rust resuelve esto con un mecanismo complementario: el **Borrowing**.

