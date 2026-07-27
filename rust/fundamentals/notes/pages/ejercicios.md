---
layout: section
---

# Ejercicios propuestos

---
layout: default
---

## Ejercicio 1: La keyword perdida
<br/>

```rust
fn main() {
    x = 42;
    println!("x es: {}", x);
}
```

🤔 El código de arriba falta algo esencial. ¿Qué keyword de Rust necesitas para que `x` exista dentro del `main`?

---
layout: default
---

## Respuesta 1: La keyword perdida
<br/>

Rust exige declarar las variables con `let` antes de usarlas. Sin `let`, el compilador no sabe que `x` es una nueva variable.

```rust
fn main() {
    let x = 42;
    println!("x es: {}", x);
}
```

---
layout: default
---

## Ejercicio 2: El identificador rebelde
<br/>

```rust
fn main() {
    let miVariable = String::from("hola");
    let 2do_intento = "mundo";
    println!("{} {}", miVariable, 2do_intento);
}
```

❌ Este código tiene dos errores de convención y sintaxis. ¿Cuáles son las reglas de identificadores que está violando?

---
layout: default
---

## Respuesta 2: El identificador rebelde
<br/>

Dos problemas:
1. Rust usa **snake_case** para variables: `mi_variable`, no `miVariable` (camelCase).
2. Los identificadores **no pueden empezar con dígitos**: `2do_intento` es inválido.

```rust
fn main() {
    let mi_variable = String::from("hola");
    let segundo_intento = "mundo";
    println!("{} {}", mi_variable, segundo_intento);
}
```

---
layout: default
---

## Ejercicio 3: La prioridad oculta
<br/>

```rust
fn main() {
    let resultado = 2 + 3 * 4;
    let esperado = (2 + 3) * 4;
    println!("resultado: {}, esperado: {}", resultado, esperado);
}
```

🤔 ¿Son `resultado` y `esperado` iguales o diferentes? ¿Qué operador tiene mayor prioridad en Rust?

---
layout: default
---

## Respuesta 3: La prioridad oculta
<br/>

Son **diferentes**. `resultado` es `14` (3 * 4 = 12, + 2 = 14) porque `*` tiene prioridad sobre `+`. `esperado` es `20` porque los paréntesis `(2 + 3)` fuerzan la suma primero.

```rust
fn main() {
    let resultado = 2 + 3 * 4;   // 14
    let esperado = (2 + 3) * 4;  // 20
    println!("resultado: {}, esperado: {}", resultado, esperado);
}
```

---
layout: default
---

## Ejercicio 4: El camino incompleto
<br/>

```rust
fn main() {
    let numero = 7;
    if numero > 10 {
        println!("Grande");
    }
}
```

🤔 El código compila pero no imprime nada. ¿Qué estructura de control podrías agregar para manejar el caso contrario y mostrar "Pequeño"?

---
layout: default
---

## Respuesta 4: El camino incompleto
<br/>

Necesitas un `else` para cubrir el caso cuando la condición es falsa:

```rust
fn main() {
    let numero = 7;
    if numero > 10 {
        println!("Grande");
    } else {
        println!("Pequeño");
    }
}
```

---
layout: default
---

## Ejercicio 5: El match incompleto
<br/>

```rust
fn main() {
    let opcion = 3;
    match opcion {
        1 => println!("Uno"),
        2 => println!("Dos"),
    }
}
```

❌ Este código no compila. ¿Qué regla de `match` está violando y cómo se soluciona?

---
layout: default
---

## Respuesta 5: El match incompleto
<br/>

`match` en Rust debe ser **exhaustivo** (cubrir todos los valores posibles). Aquí solo se cubren `1` y `2`, pero `opcion` es `i32` y puede tener cualquier valor. Se soluciona con un brazo `_` (catch-all):

```rust
fn main() {
    let opcion = 3;
    match opcion {
        1 => println!("Uno"),
        2 => println!("Dos"),
        _ => println!("Otro"),
    }
}
```

---
layout: default
---

## Ejercicio 6: El bucle que no termina
<br/>

```rust
fn main() {
    let mut contador = 0;
    loop {
        contador += 1;
        if contador == 5 {
            // ¿Qué va aquí?
        }
    }
    println!("Contador final: {}", contador);
}
```

❌ El `loop` es infinito y nunca se llega al `println!`. ¿Qué palabra clave falta para romper el ciclo cuando `contador` llegue a 5?

---
layout: default
---

## Respuesta 6: El bucle que no termina
<br/>

La palabra clave `break` detiene el `loop`:

```rust
fn main() {
    let mut contador = 0;
    loop {
        contador += 1;
        if contador == 5 {
            break;
        }
    }
    println!("Contador final: {}", contador);
}
```

---
layout: default
---

## Ejercicio 7: El tipo invisible
<br/>

```rust
fn main() {
    let x = 42;
    let y = 3.14;
    let z = true;
}
```

🤔 ¿Qué tipos infiere Rust para `x`, `y` y `z`? ¿Cómo podrías verificarlo en código?

---
layout: default
---

## Respuesta 7: El tipo invisible
<br/>

Rust infiere:
- `x`: `i32` (entero por defecto)
- `y`: `f64` (flotante por defecto)
- `z`: `bool`

Puedes verificarlo pidiendo el tamaño con `std::mem::size_of_val` o viendo el error si forzas un tipo incompatible.

---
layout: default
---

## Ejercicio 8: El índice prohibido
<br/>

```rust
fn main() {
    let arr = [10, 20, 30, 40];
    let primero = arr[0];
    let ultimo = arr[3];
    let fuera = arr[10];
    println!("{} {}", primero, ultimo);
}
```

❌ ¿Qué línea causa un error y por qué? ¿El error ocurre en compilación o en ejecución?

---
layout: default
---

## Respuesta 8: El índice prohibido
<br/>

`arr[10]` causa un error en **ejecución** (panic) porque el arreglo tiene 4 elementos (índices 0..3) y se está accediendo al índice 10, que está fuera de los límites. El compilador no puede saber en tiempo de compilación qué índice se usará (si fuera una variable), así que la verificación ocurre en runtime.

```rust
fn main() {
    let arr = [10, 20, 30, 40];
    let primero = arr[0];
    let ultimo = arr[3];
    // let fuera = arr[10]; // 💥 panic en runtime
    println!("{} {}", primero, ultimo);
}
```

---
layout: default
---

## Ejercicio 9: La tupla desestructurada
<br/>

```rust
fn main() {
    let persona = ("Ana", 30, true);
    // Extrae el nombre y la edad en variables separadas
    // sin usar persona.0, persona.1
}
```

🤔 ¿Qué característica de Rust permite extraer los valores de una tupla en variables individuales en una sola línea?

---
layout: default
---

## Respuesta 9: La tupla desestructurada
<br/>

La **desestructuración (destructuring)** permite desempaquetar una tupla en una sola línea:

```rust
fn main() {
    let persona = ("Ana", 30, true);
    let (nombre, edad, activo) = persona;
    println!("{} tiene {} años", nombre, edad);
}
```

---
layout: default
---

## Ejercicio 10: ¿Stack o Heap?
<br/>

```rust
fn main() {
    let a = 100;
    let b = "hola";
    let c = String::from("mundo");
    let d = [1, 2, 3];
    let e = vec![1, 2, 3];
}
```

🤔 ¿Cuáles de estas variables viven completamente en el Stack y cuáles almacenan datos en el Heap?

---
layout: default
---

## Respuesta 10: ¿Stack o Heap?
<br/>

En el Stack: `a` (`i32`), `b` (`&str` — el string vive en el binario), `d` (`[i32; 3]` — arreglo de tamaño fijo).

En el Heap: `c` (`String`), `e` (`Vec<i32>`). El puntero al heap está en el Stack, pero los datos reales están en el Heap.

---
layout: default
---

## Ejercicio 11: Copiar vs Mover
<br/>

```rust
fn main() {
    let x = 42;
    let y = x;
    println!("x: {}, y: {}", x, y); // ✅

    let s1 = String::from("hola");
    let s2 = s1;
    println!("s1: {}, s2: {}", s1, s2); // ❌
}
```

🤔 ¿Por qué el primer `println!` funciona y el segundo no, si la estructura de las asignaciones es idéntica?

---
layout: default
---

## Respuesta 11: Copiar vs Mover
<br/>

`i32` implementa el trait `Copy`: al hacer `let y = x`, se duplica el valor en el Stack y ambas variables son independientes.

`String` **no** implementa `Copy` porque almacena datos en el Heap. `let s2 = s1` **mueve** la propiedad: `s1` queda inválida para evitar el doble free.

```rust
let x = 42;       // i32 es Copy
let y = x;        // se copia, ambos viven

let s1 = String::from("hola"); // String no es Copy
let s2 = s1;      // se mueve, s1 muere
```

---
layout: default
---

## Ejercicio 12: El bloque que todo lo borra
<br/>

```rust
fn main() {
    let x = 10;
    {
        let y = 20;
        println!("Dentro: x={}, y={}", x, y);
    }
    println!("Fuera: x={}", x);
    println!("Fuera: y={}", y); // ❌
}
```

¿Por qué falla el último `println!`? ¿Qué regla de los delimitadores `{}` y del Stack explica este comportamiento?

---
layout: default
---

## Respuesta 12: El bloque que todo lo borra
<br/>

Las llaves `{}` crean un **ámbito (scope)**. Todo lo declarado dentro de ellas vive solo allí. Al cerrarse el bloque (la llave `}`), `y` se elimina del Stack. Fuera del bloque, `y` ya no existe. Esto es fundamental para la gestión de memoria en Rust: los recursos se liberan automáticamente al salir de su ámbito.

```rust
fn main() {
    let x = 10;
    {
        let y = 20;
        println!("Dentro: x={}, y={}", x, y);
    } // y se destruye aquí
    println!("Fuera: x={}", x);
    // println!("Fuera: y={}", y); // Error: y no existe
}
```

---
layout: default
---

## Ejercicio 13: El clon olvidado
<br/>

```rust
fn main() {
    let s1 = String::from("hola");
    let s2 = s1; 
    println!("{}, mundo!", s1); // ❌ ¿Por qué falla? ¿Cómo lo arreglas sin borrar la línea 3?
}
```

---
layout: default
---

## Respuesta 13: El clon olvidado
<br/>

`s1` se **mueve** (move) a `s2`. Después de esa asignación, `s1` ya no es válido
porque Rust transfiere la propiedad del dato en el Heap. El compilador prohíbe
usarlo para evitar el problema de "doble free".

Corrección: clonar antes de transferir.
```rust
let s2 = s1.clone();
```

---
layout: default
---

## Ejercicio 14: El agujero negro de las funciones
<br/>

```rust
fn tomar_propiedad(texto: String) {
    println!("{}", texto);
}

fn main() {
    let frase = String::from("Rust es genial");
    tomar_propiedad(frase);
    println!("{}", frase); // ❌ ¿Por qué 'frase' ya no existe aquí?
}
```

---
layout: default
---

## Respuesta 14: El agujero negro de las funciones
<br/>

Al pasar `frase` a `tomar_propiedad(frase)`, la propiedad del String se transfiere
al parámetro `texto`. Cuando la función termina y `texto` sale de alcance, el
String se destruye. En `main`, `frase` ya apunta a memoria liberada, por eso el
compilador lo prohíbe.

Corrección: pasar una referencia `&frase` o devolver la propiedad con `-> String`.

---
layout: default
---

## Ejercicio 15: Tipos primitivos en el Stack
<br/>

```rust
fn procesar_numero(n: i32) {
    println!("{}", n);
}

fn main() {
    let x = 42;
    procesar_numero(x);
    println!("{}", x); // 🤔 ¿Por qué este SI compila y el Ejercicio 2 NO?
}
```

---
layout: default
---

## Respuesta 15: Tipos primitivos en el Stack
<br/>

`i32` implementa el trait `Copy`. Los tipos `Copy` se **copian** bit a bit en el
Stack en lugar de moverse. Por eso `x` sigue siendo válido después de la llamada:
nunca se transfirió la propiedad, solo se duplicó el valor.

---
layout: default
---

## Ejercicio 16: El ciclo de devoluciones
<br/>

```rust
fn cambiar_dueno(s: String) -> String {
    s
}

fn main() {
    let s1 = String::from("datos");
    let s2 = cambiar_dueno(s1);
    // ¿Quién es el dueño de los bytes "datos" en esta línea? ¿s1 o s2?
}
```

---
layout: default
---

## Respuesta 16: El ciclo de devoluciones
<br/>

Al finalizar la línea `let s2 = cambiar_dueno(s1)`:

- `s1` ya no es el dueño (movió su propiedad al parámetro `s`).
- La función devuelve `s`, y ese valor se asigna a `s2`.
- **`s2` es el único dueño** de los bytes "datos".

---
layout: default
---

## Ejercicio 17: Mutabilidad y transferencia
<br/>

```rust
fn main() {
    let s1 = String::from("original");
    let mut s2 = s1; // ¿Es válido transferir la propiedad de algo inmutable a algo mutable?
    s2.push_str(" modificado");
    println!("{}", s2);
}
```

---
layout: default
---

## Respuesta 17: Mutabilidad y transferencia
<br/>

Sí, es válido. La mutabilidad es una propiedad del **binding** (la variable),
no del dato. Al hacer `let mut s2 = s1`, simplemente se transfiere la propiedad
a un binding que permite modificaciones. El String en sí no cambia; lo que cambia
es quién lo tiene y con qué permisos.

---
layout: default
---

## Ejercicio 18: Ownership dentro de un bucle
<br/>

```rust
fn main() {
    let lista = vec![String::from("A"), String::from("B")];
    for elemento in lista {
        println!("{}", elemento);
    }
    // ❌ Si intentas usar 'lista' aquí, fallará. ¿A dónde se movió la lista completa?
}
```

---
layout: default
---

## Respuesta 18: Ownership dentro de un bucle
<br/>

`for elemento in lista` usa `IntoIterator`, que **consume** el vector completo.
La propiedad de `lista` se transfiere al iterador al inicio del `for`. Cada
iteración extrae y toma propiedad de un elemento. Al terminar el bucle, el vector
original ya fue descompuesto; no existe más.

Corrección: iterar por referencia con `for elemento in &lista`.

---
layout: default
---

## Ejercicio 19: Estructuras que reclaman propiedad
<br/>

```rust
struct Contenedor {
    contenido: String,
}

fn main() {
    let texto = String::from("Secreto");
    let c = Contenedor { contenido: texto };
    // 🤔 ¿Puedes seguir usando la variable 'texto' aquí abajo? ¿Por qué?
}
```

---
layout: default
---

## Respuesta 19: Estructuras que reclaman propiedad
<br/>

No. Cuando escribes `Contenedor { contenido: texto }`, el String que era de
`texto` se **mueve** dentro del struct. A partir de esa línea, `texto` ya no es
un binding válido. La propiedad vive ahora en `c.contenido`.

---
layout: default
---

## Ejercicio 20: El préstamo básico
<br/>

```rust
fn calcular_longitud(s: &String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("puente");
    let len = calcular_longitud(&s1); // Creamos una referencia
    println!("La longitud de '{}' es {}.", s1, len); // ✅ ¿Por qué esto sí es válido?
}
```

---
layout: default
---

## Respuesta 20: El préstamo básico
<br/>

Porque `&s1` crea una **referencia inmutable**: `calcular_longitud` toma
prestado el String sin reclamar su propiedad. Cuando la función termina, el
préstamo expira, y `s1` sigue siendo el dueño legítimo, disponible en el
`println!`.

---
layout: default
---

## Ejercicio 21: El lector que intentó escribir
<br/>

```rust
fn modificar(s: &String) {
    s.push_str(" extra"); // ❌ ¿Qué le falta a la firma para permitir esto?
}

fn main() {
    let mut s = String::from("hola");
    modificar(&s);
}
```

---
layout: default
---

## Respuesta 21: El lector que intentó escribir
<br/>

La firma debe usar una **referencia mutable** para permitir modificar el valor:

```rust
fn modificar(s: &mut String) {
    s.push_str(" extra");
}

fn main() {
    let mut s = String::from("hola");
    modificar(&mut s);
}
```

Dos cambios: `&mut String` en la firma y `&mut s` en la llamada. Además `s`
debe declararse `mut`.

---
layout: default
---

## Ejercicio 22: El doble escritor (Aliasing Mutable)
<br/>

```rust
fn main() {
    let mut s = String::from("dinámico");
    let r1 = &mut s;
    let r2 = &mut s; // ❌ ¿Por qué Rust prohíbe tener dos referencias mutables al mismo tiempo?
    println!("{}, {}", r1, r2);
}
```

---
layout: default
---

## Respuesta 22: El doble escritor (Aliasing Mutable)
<br/>

Rust prohíbe dos referencias mutables simultáneas al mismo dato para eliminar
las **carreras de datos** (data races). Si `r1` y `r2` pudieran modificar `s` a
la vez, cualquier suposición sobre el estado de la memoria sería inválida. La
regla es: un solo escritor **o** múltiples lectores, nunca ambos.

---
layout: default
---

## Ejercicio 23: El escritor tímido
<br/>

```rust
fn main() {
    let mut s = String::from("datos");
    let r1 = &s; // Lector
    let r2 = &s; // Lector
    let r3 = &mut s; // ❌ Escritor. ¿Por qué esto colapsa el universo de Rust?
    println!("{}, {}, {}", r1, r2, r3);
}
```

---
layout: default
---

## Respuesta 23: El escritor tímido
<br/>

El compilador aplica la regla de exclusión mutua: no puedes tener una referencia
mutable (`r3`) activa al mismo tiempo que referencias inmutables (`r1`, `r2`)
al mismo dato. Aquí las tres se usan en el mismo `println!`, por lo que sus
vidas se solapan. Si se permitiera, `r3` podría mover o invalidar la memoria
que `r1`/`r2` están leyendo.

---
layout: default
---

## Ejercicio 24: El ciclo de vida de una mirada (NLL)
<br/>

```rust
fn main() {
    let mut s = String::from("hola");
    let r1 = &s; 
    println!("{}", r1); // El lector r1 se usa aquí por última vez
    
    let r2 = &mut s; // 🤔 ¿Por qué este código SÍ compila a pesar del Ejercicio 11?
    r2.push_str(" mundo");
}
```

---
layout: default
---

## Respuesta 24: El ciclo de vida de una mirada (NLL)
<br/>

Gracias a **Non-Lexical Lifetimes (NLL)**, el compilador moderno calcula el
alcance real de cada referencia basándose en su **último uso**, no en las llaves
del bloque. `r1` se usa por última vez en su `println!`; después de esa línea,
su préstamo termina. Cuando se crea `r2 = &mut s`, ya no hay ninguna referencia
activa que compita, por lo que el código es válido.

---
layout: default
---

## Ejercicio 25: Referencias a porciones (Slices)
<br/>

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let slice = &v[0..3]; // Tomamos prestada una parte (Lectura)
    v.push(6); // ❌ Intentamos modificar el vector original. ¿Por qué da error?
    println!("{:?}", slice);
}
```

---
layout: default
---

## Respuesta 25: Referencias a porciones (Slices)
<br/>

`slice = &v[0..3]` establece un **préstamo inmutable** del vector. Mientras
`slice` esté vivo, el vector está "bloqueado" para lecturas. `v.push(6)` exige
un préstamo mutable, lo que viola la regla: no puedes combinar préstamos
mutables con inmutables activos sobre el mismo dato.

---
layout: default
---

## Ejercicio 26: Desreferenciación (Entrar al Heap)
<br/>

```rust
fn main() {
    let mut x = 10;
    let r = &mut x;
    // Modifica el valor interno de x a través de 'r' para que valga 20.
    // Pista: Usa el operador asterisco (*).
}
```

---
layout: default
---

## Respuesta 26: Desreferenciación
<br/>

```rust
fn main() {
    let mut x = 10;
    let r = &mut x;
    *r = 20; // Entra al Heap/Stack a través del puntero y cambia el valor
    println!("{}", x); // Imprime 20
}
```

El operador `*` **desreferencia** el puntero, permitiendo leer o escribir
el valor al que apunta.

---
layout: default
---

## Ejercicio 27: La referencia que apunta a la nada
<br/>

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x; 
    } // x muere aquí
    println!("r: {}", r); // ❌ ¿Cuál es el error conceptual de espacio-tiempo aquí?
}
```

---
layout: default
---

## Respuesta 27: La referencia que apunta a la nada
<br/>

Error conceptual: **dangling reference** (referencia colgante). `x` vive solo
dentro del bloque interno `{}`. Cuando ese bloque termina, `x` se destruye y su
memoria se libera. `r` apuntaría a una dirección de memoria inválida. Rust lo
detecta en tiempo de compilación y lo prohíbe.

---
layout: default
---

## Ejercicio 28: El dilema de la función simple
<br/>

```rust
// ❌ Arregla la firma de esta función usando anotaciones de lifetimes ('a)
fn devolver_uno(x: &str, y: &str) -> &str {
    x
}

fn main() {
    let a = "hola";
    let b = "mundo";
    let res = devolver_uno(a, b);
}
```

---
layout: default
---

## Respuesta 28: El dilema de la función simple
<br/>

El compilador no puede inferir de cuál parámetro proviene la referencia
devuelta. Debes anotarlo explícitamente:

```rust
fn devolver_uno<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}
```

Esto le dice al compilador: "la referencia que devuelvo vive al menos tanto
como el parámetro `x` (y `y`, dado que ambos usan `'a`)."

---
layout: default
---

## Ejercicio 29: El eslabón más débil
<br/>

```rust
fn elegir<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("cadena_larga");
    let res;
    {
        let s2 = String::from("corta");
        res = elegir(&s1, &s2);
    } // s2 muere aquí. 
    println!("{}", res); // ❌ ¿Por qué falla si el resultado claramente iba a ser s1?
}
```

---
layout: default
---

## Respuesta 29: El eslabón más débil
<br/>

El contrato `<'a>` le dice al compilador: "el resultado vive a lo sumo tanto
como **el más corto** de los dos parámetros". Aunque en tiempo de ejecución el
resultado siempre sería `s1`, el compilador no hace análisis de flujo de valores;
trabaja con el tipo y las anotaciones. Como `s2` muere antes de que `res` sea
usado, la garantía se rompe desde el punto de vista del sistema de tipos.

Solución: darle a `s2` el mismo o mayor alcance que `res`, o usar `'static`.

---
layout: default
---

## Ejercicio 30: Structs con pasajeros prestados
<br/>

```rust
// ❌ Corrige este struct para que acepte una referencia
struct Perfil {
    nombre: &str, 
}

fn main() {
    let nombre_usuario = String::from("Carlos");
    let p = Perfil { nombre: &nombre_usuario };
}
```

---
layout: default
---

## Respuesta 30: Structs con pasajeros prestados
<br/>

Un struct que contiene una referencia debe declarar un lifetime para que el
compilador sepa que el struct no puede sobrevivir al dato al que apunta:

```rust
struct Perfil<'a> {
    nombre: &'a str,
}
```

---
layout: default
---

## Ejercicio 31: Mezclando Lifetimes diferentes
<br/>

```rust
// Imagina que 'x' e 'y' no tienen nada que ver entre sí.
// Modifica la firma para que el retorno dependa ÚNICAMENTE de la vida de 'x'.
fn procesar_separados(x: &str, y: &str) -> &str {
    x
}
```

---
layout: default
---

## Respuesta 31: Mezclando Lifetimes diferentes
<br/>

Cuando el retorno solo depende de `x`, se usan dos parámetros de lifetime
distintos:

```rust
fn procesar_separados<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
```

Así `y` puede tener cualquier vida sin afectar la garantía del resultado.

---
layout: default
---

## Ejercicio 32: El ciudadano eterno
<br/>

```rust
struct Mensaje<'a> {
    texto: &'a str,
}

fn main() {
    let m;
    {
        let literal = "Hola, soy estático"; // Vive en el binario (rodata)
        m = Mensaje { texto: literal };
    }
    println!("{}", m.texto); // 🤔 ¿Por qué esto SÍ compila perfectamente? ¿Qué lifetime tiene un literal?
}
```

---
layout: default
---

## Respuesta 32: El ciudadano eterno
<br/>

Los literales de cadena (`"..."`) tienen lifetime `'static`: están embebidos
directamente en el binario del programa (sección `.rodata`) y **viven durante
toda la ejecución**. Por eso el compilador acepta que `m.texto` sea válido
fuera del bloque interno; la memoria del literal nunca se libera.

---
layout: default
---

## Ejercicio 33: Métodos con Lifetimes
<br/>

```rust
struct Descriptor<'a> {
    info: &'a str,
}

// ❌ Completa la sintaxis de 'impl' para que el método pueda retornar la referencia interna
impl Descriptor {
    fn obtener_info(&self) -> &str {
        self.info
    }
}
```

---
layout: default
---

## Respuesta 33: Métodos con Lifetimes
<br/>

El bloque `impl` debe incluir el parámetro de lifetime del struct:

```rust
impl<'a> Descriptor<'a> {
    fn obtener_info(&self) -> &str {
        self.info
    }
}
```

El lifetime del valor retornado se infiere por las reglas de elision: está
ligado al de `&self`, lo que es correcto porque `self.info` no puede
sobrevivir al struct.

---
layout: default
---

## Ejercicio 34: El vector devorado
<br/>

```rust
fn procesar_primer_elemento(v: Vec<String>) {
    if let Some(primero) = v.first() {
        println!("Primero: {}", primero);
    }
}

fn main() {
    let mis_datos = vec![String::from("A"), String::from("B")];
    procesar_primer_elemento(mis_datos);
    println!("Total elementos: {}", mis_datos.len()); // ❌ ¿Por qué el vector entero dejó de existir?
}
```

---
layout: default
---

## Respuesta 34: El vector devorado
<br/>

La función recibe `Vec<String>` **por valor**, tomando propiedad del vector
completo. Aunque solo usa el primer elemento, el vector entero fue consumido
al cruzar la barrera de la función. Después de la llamada, `mis_datos` ya no
existe.

Corrección: cambiar la firma a `fn procesar_primer_elemento(v: &Vec<String>)`
o `v: &[String]`.

---
layout: default
---

## Ejercicio 35: Extracción parcial de Tuplas
<br/>

```rust
fn main() {
    let tupla = (String::from("Llave"), String::from("Valor"));
    let llave = tupla.0; // Extraemos el primer elemento
    println!("Llave: {}", llave);
    println!("Tupla completa: {:?}", tupla); // ❌ ¿Por qué la tupla quedó "parcialmente destruída"?
}
```

---
layout: default
---

## Respuesta 35: Extracción parcial de Tuplas
<br/>

Al hacer `let llave = tupla.0`, se **mueve** el primer elemento de la tupla
fuera de ella. La tupla queda parcialmente destruida: `tupla.0` ya no tiene
dueño válido, así que la tupla entera se vuelve inutilizable como unidad.
Rust no permite usar un valor del que algún campo fue movido.

Corrección: clonar (`tupla.0.clone()`) o desestructurar completa en un único
`let (llave, valor) = tupla;`.

---
layout: default
---

## Ejercicio 36: El Option insaciable
<br/>

```rust
fn revisar_opcional(opt: Option<String>) {
    match opt {
        Some(texto) => println!("Texto: {}", texto),
        None => println!("Vacío"),
    }
}

fn main() {
    let mi_opcion = Some(String::from("Contenido"));
    revisar_opcional(mi_opcion);
    // 🤔 ¿Cómo podrías revisar 'mi_opcion' sin que la función le robe la propiedad?
}
```

---
layout: default
---

## Respuesta 36: El Option insaciable
<br/>

La función toma `Option<String>` por valor. Para no ceder la propiedad:

```rust
revisar_opcional(mi_opcion.as_ref()); // Convierte a Option<&String>
// o
revisar_opcional(&mi_opcion);         // Presta el Option completo
```

Y ajustar la firma a `fn revisar_opcional(opt: Option<&String>)` o
`fn revisar_opcional(opt: &Option<String>)`.

---
layout: default
---

## Ejercicio 37: Reemplazo en estructuras mutables
<br/>

```rust
struct Nodo {
    valor: String,
}

fn main() {
    let mut n = Nodo { valor: String::from("Viejo") };
    let extraido = n.valor; // ❌ Intentas sacar la propiedad dejando el struct vacío.
    n.valor = String::from("Nuevo");
    // Pista: Investiga std::mem::replace o std::mem::take para solucionar esto.
}
```

---
layout: default
---

## Respuesta 37: Reemplazo en estructuras mutables
<br/>

Mover `n.valor` directamente deja el campo en estado no inicializado, lo que
Rust no permite si el struct debe seguir siendo válido. La solución idiomática:

```rust
// Opción 1: replace (intercambia el valor por otro)
let extraido = std::mem::replace(&mut n.valor, String::from("Nuevo"));

// Opción 2: take (reemplaza por el default del tipo)
let extraido = std::mem::take(&mut n.valor);
n.valor = String::from("Nuevo");
```

---
layout: default
---

## Ejercicio 38: El Closure acaparador
<br/>

```rust
fn main() {
    let texto = String::from("Datos del sistema");
    let imprimir = || println!("{}", texto);
    
    let manejar_hilo = std::thread::spawn(imprimir); // ❌ El hilo exige que el closure tenga propiedad total.
    manejar_hilo.join().unwrap();
    // Pista: Necesitas usar la palabra clave 'move' antes de las barras del closure.
}
```

---
layout: default
---

## Respuesta 38: El Closure acaparador
<br/>

Los hilos en Rust exigen que todo lo que capturan tenga lifetime `'static` (o
sea propiedad del closure). `imprimir` captura `texto` por referencia, pero esa
referencia podría expirar antes de que el hilo termine. La solución es `move`:

```rust
let imprimir = move || println!("{}", texto);
```

Ahora el closure es **dueño** de `texto`, y puede vivir indefinidamente en el hilo.

---
layout: default
---

## Ejercicio 39: Clonación selectiva en bucles
<br/>

```rust
fn main() {
    let plantilla = String::from("Usuario: ");
    let nombres = vec!["Ana", "Pedro", "Luis"];
    let mut registros = Vec::new();

    for nombre in nombres {
        let mut registro = plantilla; // ❌ Falla en la segunda iteración.
        registro.push_str(nombre);
        registros.push(registro);
    }
}
```

---
layout: default
---

## Respuesta 39: Clonación selectiva en bucles
<br/>

En la primera iteración, `plantilla` se mueve dentro del bucle. En la segunda,
ya no existe. Solución: clonar en cada iteración.

```rust
let mut registro = plantilla.clone();
```

---
layout: default
---

## Ejercicio 40: La trampa del método .to_string()
<br/>

```rust
fn main() {
    let literal = "datos_estáticos";
    let s1 = literal.to_string();
    let s2 = s1; // Transferencia
    println!("{}", s1); // ❌ Falla. ¿Por qué el .to_string() genera una variable con Ownership en el Heap?
}
```

---
layout: default
---

## Respuesta 40: La trampa del método .to_string()
<br/>

`.to_string()` (y `.to_owned()`) **asigna un nuevo String en el Heap** con sus
propias reglas de ownership. A diferencia de un `&str` literal, el `String`
resultante no es `Copy`. Al hacer `let s2 = s1`, se transfiere la propiedad,
invalidando `s1`.

---
layout: default
---

## Ejercicio 41: El tipo Box\<T\> (Punteros Inteligentes)
<br/>

```rust
fn main() {
    let b1 = Box::new(String::from("Datos en el Heap profundo"));
    let b2 = b1; 
    // 🤔 ¿Box<T> se comporta igual que los tipos primitivos del Stack
    //    o hereda las leyes de Ownership del String?
}
```

---
layout: default
---

## Respuesta 41: El tipo Box\<T\>
<br/>

`Box<T>` **hereda las leyes de Ownership**. Es un puntero inteligente que posee
el dato en el Heap. Al hacer `let b2 = b1`, se mueve la propiedad del `Box`
(y del dato que contiene) a `b2`. `b1` queda inválido, igual que con `String`.

---
layout: default
---

## Ejercicio 42: Vaciando colecciones con .drain()
<br/>

```rust
fn main() {
    let mut palabras = vec![String::from("uno"), String::from("dos")];
    for p in palabras.drain(..) {
        println!("Procesando: {}", p);
    }
    println!("Elementos restantes: {}", palabras.len()); // 🤔 ¿Compila? ¿Qué estado tiene el vector ahora?
}
```

---
layout: default
---

## Respuesta 42: Vaciando colecciones con .drain()
<br/>

Sí compila. `.drain(..)` vacía el vector durante la iteración pero **no consume
el vector en sí**: la propiedad del `Vec` permanece en `palabras`. Al terminar
el bucle, `palabras` sigue existiendo, solo que vacío. `palabras.len()` devuelve
`0`.

---
layout: default
---

## Ejercicio 43: Pérdida de propiedad por indexación directa
<br/>

```rust
fn main() {
    let nombres = vec![String::from("Alex"), String::from("Maria")];
    let primero = nombres[0]; // ❌ ¿Por qué no puedes mover un elemento directamente usando índices?
    println!("{}", primero);
}
```

---
layout: default
---

## Respuesta 43: Pérdida de propiedad por indexación directa
<br/>

El operador de indexación `[]` devuelve una **referencia** al elemento, no el
elemento en sí. No se puede mover un valor fuera de un vector por índice porque
dejaría esa posición del vector en un estado no inicializado (hueco en la
memoria del `Vec`), lo que Rust no permite. Para extraer elementos usa
`.remove(0)`, `.pop()`, o `.swap_remove(0)`.

---
layout: default
---

## Ejercicio 44: La función constructora
<br/>

```rust
struct Config {
    ruta: String,
}

impl Config {
    fn new(r: String) -> Self {
        Config { ruta: r } // 🤔 ¿Quién tiene la propiedad de 'r' al finalizar esta función?
    }
}

fn main() {
    let path = String::from("/var/log");
    let c = Config::new(path);
}
```

---
layout: default
---

## Respuesta 44: La función constructora
<br/>

Al llamar `Config::new(path)`, la propiedad del String pasa al parámetro `r`.
Dentro de `new`, `r` se mueve al campo `ruta` del struct. Al retornar, el
struct (que ahora posee el String) se mueve a `c`. Al finalizar `main`, `c`
sale de alcance y el String se destruye.

**`Config` (a través de `c`) tiene la propiedad final.**

---
layout: default
---

## Ejercicio 45: Desestructuración con patrones (match)
<br/>

```rust
fn main() {
    let resultado: Result<String, i32> = Ok(String::from("Éxito"));
    if let Ok(texto) = resultado {
        println!("{}", texto);
    }
    // ❌ Si intentas usar 'resultado' aquí abajo, fallará.
    //    ¿Cómo evitas que 'if let' consuma la propiedad?
}
```

---
layout: default
---

## Respuesta 45: Desestructuración con patrones (match)
<br/>

`if let Ok(texto) = resultado` mueve el `String` contenido en `Ok`. Para evitar
consumir la propiedad, desestrctura una referencia:

```rust
if let Ok(texto) = &resultado {   // texto: &String
    println!("{}", texto);
}
println!("{:?}", resultado); // Sigue siendo válido
```

---
layout: default
---

## Ejercicio 46: Asignaciones encadenadas
<br/>

```rust
fn main() {
    let mut a = String::from("materia");
    let b = a;
    let c = b;
    a = c; // 🤔 ¿Esto es válido en Rust? Analiza el viaje de la propiedad bit a bit.
    println!("{}", a);
}
```

---
layout: default
---

## Respuesta 46: Asignaciones encadenadas
<br/>

Sí, es completamente válido. El viaje de la propiedad:

1. `a` posee "materia".
2. `let b = a;` → `b` posee "materia", `a` inválido.
3. `let c = b;` → `c` posee "materia", `b` inválido.
4. `a = c;`     → `a` vuelve a poseer "materia", `c` inválido.

En la línea del `println!`, `a` es el dueño legítimo.

---
layout: default
---

## Ejercicio 47: Mutabilidad heredada en colecciones
<br/>

```rust
fn main() {
    let mut lista = vec![String::from("Trigo")];
    let elemento = lista.pop().unwrap(); // Sacamos el elemento
    // 🤔 'elemento' fue extraído de un vector mutable. ¿Es 'elemento' mutable por defecto?
    // Intenta hacer: elemento.push_str(" limpio");
}
```

---
layout: default
---

## Respuesta 47: Mutabilidad heredada en colecciones
<br/>

No. `elemento` **no es mutable por defecto**. La mutabilidad no se hereda del
contenedor. El binding `elemento` es inmutable a menos que lo declares
explícitamente:

```rust
let mut elemento = lista.pop().unwrap();
elemento.push_str(" limpio"); // Ahora sí compila
```

---
layout: default
---

## Ejercicio 48: Shadowing vs Ownership Transfer
<br/>

```rust
fn main() {
    let x = String::from("Alfa");
    let x = String::from("Beta"); // Shadowing
    // 🤔 ¿La memoria de "Alfa" se liberó inmediatamente o sigue existiendo
    //    en el Heap hasta el fin del main?
}
```

---
layout: default
---

## Respuesta 48: Shadowing vs Ownership Transfer
<br/>

Con shadowing, la primera variable (`"Alfa"`) **no se destruye inmediatamente**
en el punto del shadowing. Ambas existen como bindings distintos en el mismo
scope. En Rust, el orden de destrucción es el **inverso al de declaración**,
al finalizar el scope. Por tanto:

1. Al cerrar `main`, primero se destruye el segundo `x` → libera "Beta".
2. Luego se destruye el primer `x` → libera "Alfa".

`"Alfa"` vive en el Heap hasta el fin de `main`.

---
layout: default
---

## Ejercicio 49: Propiedad en Enumeraciones Complejas
<br/>

```rust
enum Mensaje {
    Texto(String),
    Salir,
}

fn procesar(m: Mensaje) { /* Reclama propiedad */ }

fn main() {
    let m = Mensaje::Texto(String::from("Alerta"));
    procesar(m);
    // ❌ ¿Puedes usar 'm' si el enum por dentro contenía un String?
}
```

---
layout: default
---

## Respuesta 49: Propiedad en Enumeraciones Complejas
<br/>

No. Aunque el variant sea `Texto(String)`, el enum completo (`m`) se mueve a
`procesar(m)`. La presencia de un `String` dentro del enum hace que el enum
entero no sea `Copy`. Después de la llamada, `m` ya no es accesible.

---
layout: default
---

## Ejercicio 50: El operador de descarte (_)
<br/>

```rust
fn main() {
    let s = String::from("Materia efímera");
    let _ = s; // Descarte inmediato
    println!("{}", s); // ❌ ¿Por qué el guión bajo destruye o invalida la variable al instante?
}
```

---
layout: default
---

## Respuesta 50: El operador de descarte (_)
<br/>

`let _ = s;` **mueve** `s` hacia el patrón `_`, que descarta el valor
**inmediatamente** al final del statement (no al final del scope). Esto es
diferente a `_nombre`, que sí crea un binding y vive hasta el fin del scope.
El resultado: `s` queda inválido en la siguiente línea.

---
layout: default
---

## Ejercicio 51: Mutabilidad en cascada a través de funciones
<br/>

```rust
fn agregar_prefijo(s: &mut String) {
    s.insert_str(0, "Pre-");
}

fn main() {
    let s = String::from("config"); // ❌ ¿Qué le falta a esta variable para cruzar el túnel mutable?
    agregar_prefijo(&mut s);
}
```

---
layout: default
---

## Respuesta 51: Mutabilidad en cascada a través de funciones
<br/>

La variable `s` debe declararse `mut` para poder prestarla como referencia
mutable:

```rust
let mut s = String::from("config");
agregar_prefijo(&mut s);
```

---
layout: default
---

## Ejercicio 52: La referencia del Iterator
<br/>

```rust
fn main() {
    let mut numeros = vec![1, 2, 3];
    for n in &numeros { // Prestamos el vector como lectura
        numeros.push(*n); // ❌ Intentas modificar la colección mientras el bucle la lee. ¿Por qué truena?
    }
}
```

---
layout: default
---

## Respuesta 52: La referencia del Iterator
<br/>

El bucle `for n in &numeros` crea un **préstamo inmutable** del vector que dura
todo el ciclo. `numeros.push(*n)` exige un préstamo mutable. Rust prohíbe tener
ambos activos simultáneamente: la regla es "lectores múltiples O un escritor,
no los dos".

---
layout: default
---

## Ejercicio 53: El préstamo en funciones puras
<br/>

```rust
fn es_largo(s: &str) -> bool {
    s.len() > 10
}

fn main() {
    let mut texto = String::from("Inmutable durante la lectura");
    let r = &texto;
    let chequeo = es_largo(r); // 🤔 ¿Puede una función aceptar una referencia de otra referencia?
    println!("¿Es largo? {}", chequeo);
}
```

---
layout: default
---

## Respuesta 53: El préstamo en funciones puras
<br/>

Sí. Rust aplica **deref coercion**: `&String` se convierte automáticamente a
`&str` al pasar como argumento. Además, pasar `r` (que es `&String`) a una
función que acepta `&str` es válido: es un préstamo de un préstamo, y el
compilador lo resuelve transparentemente.

---
layout: default
---

## Ejercicio 54: Préstamos simultáneos en Structs
<br/>

```rust
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Punto { x: 10, y: 20 };
    let r1 = &mut p.x; // Modifica X
    let r2 = &mut p.y; // Modifica Y
    // 🤔 ¿Por qué Rust permite dos referencias mutables al mismo struct si apuntan a campos diferentes?
    *r1 += 1;
    *r2 += 1;
}
```

---
layout: default
---

## Respuesta 54: Préstamos simultáneos en Structs
<br/>

El borrow checker es suficientemente inteligente para distinguir **campos no
solapados** del mismo struct. `&mut p.x` y `&mut p.y` apuntan a regiones de
memoria distintas y no hay posibilidad de aliasing. Rust permite esta
bifurcación (split borrow) porque puede garantizar que no existe conflicto.

---
layout: default
---

## Ejercicio 55: El bloqueo por alcance estructural
<br/>

```rust
struct Monitor {
    estado: String,
}

fn main() {
    let mut m = Monitor { estado: String::from("Activo") };
    let ref_estado = &m.estado;    // Bloqueo de lectura en un campo
    let ref_struct = &mut m;       // ❌ Intento de bloqueo mutable del struct completo.
    println!("{}", ref_estado);
}
```

---
layout: default
---

## Respuesta 55: El bloqueo por alcance estructural
<br/>

`ref_estado = &m.estado` establece un préstamo inmutable sobre parte del struct.
`&mut m` exigiría un préstamo mutable del struct **completo**, lo que incluye
`m.estado`. Esto violaría la regla: no se puede tener un préstamo mutable de
algo mientras hay un préstamo inmutable activo sobre cualquier parte de él.

---
layout: default
---

## Ejercicio 56: Referencias mutables dentro de condicionales
<br/>

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let r = &mut v;
    if r.len() > 0 {
        r.push(4); // 🤔 ¿Por qué es seguro leer la longitud y luego modificar usando el mismo puntero mutable?
    }
}
```

---
layout: default
---

## Respuesta 56: Referencias mutables dentro de condicionales
<br/>

Existe una sola referencia mutable `r`. Usar `r.len()` (lectura a través de
`&mut`) y `r.push(4)` (escritura a través de `&mut`) sobre el **mismo** puntero
no crea aliasing. Las restricciones de Rust son sobre referencias **distintas**
al mismo dato, no sobre operaciones sucesivas a través de la misma referencia.
El código es completamente seguro y válido.
