use printpdf::*;
use owned_ttf_parser::{AsFaceRef, OwnedFace};
use std::{fs::File, io::BufWriter};

mod shapes;
use shapes::{draw_symbol, Symbol, SymbolColor};

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
    // load an external font so we can query exact glyph metrics
    let font_bytes = include_bytes!("../vendor/printpdf/assets/fonts/RobotoMedium.ttf");
    let mut reader = std::io::Cursor::new(font_bytes.as_ref());
    let font = doc.add_external_font(&mut reader).unwrap();

    let caption = "Jane Doe";
    let font_size_pt: f32 = 10.0;

    // helper for converting between points and millimetres
    let pt_to_mm = |pt: f32| pt * 25.4 / 72.0;

    // compute exact text width using glyph metrics
    let face = OwnedFace::from_vec(font_bytes.to_vec(), 0).unwrap();
    let units_per_em = face.as_face_ref().units_per_em() as f32;
    let mut width_units = 0u32;
    for ch in caption.chars() {
        if let Some(gid) = face.as_face_ref().glyph_index(ch) {
            if let Some(adv) = face.as_face_ref().glyph_hor_advance(gid) {
                width_units += adv as u32;
            }
        }
    }
    let ascent = face.as_face_ref().ascender() as f32;
    let descent = face.as_face_ref().descender() as f32;
    let width_pt = width_units as f32 / units_per_em * font_size_pt;
    let height_pt = (ascent - descent) / units_per_em * font_size_pt;
    let descent_pt = -descent / units_per_em * font_size_pt;
    let text_w_mm = pt_to_mm(width_pt);
    let text_h_mm = pt_to_mm(height_pt);
    let descent_mm = pt_to_mm(descent_pt);

    // centre inside the rounded rectangle
    let (x0, y0) = rounded_rect_origin;
    let text_x = x0.0 + (rounded_rect_w.0 - text_w_mm) * 0.5;
    let baseline_y = y0.0 + (rounded_rect_h.0 - text_h_mm) * 0.5 + descent_mm;

    layer.use_text(caption, font_size_pt, Mm(text_x), Mm(baseline_y), &font);

    // draw various audiogram symbols below
    let sym_font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let mut sx = Mm(origin.0 .0);
    let sy = Mm(origin.1 .0 - 50.0);
    let step = Mm(6.0);
    let symbols = [
        Symbol::Square,
        Symbol::SquareFilled,
        Symbol::Triangle,
        Symbol::TriangleFilled,
        Symbol::Circle,
        Symbol::CircleFilled,
        Symbol::S,
        Symbol::SFilled,
        Symbol::U,
        Symbol::UFilled,
        Symbol::X,
        Symbol::XFilled,
        Symbol::A,
        Symbol::AFilled,
        Symbol::Greater,
        Symbol::GreaterFilled,
        Symbol::Less,
        Symbol::LessFilled,
        Symbol::LeftBracket,
        Symbol::LeftBracketFilled,
        Symbol::RightBracket,
        Symbol::RightBracketFilled,
        Symbol::Star,
        Symbol::StarFilled,
        Symbol::ArrowDownRight,
        Symbol::ArrowDownRightFilled,
        Symbol::ArrowDownLeft,
        Symbol::ArrowDownLeftFilled,
        Symbol::VT,
        Symbol::VTFilled,
    ];
    for (i, sym) in symbols.iter().enumerate() {
        let color = if i % 2 == 0 { SymbolColor::Red } else { SymbolColor::Blue };
        draw_symbol(&layer, &sym_font, *sym, (sx, sy), 10.0, color);
        sx = Mm(sx.0 + step.0);
    }

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
