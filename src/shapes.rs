use printpdf::*;
use printpdf::path::{PaintMode, WindingOrder};

#[derive(Copy, Clone, PartialEq)]
pub enum Symbol {
    Square,
    SquareFilled,
    Triangle,
    TriangleFilled,
    Circle,
    CircleFilled,
    S,
    SFilled,
    U,
    UFilled,
    X,
    XFilled,
    A,
    AFilled,
    Greater,
    GreaterFilled,
    Less,
    LessFilled,
    LeftBracket,
    LeftBracketFilled,
    RightBracket,
    RightBracketFilled,
    Star,
    StarFilled,
    ArrowDownRight,
    ArrowDownRightFilled,
    ArrowDownLeft,
    ArrowDownLeftFilled,
    VT,
    VTFilled,
}

#[derive(Copy, Clone)]
pub enum SymbolColor {
    Red,
    Blue,
}

impl SymbolColor {
    fn to_color(&self) -> Color {
        match self {
            SymbolColor::Red => Color::Rgb(Rgb::new(1.0, 0.0, 0.0, None)),
            SymbolColor::Blue => Color::Rgb(Rgb::new(0.0, 0.0, 1.0, None)),
        }
    }
}

pub fn draw_symbol(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    symbol: Symbol,
    pos: (Mm, Mm),
    size: f32,
    color: SymbolColor,
) {
    use Symbol::*;

    match symbol {
        Square | SquareFilled => {
            draw_square(layer, pos, size, symbol == SquareFilled, color);
        }
        Triangle | TriangleFilled => {
            draw_triangle(layer, pos, size, symbol == TriangleFilled, color);
        }
        Circle | CircleFilled => {
            draw_circle(layer, pos, size, symbol == CircleFilled, color);
        }
        _ => {
            let text = match symbol {
                Square => unreachable!(),
                SquareFilled => unreachable!(),
                Triangle => unreachable!(),
                TriangleFilled => unreachable!(),
                Circle => unreachable!(),
                CircleFilled => unreachable!(),
                S | SFilled => "S",
                U | UFilled => "U",
                X => "X",
                XFilled => "\u{2716}", // ✖
                A | AFilled => "A",
                Greater => ">",
                GreaterFilled => "\u{25B6}", // ▶
                Less => "<",
                LessFilled => "\u{25C0}", // ◀
                LeftBracket | LeftBracketFilled => "[",
                RightBracket | RightBracketFilled => "]",
                Star => "\u{2606}", // ☆
                StarFilled => "\u{2605}", // ★
                ArrowDownRight | ArrowDownRightFilled => "\u{2198}",
                ArrowDownLeft | ArrowDownLeftFilled => "\u{2199}",
                VT | VTFilled => "VT",
            };

            let c = color.to_color();
            layer.set_fill_color(c.clone());
            layer.set_outline_color(c);
            layer.use_text(text, size, pos.0, pos.1, font);
        }
    }
}

fn pt_to_mm(pt: f32) -> f32 {
    pt * 25.4 / 72.0
}

fn draw_square(
    layer: &PdfLayerReference,
    origin: (Mm, Mm),
    size_pt: f32,
    filled: bool,
    color: SymbolColor,
) {
    use printpdf::{Point, Polygon};
    let side = pt_to_mm(size_pt);
    let (x0, y0) = (origin.0 .0, origin.1 .0);
    let pts = vec![
        (Point::new(Mm(x0), Mm(y0)), false),
        (Point::new(Mm(x0 + side), Mm(y0)), false),
        (Point::new(Mm(x0 + side), Mm(y0 + side)), false),
        (Point::new(Mm(x0), Mm(y0 + side)), false),
    ];

    let poly = Polygon {
        rings: vec![pts],
        mode: if filled { PaintMode::FillStroke } else { PaintMode::Stroke },
        winding_order: WindingOrder::NonZero,
    };

    let c = color.to_color();
    layer.set_fill_color(c.clone());
    layer.set_outline_color(c);
    layer.add_polygon(poly);
}

fn draw_triangle(
    layer: &PdfLayerReference,
    origin: (Mm, Mm),
    size_pt: f32,
    filled: bool,
    color: SymbolColor,
) {
    use printpdf::{Point, Polygon};
    let side = pt_to_mm(size_pt);
    let (x0, y0) = (origin.0 .0, origin.1 .0);
    let pts = vec![
        (Point::new(Mm(x0), Mm(y0)), false),
        (Point::new(Mm(x0 + side / 2.0), Mm(y0 + side)), false),
        (Point::new(Mm(x0 + side), Mm(y0)), false),
    ];

    let poly = Polygon {
        rings: vec![pts],
        mode: if filled { PaintMode::FillStroke } else { PaintMode::Stroke },
        winding_order: WindingOrder::NonZero,
    };

    let c = color.to_color();
    layer.set_fill_color(c.clone());
    layer.set_outline_color(c);
    layer.add_polygon(poly);
}

fn draw_circle(
    layer: &PdfLayerReference,
    origin: (Mm, Mm),
    size_pt: f32,
    filled: bool,
    color: SymbolColor,
) {
    use printpdf::{Point, Polygon};

    const C: f32 = 0.55191505;
    let d = pt_to_mm(size_pt);
    let r = d / 2.0;
    let c = r * C;
    let (x0, y0) = (origin.0 .0, origin.1 .0);
    let pts = vec![
        (Point::new(Mm(x0 + r), Mm(y0)), false),
        // bottom-right quarter
        (Point::new(Mm(x0 + r), Mm(y0)), true),
        (Point::new(Mm(x0 + r + c), Mm(y0)), true),
        (Point::new(Mm(x0 + d), Mm(y0 + r - c)), true),
        (Point::new(Mm(x0 + d), Mm(y0 + r)), false),
        // top-right quarter
        (Point::new(Mm(x0 + d), Mm(y0 + r)), true),
        (Point::new(Mm(x0 + d), Mm(y0 + r + c)), true),
        (Point::new(Mm(x0 + r + c), Mm(y0 + d)), true),
        (Point::new(Mm(x0 + r), Mm(y0 + d)), false),
        // top-left quarter
        (Point::new(Mm(x0 + r), Mm(y0 + d)), true),
        (Point::new(Mm(x0 + r - c), Mm(y0 + d)), true),
        (Point::new(Mm(x0), Mm(y0 + r + c)), true),
        (Point::new(Mm(x0), Mm(y0 + r)), false),
        // bottom-left quarter
        (Point::new(Mm(x0), Mm(y0 + r)), true),
        (Point::new(Mm(x0), Mm(y0 + r - c)), true),
        (Point::new(Mm(x0 + r - c), Mm(y0)), true),
        (Point::new(Mm(x0 + r), Mm(y0)), false),
    ];

    let poly = Polygon {
        rings: vec![pts],
        mode: if filled { PaintMode::FillStroke } else { PaintMode::Stroke },
        winding_order: WindingOrder::NonZero,
    };

    let col = color.to_color();
    layer.set_fill_color(col.clone());
    layer.set_outline_color(col);
    layer.add_polygon(poly);
}
