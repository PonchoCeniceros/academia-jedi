---
layout: section
---

# Estructuras de Control

---
layout: center
---

## 1. Condicional Estándar (`if` / `else`)
Evalúa una condición booleana pura. No lleva paréntesis en la condición.

```rust
    if condicion_booleana {
        // Se ejecuta si es true
    } else {
        // Opcional: Se ejecuta si es false
    }
```

--- 
layout: center
---

## 2. Coincidencia de Patrones Completa (`match`)
Obligatorio en Rust para evaluar múltiples caminos. Debe ser exhaustivo (cubrir todas las posibilidades).

```rust
    match expresion {
        patron_a => { /* código si coincide A */ },
        patron_b => { /* código si coincide B */ },
        _ => { /* opcional: "catch-all" para cualquier otro caso */ },
    }
```

--- 
layout: center
---

## 3. Condicional de Patrón Único (`if let`)
La versión compacta de match cuando solo te importa un caso específico (ideal para extraer valores de Option o Result).

```rust
    if let patron = expresion {
        // Se ejecuta si coincidió; las variables internas del patrón se pueden usar aquí
    } else {
        // Opcional: Se ejecuta si no coincidió
    }
```

--- 
layout: center
---

## 4. Bucle por Colección (`for in`)
La forma más común y segura de iterar en Rust. Funciona con rangos, arreglos o vectores.

```rust
    for variable in coleccion_o_rango {
        // Se ejecuta por cada elemento (variable toma el valor actual)
    }
```

--- 
layout: center
---

## 5. Bucle Condicional (`while`)
Ejecuta el bloque repetidamente mientras la condición booleana sea verdadera.

```rust
    while condicion_booleana {
        // Se ejecuta en bucle mientras sea true
    }
```

--- 
layout: center
---

## 6. Bucle de Patrón Único (`while let`)
Ejecuta el bucle repetidamente mientras la expresión siga coincidiendo con el patrón (muy usado para vaciar colas o canales).

```rust
    while let patron = expresion {
        // Se ejecuta en bucle mientras siga coincidiendo el patrón
    }
```

--- 
layout: center
---

## 7. Bucle Infinito (`loop`)
Un bucle eterno que no depende de una condición inicial. Es óptimo en Rust porque el compilador sabe que la única forma de salir es con un break explícito.

```rust
    loop {
        // Se ejecuta infinitamente
        if condicion_de_salida {
            break; // Detiene el bucle (puede devolver un valor: break valor;)
        }
    }
```
