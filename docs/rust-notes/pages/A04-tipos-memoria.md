---
layout: section
---

# Tipos de Memoria

---
layout: center
---

## El Stack
El Stack es una estructura de datos de acceso ultra rápido. Funciona bajo la regla de que lo último en llegar es lo primero en salir. El compilador solo permite guardar aquí datos cuyo tamaño exacto se conoce desde el momento en que se escribe el código.

<br>

* **¿Cómo funciona?**: Al llamar a una función, sus variables se "apilan" una arriba de otra en memoria. Al terminar la función, todo ese bloque se limpia instantáneamente.

<br>

* **Tipos que viven aquí**: Los Literales numéricos y booleanos, caracteres individuales (`char`), y las colecciones de tamaño fijo (Arreglos `[T; N]` y Tuplas).

---
layout: center
---

* **Comportamiento**: Cuando una variable almacena un dato que vive completamente en el Stack, Rust permite que su valor se **duplique automáticamente** al asignarla a otra variable. Al hacer `let y = x;`, Rust clona los bits en la memoria de forma instantánea y transparente.

<br>

* **Resultado**: Ambas variables siguen vivas y son independientes. Esto ocurre porque duplicar datos pequeños y fijos en el Stack es sumamente barato para la computadora.

```rust
let x = 42;    // Vive en el Stack
let y = x;     // Se duplica automáticamente
println!("x: {}, y: {}", x, y); // ¡Ambos se pueden usar!
```

---
layout: center
---

## El Heap
El Heap se utiliza para datos más complejos cuyo tamaño es dinámico, desconocido al compilar, o que necesitan crecer y cambiar durante la ejecución del programa.

<br>

* **¿Cómo funciona?**: El sistema operativo busca un espacio libre en el Heap, guarda los datos ahí y genera un **apuntador** (una variable con la dirección de esa ubicación). El apuntador (que mide siempre lo mismo) se guarda en el Stack para acceso rápido, pero el contenido real está en el Heap.

<br>

* **Tipos que viven aquí**: Estructuras dinámicas como `String` o listas que pueden crecer.

---
layout: center
---

* **Comportamiento**: A diferencia del Stack, los datos que se guardan en el Heap **no se duplican de forma automática**. Rust implementa una regla estricta de propiedad para evitar accidentes en la memoria. Al asignar una variable del Heap a otra (ej. `let s2 = s1;`), Rust no duplica el contenido del Heap; solo copia la "tarjeta/puntero" en el Stack y **destruye** la variable original (`s1`).

<br>

* **Resultado**: La propiedad del dato se **mueve** a la nueva variable. La variable vieja queda totalmente inválida para proteger el programa.

```rust
let s1 = String::from("hola"); // Datos en el Heap
let s2 = s1; // La propiedad se mueve a s2. s1 queda inválida.

// println!("{}", s1);
// ❌ ERROR de compilación: No puedes usar s1 porque su valor se movió.
```

