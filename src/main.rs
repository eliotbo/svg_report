use printpdf::*;
use std::{fs::File, io::BufWriter};

fn main() {
    /* ── create a Letter page ─────────────────────────────────────────── */
    let (doc, page, layer) = PdfDocument::new("Text‑input demo", Mm(215.9), Mm(279.4), "Layer 1");
    let layer = doc.get_page(page).get_layer(layer);

    /* ── geometry parameters ─────────────────────────────────────────── */
    let rect_w = Mm(80.0); // width  80 mm
    let rect_h = Mm(15.0); // height 15 mm
    let origin = (Mm(65.0), Mm(140.0)); // lower‑left corner on page

    /* ── draw text input outline with curved sides ───────────────────── */
    draw_curved_sides_rect(&layer, origin, rect_w, rect_h);

    // draw a rounded rectangle below the pill
    let rounded_rect_origin = (Mm(origin.0 .0), Mm(origin.1 .0 - 25.0));
    let rounded_rect_w = rect_w;
    let rounded_rect_h = Mm(20.0);
    let corner_radius = Mm(4.0);
    draw_rounded_rect(&layer, rounded_rect_origin, rounded_rect_w, rounded_rect_h, corner_radius);

    /* ── add centred text ────────────────────────────────────────────── */
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let caption = "Jane Doe";
    let font_size_pt: f32 = 10.0;

    // text width in points (1 pt = 1/72 inch)
    let text_w_pt = {
        // printpdf does not currently expose a convenient API for obtaining the
        // rendered text width. For the sake of positioning the caption we use a
        // rough estimate based on the number of characters.
        caption.chars().count() as f32 * font_size_pt * 0.6
    };

    // convert helper: points ‑> Mm
    let pt_to_mm = |pt: f32| Mm(pt * 25.4 / 72.0);

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
/* helper: draw a capsule-like rectangle with curved left and right sides */
fn draw_curved_sides_rect(
    layer: &PdfLayerReference,
    origin: (Mm, Mm),
    w: Mm,
    h: Mm,
) {
    use printpdf::{Line, Point};

    const C: f32 = 0.55191505; // bezier approximation constant

    let (x0, y0) = origin;
    let (w, h) = (w.0, h.0);
    let r = h / 2.0; // radius for the side curves
    let c = r * C;

    let pts = vec![
        // start at bottom left after left curve
        (Point::new(Mm(x0.0 + r), Mm(y0.0)), false),
        // bottom edge
        (Point::new(Mm(x0.0 + w - r), Mm(y0.0)), false),

        // right side - bottom quarter
        (Point::new(Mm(x0.0 + w - r), Mm(y0.0)), true),
        (Point::new(Mm(x0.0 + w - r + c), Mm(y0.0)), true),
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + r - c)), true),
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + r)), false),

        // right side - top quarter
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + r)), true),
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + r + c)), true),
        (Point::new(Mm(x0.0 + w - r + c), Mm(y0.0 + h)), true),
        (Point::new(Mm(x0.0 + w - r), Mm(y0.0 + h)), false),

        // top edge
        (Point::new(Mm(x0.0 + r), Mm(y0.0 + h)), false),

        // left side - top quarter
        (Point::new(Mm(x0.0 + r), Mm(y0.0 + h)), true),
        (Point::new(Mm(x0.0 + r - c), Mm(y0.0 + h)), true),
        (Point::new(Mm(x0.0), Mm(y0.0 + r + c)), true),
        (Point::new(Mm(x0.0), Mm(y0.0 + r)), false),

        // left side - bottom quarter
        (Point::new(Mm(x0.0), Mm(y0.0 + r)), true),
        (Point::new(Mm(x0.0), Mm(y0.0 + r - c)), true),
        (Point::new(Mm(x0.0 + r - c), Mm(y0.0)), true),
        (Point::new(Mm(x0.0 + r), Mm(y0.0)), false),
    ];

    let line = Line {
        points: pts,
        is_closed: true,
        ..Default::default()
    };

    layer.add_line(line);
}

/* helper: draw a rectangle with rounded corners */
fn draw_rounded_rect(
    layer: &PdfLayerReference,
    origin: (Mm, Mm),
    w: Mm,
    h: Mm,
    r: Mm,
) {
    use printpdf::{Line, Point};

    const C: f32 = 0.55191505; // bezier approximation constant

    let (x0, y0) = origin;
    let (w, h, r) = (w.0, h.0, r.0);
    let c = r * C;

    let pts = vec![
        // start at bottom left after corner
        (Point::new(Mm(x0.0 + r), Mm(y0.0)), false),
        // bottom edge
        (Point::new(Mm(x0.0 + w - r), Mm(y0.0)), false),

        // bottom-right corner
        (Point::new(Mm(x0.0 + w - r), Mm(y0.0)), true),
        (Point::new(Mm(x0.0 + w - r + c), Mm(y0.0)), true),
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + r - c)), true),
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + r)), false),

        // right edge
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + h - r)), false),

        // top-right corner
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + h - r)), true),
        (Point::new(Mm(x0.0 + w), Mm(y0.0 + h - r + c)), true),
        (Point::new(Mm(x0.0 + w - r + c), Mm(y0.0 + h)), true),
        (Point::new(Mm(x0.0 + w - r), Mm(y0.0 + h)), false),

        // top edge
        (Point::new(Mm(x0.0 + r), Mm(y0.0 + h)), false),

        // top-left corner
        (Point::new(Mm(x0.0 + r), Mm(y0.0 + h)), true),
        (Point::new(Mm(x0.0 + r - c), Mm(y0.0 + h)), true),
        (Point::new(Mm(x0.0), Mm(y0.0 + h - r + c)), true),
        (Point::new(Mm(x0.0), Mm(y0.0 + h - r)), false),

        // left edge
        (Point::new(Mm(x0.0), Mm(y0.0 + r)), false),

        // bottom-left corner
        (Point::new(Mm(x0.0), Mm(y0.0 + r)), true),
        (Point::new(Mm(x0.0), Mm(y0.0 + r - c)), true),
        (Point::new(Mm(x0.0 + r - c), Mm(y0.0)), true),
        (Point::new(Mm(x0.0 + r), Mm(y0.0)), false),
    ];

    let line = Line {
        points: pts,
        is_closed: true,
        ..Default::default()
    };

    layer.add_line(line);
}
