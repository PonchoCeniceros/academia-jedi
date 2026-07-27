# Building

Pista de **desarrollo aplicado** en Rust: construir cosas que alguien usaría, en
contraste con `../fundamentals/`, que se enfoca en dominar el lenguaje a través
de algoritmos.

## Estructura prevista

```
building/
├── notes/       # deck de conceptos (módulos y crates, arquitectura de
│                # errores, async/await) + apéndices de entrada a las
│                # tecnologías (tokio, axum, polars)
└── projects/    # un proyecto Cargo independiente por cada cosa construida,
                 # con su propio Cargo.toml y su README con los detalles
                 # específicos de las crates que usa
```

## Por qué separado de `fundamentals/`

- **Dependencias aisladas.** Los trials son binarios sueltos sin dependencias
  pesadas; meter `tokio`/`axum`/`polars` en ese paquete inflaría la compilación
  de los 34 trials.
- **Las notas no se pudren.** Los conceptos (async, propagación de errores) son
  estables; las APIs de las crates cambian seguido. Por eso el detalle de cada
  tecnología vive en el README de su proyecto, junto al `Cargo.toml` que fija su
  versión, y no en el deck.
