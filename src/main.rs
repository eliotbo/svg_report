use printpdf::*;
use std::{fs::File, io::BufWriter};

fn main() {
    /* ── create a Letter page ─────────────────────────────────────────── */
    let (doc, page, layer) = PdfDocument::new("Text‑input demo", Mm(215.9), Mm(279.4), "Layer 1");
    let layer = doc.get_page(page).get_layer(layer);

    /* ── geometry parameters ─────────────────────────────────────────── */
    let rect_w = Mm(80.0); // width  80 mm
    let rect_h = Mm(15.0); // height 15 mm
    let radius = Mm(4.0); // 4 mm corner radius
    let origin = (Mm(65.0), Mm(140.0)); // lower‑left corner on page

    /* ── draw rounded rectangle ──────────────────────────────────────── */
    draw_rounded_rect(&layer, origin, rect_w, rect_h, radius);

    /* ── add centred text ────────────────────────────────────────────── */
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let caption = "Jane Doe";
    let font_size_pt = 10.0;

    // text width in points (1 pt = 1/72 inch)
    let text_w_pt =
        font.get_text_width(caption, font_size_pt).unwrap() as f64 * font_size_pt / 1000.0;

    // convert helper: points ‑> Mm
    let pt_to_mm = |pt: f64| Mm(pt * 25.4 / 72.0);

    // centre inside the rectangle
    let (x0, y0) = origin;
    let text_x = x0.0 + (rect_w.0 - pt_to_mm(text_w_pt).0) * 0.5;
    let text_y = y0.0 + (rect_h.0 + pt_to_mm(font_size_pt * 0.30).0) * 0.5;

    layer.use_text(caption, font_size_pt, Mm(text_x), Mm(text_y), &font);

    /* ── save ─────────────────────────────────────────────────────────── */
    doc.save(&mut BufWriter::new(
        File::create("text_input_demo.pdf").unwrap(),
    ))
    .unwrap();
}

/* --------------------------------------------------------------------- */
/* helper: rounded rectangle built from straight edges + curved corners  */
fn draw_rounded_rect(layer: &PdfLayerReference, origin: (Mm, Mm), w: Mm, h: Mm, r: Mm) {
    use printpdf::{Line, Point};

    let (x0, y0) = origin;
    let (w, h, r) = (w.0, h.0, r.0);

    // points around the perimeter, clockwise
    let pts = vec![
        (Point::new(Mm(x0.0 + r), Mm(y0.0)), false),     // move
        (Point::new(Mm(x0.0 + w - r), Mm(y0.0)), false), // top edge
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + r)), true),  // top‑right curve
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + h - r)), false), // right edge
        (Point::new(Mm(x0.0 + w - r), Mm(y0.0 + h)), true), // bottom‑right curve
        (Point::new(Mm(x0.0 + r), Mm(y0.0 + h)), false), // bottom edge
        (Point::new(Mm(x0.0), Mm(y0.0 + h - r)), true),  // bottom‑left curve
        (Point::new(Mm(x0.0), Mm(y0.0 + r)), false),     // left edge
        (Point::new(Mm(x0.0 + r), Mm(y0.0)), true),      // top‑left curve, back to start
    ];

    // build and add shape
    Line {
        points: pts,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    }
    .add_to_layer(layer.clone());
}
