use printpdf::*;

#[derive(Copy, Clone)]
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
    let text = match symbol {
        Square => "\u{25A1}",        // □
        SquareFilled => "\u{25A0}",   // ■
        Triangle => "\u{25B3}",      // △
        TriangleFilled => "\u{25B2}", // ▲
        Circle => "\u{25CB}",        // ○
        CircleFilled => "\u{25CF}",   // ●
        S | SFilled => "S",
        U | UFilled => "U",
        X => "X",
        XFilled => "\u{2716}",       // ✖
        A | AFilled => "A",
        Greater => ">",
        GreaterFilled => "\u{25B6}",  // ▶
        Less => "<",
        LessFilled => "\u{25C0}",     // ◀
        LeftBracket | LeftBracketFilled => "[",
        RightBracket | RightBracketFilled => "]",
        Star => "\u{2606}",          // ☆
        StarFilled => "\u{2605}",     // ★
        ArrowDownRight | ArrowDownRightFilled => "\u{2198}",
        ArrowDownLeft | ArrowDownLeftFilled => "\u{2199}",
        VT | VTFilled => "VT",
    };

    let c = color.to_color();
    layer.set_fill_color(c.clone());
    layer.set_outline_color(c);
    layer.use_text(text, size, pos.0, pos.1, font);
}
