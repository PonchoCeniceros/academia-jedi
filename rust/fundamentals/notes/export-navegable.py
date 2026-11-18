#!/usr/bin/env python3
"""Exporta el deck completo a un PDF navegable.

Sobre el export normal de Slidev agrega:

- Marcadores (panel lateral del lector): sección -> tema, p. ej.
  "3 · Memoria" -> "3.2 · Ownership".
- Los enlaces del índice (lámina "Contenido") saltan a la página dentro del
  PDF; Slidev los exporta como URLs a localhost, que no sirven.
- Tras la portada de cada sección, una página con sus temas, clicables.
- Botones "<- Índice" y "<- Sección" en la esquina de cada página: regresan
  al índice general y a la página de temas de la sección en curso.

Los números de página salen de slides.md y pages/*.md en cada corrida (la
misma cuenta que export-sections.py), y se verifican contra el texto del PDF:
la página donde empieza cada tema debe traer su encabezado.

`slidev export` a veces escribe páginas en blanco sin avisar; se reintenta
hasta que todas las páginas traen texto.

Uso:
    python3 export-navegable.py                 # exports/Rust · Cuaderno de campo - Navegable.pdf
    python3 export-navegable.py --light         # versión clara

Requiere: pypdf y reportlab (pip install --user pypdf reportlab)
"""
import io
import pathlib
import re
import subprocess
import sys
import time

from pypdf import PdfReader, PdfWriter
from pypdf.generic import (
    ArrayObject,
    DictionaryObject,
    FloatObject,
    NameObject,
    NumberObject,
)
from reportlab.pdfgen import canvas

ROOT = pathlib.Path(__file__).resolve().parent
SLIDES_MD = ROOT / "slides.md"
PAGES = ROOT / "pages"
OUT = ROOT / "exports"
INDICE = 3  # lámina "Contenido"


def sin_comentarios(text: str) -> str:
    return re.sub(r"<!--.*?-->", "", text, flags=re.S)


def count_slides(text: str) -> int:
    # Slidev parte en todo `---` de columna 0, aun dentro de comentarios.
    lines = text.split("\n")
    sep_idx = [i for i, l in enumerate(lines) if l.rstrip() == "---"]
    assert len(sep_idx) % 2 == 0, "separadores '---' desbalanceados"
    return len(sep_idx) // 2


def build_outline() -> list[tuple[str, int, list[tuple[str, int]]]]:
    """[(sección, página, [(tema, página), ...]), ...] con páginas 1-based."""
    # Las láminas comentadas al final traen los separadores con sangría.
    lines = SLIDES_MD.read_text().split("\n")
    sep_idx = [i for i, l in enumerate(lines) if l.rstrip() == "---"]
    pairs = list(zip(sep_idx[0::2], sep_idx[1::2]))

    outline: list = []
    cursor = 1
    for k, (a, b) in enumerate(pairs):
        src = None
        for l in lines[a + 1 : b]:
            m = re.match(r"^\s*src:\s*(.+?)\s*$", l)
            if m:
                src = m.group(1)
        if src is None:
            fin = pairs[k + 1][0] if k + 1 < len(pairs) else len(lines)
            cuerpo = "\n".join(lines[b + 1 : fin])
            m = re.search(r'<h1 class="cp-cover-text">([^<]+)</h1>', cuerpo)
            if m and re.match(r"^\d+ · ", m.group(1)):
                outline.append((m.group(1).strip(), cursor, []))
            cursor += 1
            continue
        texto = (PAGES / pathlib.Path(src).name).read_text()
        m = re.search(r"^#{1,3} (.+)$", sin_comentarios(texto), re.M)
        titulo = m.group(1).strip() if m else pathlib.Path(src).stem
        if outline and re.match(r"^\d+ · ", outline[-1][0]):
            outline[-1][2].append((titulo, cursor))
        else:
            outline.append((titulo, cursor, []))
        cursor += count_slides(texto)
    return outline


def paginas_vacias(pdf: pathlib.Path) -> list[int]:
    reader = PdfReader(str(pdf))
    return [i + 1 for i, p in enumerate(reader.pages) if len((p.extract_text() or "").strip()) < 3]


def exportar(destino: pathlib.Path, dark: bool, intentos: int = 4) -> None:
    cmd = ["pnpm", "exec", "slidev", "export", "slides.md", "--timeout", "120000", "--wait", "300",
           "--output", str(destino)]
    if dark:
        cmd.append("--dark")
    for intento in range(1, intentos + 1):
        subprocess.run(cmd, cwd=ROOT, check=True)
        vacias = paginas_vacias(destino)
        if not vacias:
            return
        print(f"   ! intento {intento}: {len(vacias)} páginas sin texto, reintentando...")
        time.sleep(3)
    raise RuntimeError(f"No se pudo exportar el deck completo tras {intentos} intentos")


def norm(s: str) -> str:
    return re.sub(r"\s+", "", s)


def verificar(reader: PdfReader, outline) -> None:
    errores = []
    for sec, pag, temas in outline:
        for titulo, p in [(sec, pag), *temas]:
            texto = norm(reader.pages[p - 1].extract_text() or "")
            if norm(titulo.split("·")[-1]) not in texto:
                errores.append(f"  '{titulo}' no aparece en la página {p}")
    if errores:
        sys.exit("Los números de página no cuadran con el PDF:\n" + "\n".join(errores))


def colores(dark: bool) -> dict[str, tuple[float, float, float]]:
    if dark:
        return {"fondo": (0.133, 0.133, 0.133), "titulo": (0.85, 0.85, 0.85),
                "texto": (0.75, 0.75, 0.75), "tenue": (0.5, 0.5, 0.5)}
    return {"fondo": (1, 1, 1), "titulo": (0.15, 0.15, 0.15),
            "texto": (0.25, 0.25, 0.25), "tenue": (0.55, 0.55, 0.55)}


def botones(ancho: float, alto: float, con_seccion: bool) -> tuple[PdfReader, list[float], list[float] | None]:
    """Capa transparente con "<- Índice" (y "<- Sección") arriba a la derecha,
    y los rectángulos clicables de cada uno."""
    buf = io.BytesIO()
    c = canvas.Canvas(buf, pagesize=(ancho, alto))
    c.setFont("Helvetica", 7)
    c.setFillColorRGB(0.6, 0.6, 0.6)
    y = alto - 14
    texto = "<- Índice"
    w = c.stringWidth(texto, "Helvetica", 7)
    x = ancho - w - 10
    c.drawString(x, y, texto)
    rect_indice = [x - 4, y - 4, x + w + 4, y + 10]
    rect_seccion = None
    if con_seccion:
        texto = "<- Sección"
        w2 = c.stringWidth(texto, "Helvetica", 7)
        x2 = x - w2 - 14
        c.drawString(x2, y, texto)
        rect_seccion = [x2 - 4, y - 4, x2 + w2 + 4, y + 10]
    c.save()
    buf.seek(0)
    return PdfReader(buf), rect_indice, rect_seccion


def pagina_temas(ancho: float, alto: float, seccion: str, temas: list[tuple[str, int]],
                 dark: bool) -> tuple[PdfReader, list[list[float]]]:
    """Página "temas de la sección": la lista de temas con su número de lámina
    (el que trae impreso cada lámina, no el de página del PDF, que se corre por
    estas páginas insertadas) y el rectángulo clicable de cada renglón."""
    col = colores(dark)
    buf = io.BytesIO()
    c = canvas.Canvas(buf, pagesize=(ancho, alto))
    c.setFillColorRGB(*col["fondo"])
    c.rect(0, 0, ancho, alto, stroke=0, fill=1)
    x0, x1 = 60, ancho - 60
    y = alto - 70
    c.setFillColorRGB(*col["titulo"])
    c.setFont("Helvetica-Bold", 22)
    c.drawString(x0, y, seccion)
    c.setFillColorRGB(*col["tenue"])
    c.setFont("Helvetica", 9)
    c.drawString(x0, y - 18, "Temas de la sección")
    y -= 52
    paso = min(26, (y - 30) / max(1, len(temas)))
    rects = []
    for titulo, pag in temas:
        c.setFillColorRGB(*col["texto"])
        c.setFont("Helvetica", 13)
        c.drawString(x0, y, titulo)
        c.setFillColorRGB(*col["tenue"])
        c.setFont("Helvetica", 11)
        c.drawRightString(x1, y, str(pag))
        c.setStrokeColorRGB(*col["tenue"])
        c.setLineWidth(0.3)
        c.setDash(1, 3)
        c.line(x0 + c.stringWidth(titulo, "Helvetica", 13) + 8, y + 2,
               x1 - c.stringWidth(str(pag), "Helvetica", 11) - 8, y + 2)
        rects.append([x0 - 4, y - 6, x1 + 4, y + 15])
        y -= paso
    c.save()
    buf.seek(0)
    return PdfReader(buf), rects


def enlace_interno(writer: PdfWriter, rect: list[float], destino: int) -> DictionaryObject:
    return DictionaryObject({
        NameObject("/Type"): NameObject("/Annot"),
        NameObject("/Subtype"): NameObject("/Link"),
        NameObject("/Rect"): ArrayObject([FloatObject(v) for v in rect]),
        NameObject("/Border"): ArrayObject([NumberObject(0)] * 3),
        NameObject("/Dest"): ArrayObject([writer.pages[destino].indirect_reference, NameObject("/Fit")]),
    })


def agregar_enlace(writer: PdfWriter, pagina: int, rect: list[float], destino: int) -> None:
    page = writer.pages[pagina]
    if "/Annots" not in page:
        page[NameObject("/Annots")] = ArrayObject()
    page["/Annots"].append(writer._add_object(enlace_interno(writer, rect, destino)))


def main() -> None:
    dark = "--light" not in sys.argv
    OUT.mkdir(exist_ok=True)
    crudo = OUT / "_deck-completo.pdf"
    final = OUT / f"Rust · Cuaderno de campo - Navegable - {'Dark' if dark else 'Light'}.pdf"

    outline = build_outline()
    if "--desde" in sys.argv:  # reusar un export ya hecho (útil al ajustar este script)
        crudo = pathlib.Path(sys.argv[sys.argv.index("--desde") + 1]).resolve()
    else:
        print("Exportando el deck completo...")
        exportar(crudo, dark)

    reader = PdfReader(str(crudo))
    verificar(reader, outline)
    es_seccion = lambda titulo: re.match(r"^\d+ · ", titulo)
    portadas = {pag: (sec, temas) for sec, pag, temas in outline if es_seccion(sec)}

    # 0. Armar el PDF final: tras cada portada de sección va su página de temas.
    # nuevo[p] = índice (0-based) en el PDF final de la página p (1-based) del export.
    nuevo: dict[int, int] = {}
    pag_temas: dict[int, int] = {}  # portada (1-based, export) -> página de temas (0-based, final)
    for p in range(1, len(reader.pages) + 1):
        nuevo[p] = p - 1 + len(pag_temas)
        if p in portadas:
            pag_temas[p] = nuevo[p] + 1
    writer = PdfWriter(clone_from=reader)
    caja = writer.pages[0].mediabox
    ancho, alto = float(caja.width), float(caja.height)
    rects_temas: dict[int, list[list[float]]] = {}
    for portada, destino in pag_temas.items():
        sec, temas = portadas[portada]
        capa, rects = pagina_temas(ancho, alto, sec, temas, dark)
        writer.insert_page(capa.pages[0], destino)
        rects_temas[portada] = rects
    total = len(writer.pages)

    # 1. Enlaces del índice: Slidev los exporta como http://localhost:NNNN/45.
    # El número del href puede estar desfasado (sync-indice.py), así que el
    # destino sale de la cuenta verificada: el k-ésimo enlace va a la sección k.
    enlaces = [ref.get_object() for ref in writer.pages[nuevo[INDICE]].get("/Annots") or []
               if re.match(r"^https?://localhost:\d+/\d+$",
                           (ref.get_object().get("/A") or {}).get("/URI", ""))]
    if len(enlaces) != len(portadas):
        sys.exit(f"El índice trae {len(enlaces)} enlaces y hay {len(portadas)} secciones")
    for annot, portada in zip(enlaces, sorted(portadas)):
        del annot["/A"]
        annot[NameObject("/Dest")] = ArrayObject(
            [writer.pages[nuevo[portada]].indirect_reference, NameObject("/Fit")])

    # El resto de enlaces del deck (p. ej. "← Índice" de las portadas) apunta a
    # números de lámina; se vuelven saltos a la página que les toca en el PDF.
    for page in writer.pages:
        for ref in page.get("/Annots") or []:
            annot = ref.get_object()
            m = re.match(r"^https?://localhost:\d+/(\d+)$", (annot.get("/A") or {}).get("/URI", ""))
            if m and int(m.group(1)) in nuevo:
                del annot["/A"]
                annot[NameObject("/Dest")] = ArrayObject(
                    [writer.pages[nuevo[int(m.group(1))]].indirect_reference, NameObject("/Fit")])

    # 2. Renglones clicables de cada página de temas.
    for portada, rects in rects_temas.items():
        for (titulo, p), rect in zip(portadas[portada][1], rects):
            agregar_enlace(writer, pag_temas[portada], rect, nuevo[p])

    # 3. Botones "<- Índice" y "<- Sección" en todas las páginas tras el índice.
    solo_indice = botones(ancho, alto, con_seccion=False)
    con_seccion = botones(ancho, alto, con_seccion=True)
    # Cada página pertenece al último marcador de primer nivel que la precede;
    # si ese marcador no es una sección (p. ej. "Cómo leer"), no hay "Sección".
    inicios = {pag: (pag if es_seccion(sec) else None) for sec, pag, _ in outline}
    seccion_actual = None
    for p in range(INDICE + 1, len(reader.pages) + 1):
        seccion_actual = inicios.get(p, seccion_actual)
        if p in portadas:  # la portada ya trae su "← Índice" desde slides.md
            continue
        capa, r_ind, r_sec = con_seccion if seccion_actual else solo_indice
        i = nuevo[p]
        writer.pages[i].merge_page(capa.pages[0])
        agregar_enlace(writer, i, r_ind, nuevo[INDICE])
        if seccion_actual:
            agregar_enlace(writer, i, r_sec, pag_temas[seccion_actual])
    for portada, i in pag_temas.items():
        capa, r_ind, _ = solo_indice
        writer.pages[i].merge_page(capa.pages[0])
        agregar_enlace(writer, i, r_ind, nuevo[INDICE])

    # 4. Marcadores.
    writer.add_outline_item("Contenido", nuevo[INDICE])
    for sec, pag, temas in outline:
        padre = writer.add_outline_item(sec, nuevo[pag])
        for titulo, p in temas:
            writer.add_outline_item(titulo, nuevo[p], parent=padre)
    writer.page_mode = "/UseOutlines"
    writer.add_metadata({"/Title": "Rust · Cuaderno de campo"})

    with open(final, "wb") as f:
        writer.write(f)
    if "--desde" not in sys.argv:
        crudo.unlink()
    n_temas = sum(len(t) for _, _, t in outline)
    print(f"\nListo: {final.relative_to(ROOT)}")
    print(f"  {total} páginas · {len(portadas)} secciones con página de temas · {n_temas} temas · "
          f"{len(enlaces)} enlaces del índice")


if __name__ == "__main__":
    main()
