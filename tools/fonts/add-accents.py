"""
Add Portuguese (and Spanish/French) accented letters to the AVHershey Simplex fonts, drawn in
the fonts' own stroke: the originals carry ASCII only, so "LABORATÓRIO" fell back to another
font for its Ó. Accents are polylines thickened to the stem width, placed over the base letter
(over x-height for lower case, over the cap height for capitals); í drops i's dot.

    python tools/fonts/add-accents.py   (needs fonttools; idempotent: an existing glyph is kept)

The fonts are CC0 (public/fonts/AVHERSHEY-LICENSE.txt).
"""
import math
from pathlib import Path

from fontTools.pens.boundsPen import BoundsPen
from fontTools.pens.recordingPen import RecordingPen
from fontTools.pens.t2CharStringPen import T2CharStringPen
from fontTools.pens.transformPen import TransformPen
from fontTools.ttLib import TTFont

FONTS = Path(__file__).resolve().parents[2] / "public" / "fonts"
# letter → (base, accent)
LETTERS = {}
for base, marks in {"a": "grave acute circ tilde dier", "e": "grave acute circ dier", "i": "grave acute circ dier", "o": "grave acute circ tilde dier", "u": "grave acute circ dier", "n": "tilde", "c": "cedilla"}.items():
    for m in marks.split():
        for b in (base, base.upper()):
            name = {"grave": "̀", "acute": "́", "circ": "̂", "tilde": "̃", "dier": "̈", "cedilla": "̧"}[m]
            import unicodedata

            ch = unicodedata.normalize("NFC", b + name)
            if len(ch) == 1:
                LETTERS[ch] = (b, m)


def thick_line(pen, a, b, w):
    """a segment from a to b as a filled quad of width w (round-ish ends: a square cap)"""
    (x0, y0), (x1, y1) = a, b
    dx, dy = x1 - x0, y1 - y0
    n = math.hypot(dx, dy) or 1.0
    ux, uy = dx / n * w / 2, dy / n * w / 2  # along
    px, py = -uy, ux  # across
    pts = [(x0 - ux + px, y0 - uy + py), (x1 + ux + px, y1 + uy + py), (x1 + ux - px, y1 + uy - py), (x0 - ux - px, y0 - uy - py)]
    # counter-clockwise, like the font's outer contours
    area = sum(p[0] * q[1] - q[0] * p[1] for p, q in zip(pts, pts[1:] + pts[:1]))
    if area < 0:
        pts.reverse()
    pen.moveTo(pts[0])
    for p in pts[1:]:
        pen.lineTo(p)
    pen.closePath()


def polyline(pen, pts, w):
    for a, b in zip(pts, pts[1:]):
        thick_line(pen, a, b, w)


def accent(pen, mark, cx, y, w, size):
    """the mark centred on cx, its foot at y; size is its height"""
    h = size
    if mark == "acute":
        polyline(pen, [(cx - h * 0.35, y), (cx + h * 0.35, y + h)], w)
    elif mark == "grave":
        polyline(pen, [(cx + h * 0.35, y), (cx - h * 0.35, y + h)], w)
    elif mark == "circ":
        polyline(pen, [(cx - h * 0.6, y), (cx, y + h * 0.8), (cx + h * 0.6, y)], w)
    elif mark == "tilde":
        pts = [(cx + h * 0.75 * (2 * k / 12 - 1), y + h * 0.35 + h * 0.3 * math.sin(math.pi * 2 * k / 12)) for k in range(13)]
        polyline(pen, pts, w)
    elif mark == "dier":
        for x in (cx - h * 0.45, cx + h * 0.45):
            polyline(pen, [(x, y + h * 0.35), (x, y + h * 0.35 + 0.01)], w * 1.4)


def main():
    for path in sorted(FONTS.glob("AVHersheySimplex*.otf")):
        font = TTFont(path)
        cmap = font.getBestCmap()
        gs = font.getGlyphSet()
        top = font["CFF "].cff.topDictIndex[0]
        cff = font["CFF "].cff
        hmtx = font["hmtx"]
        stem = BoundsPen(gs)
        gs[cmap[ord("I")]].draw(stem)
        w = stem.bounds[2] - stem.bounds[0]  # the stroke width
        order = font.getGlyphOrder()
        added = []
        for ch, (base, mark) in LETTERS.items():
            if ord(ch) in cmap:
                continue
            bname = cmap[ord(base)]
            rec = RecordingPen()
            gs[bname].draw(rec)
            if base == "i":
                # dotless: drop the contours above x-height (the dot)
                contours, cur = [], []
                for op in rec.value:
                    cur.append(op)
                    if op[0] in ("closePath", "endPath"):
                        contours.append(cur)
                        cur = []
                keep = []
                for c in contours:
                    ys = [p[1] for op in c for p in op[1]]
                    if ys and min(ys) < 450:
                        keep.extend(c)
                rec.value = keep
            bb = BoundsPen(gs)
            gs[bname].draw(bb)
            x0, _, x1, _ = bb.bounds
            cx = (x0 + x1) / 2
            upper = base.isupper()
            # a CFF charstring's width is stored relative to the private dict's nominalWidthX
            pen = T2CharStringPen(hmtx[bname][0] - top.Private.nominalWidthX, gs)
            rec.replay(pen)
            if mark == "cedilla":
                # the comma's tail under the c
                cb = BoundsPen(gs)
                gs[cmap[ord(",")]].draw(cb)
                tp = TransformPen(pen, (1, 0, 0, 1, cx - (cb.bounds[0] + cb.bounds[2]) / 2, -cb.bounds[3] + 10))
                gs[cmap[ord(",")]].draw(tp)
            else:
                foot = 680 if upper else 500
                accent(pen, mark, cx, foot, w, 140 if upper else 150)
            name = "uni%04X" % ord(ch)
            cs = pen.getCharString(private=top.Private, globalSubrs=cff.GlobalSubrs)
            idx = top.CharStrings.charStringsIndex
            idx.items.append(cs)
            top.CharStrings.charStrings[name] = len(idx.items) - 1
            top.charset.append(name)
            order.append(name)
            hmtx[name] = hmtx[bname]
            for table in font["cmap"].tables:
                if table.isUnicode():
                    table.cmap[ord(ch)] = name
            added.append(ch)
        font.setGlyphOrder(order)
        font.save(path)
        print(path.name, "added", "".join(added) or "nothing")


if __name__ == "__main__":
    main()
