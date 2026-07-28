# Notas de Algoritmos

Deck Slidev con **patrones de razonamiento** para programación competitiva.

```bash
pnpm --dir docs/algorithms run dev     # http://localhost:3033
```

## Por qué vive aquí y no bajo `rust/`

Estas notas son **independientes del lenguaje**: el patrón de razonamiento
aplica igual en Rust, Python o cualquier otro. Por eso viven en `docs/`, junto a
los whiteboards, y no dentro de `rust/fundamentals/`, que trata específicamente
del lenguaje.

Los ejemplos citan los trials del repo por conveniencia (son código real y ya
resuelto), no porque el patrón dependa de Rust.

## Contenido

| Sección | Estado |
| --- | --- |
| 1. Programación Dinámica — el mapa (A→E) + 4 ejemplos | ✅ |
| 2. Búsqueda Binaria | pendiente |
| 3. Backtracking | pendiente |

## El mapa de DP

Cinco preguntas que convierten un problema de optimización recursivo en código:

| # | Pregunta | Se traduce en |
| --- | --- | --- |
| **A** | ¿Cuál es mi estado? | los parámetros y la clave del memo |
| **B** | ¿Qué decisiones puedo tomar? | una llamada recursiva por opción (o un bucle) |
| **C** | ¿Qué optimizo? | el significado del valor devuelto |
| **D** | ¿Cómo combino las decisiones? | `max` · `min` · `suma` · `OR` |
| **E** | ¿Qué represento si es imposible? | el valor neutro del operador de D |

Ejemplos desarrollados: **70** Climbing Stairs (`suma`), **746** Min Cost
Climbing Stairs (`min`), **198** House Robber (`max`) y **322** Coin Change
(`min` con ramificación n-aria y casos imposibles).

## Decks del repo

| Deck | Tema | Puerto |
| --- | --- | --- |
| `rust/fundamentals/notes` | Rust Fundamentos — el lenguaje | 3031 |
| `rust/building/notes` | Rust Aplicado — construir con el lenguaje | 3032 |
| `docs/algorithms` | Algoritmos — patrones, sin lenguaje | 3033 |
