# Building

Pista de **desarrollo aplicado** en Rust: construir cosas que alguien usaría, en
contraste con `../fundamentals/`, que se enfoca en dominar el lenguaje a través
de algoritmos.

## Estructura

```
building/
├── notes/       # deck Slidev de conceptos  ← ya existe
└── projects/    # un proyecto Cargo independiente por cada cosa construida,
                 # con su propio Cargo.toml y su README con los detalles
                 # específicos de las crates que usa  ← por crear
```

## El deck

```bash
pnpm --dir rust/building/notes run dev     # http://localhost:3032
```

Contenido (en progreso):

| Sección | Estado |
| --- | --- |
| 1. Módulos y Crates | ✅ |
| 2. Arquitectura de Errores (`thiserror` / `anyhow`) | pendiente |
| 3. Async / Await | pendiente |
| Apéndices: Tokio · Axum · Polars | pendiente |

El deck de fundamentals corre en el puerto 3031, este en el 3032, así que
ambos pueden estar arriba a la vez.

## Por qué separado de `fundamentals/`

- **Dependencias aisladas.** Los trials son binarios sueltos sin dependencias
  pesadas; meter `tokio`/`axum`/`polars` en ese paquete inflaría la compilación
  de los 34 trials.
- **Las notas no se pudren.** Los conceptos (async, propagación de errores) son
  estables; las APIs de las crates cambian seguido. Por eso el detalle de cada
  tecnología vive en el README de su proyecto, junto al `Cargo.toml` que fija su
  versión, y no en el deck.
