use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

// A4 page dimensions in millimeters
const PAGE_WIDTH: f32 = 210.0;
const PAGE_HEIGHT: f32 = 297.0;
const MARGIN: f32 = 10.0;

// Font sizes
const FONT_SIZE_LARGE_TITLE: f32 = 18.;
const FONT_SIZE_TITLE: f32 = 11.;
const FONT_SIZE_NORMAL: f32 = 9.;
const FONT_SIZE_SMALL: f32 = 7.;
const FONT_SIZE_SYMBOL: f32 = 12.;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new PDF document
    let (doc, page1, layer1) =
        PdfDocument::new("Audiogram", Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Load a standard font
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    // Draw all sections of the audiogram
    draw_header(&current_layer, &font);
    draw_tonal_audiometry(&current_layer, &font);
    draw_evaluation_details(&current_layer, &font);
    draw_speech_audiometry(&current_layer, &font);
    draw_immitancemetry(&current_layer, &font);
    draw_footer(&current_layer, &font);
    draw_main_borders(&current_layer);

    // Save the PDF
    let file = File::create("audiogram_layout.pdf")?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer)?;

    println!("Successfully created audiogram_layout.pdf");
    Ok(())
}

fn draw_header(layer: &PdfLayerReference, font: &IndirectFontRef) {
    // --- Top Left: Evaluation Details ---
    let y_pos = 280.0;
    layer.use_text(
        "Date de l'évaluation: 2025-06-27",
        FONT_SIZE_NORMAL,
        Mm(MARGIN),
        Mm(y_pos),
        &font,
    );
    layer.use_text(
        "Lieu de l'évaluation:",
        FONT_SIZE_NORMAL,
        Mm(MARGIN),
        Mm(y_pos - 5.0),
        &font,
    );

    // --- Center: Patient and Audiologist ---
    let center_x = PAGE_WIDTH / 2.0;
    layer.begin_text_section();
    layer.set_font(&font, FONT_SIZE_LARGE_TITLE);
    layer.set_text_cursor(Mm(center_x - 25.0), Mm(y_pos));
    layer.write_text("ROXANNE BOLDUC", &font);
    layer.set_font(&font, FONT_SIZE_NORMAL);
    layer.set_text_cursor(Mm(center_x - 35.0), Mm(y_pos - 6.0));
    layer.write_text("Jane DOE, 1954-06-10 (71 ans)", &font);
    layer.set_text_cursor(Mm(center_x - 15.0), Mm(y_pos - 10.0));
    layer.write_text("AUDIOLOGISTE", &font);
    layer.end_text_section();

    // --- Top Right: Tympanogram Snippet ---
    draw_simple_grid(
        layer, 160.0, 275.0, 40.0, 15.0, 4, 3, "-200", "200", "0ml", "2.5",
    );
}

fn draw_tonal_audiometry(layer: &PdfLayerReference, font: &IndirectFontRef) {
    let y_start = 255.0;
    let grid_height = 70.0;
    let grid_width_single = 80.0;
    let center_x = PAGE_WIDTH / 2.0;

    // --- Main Title ---
    layer.use_text(
        "AUDIOMÉTRIE TONALE",
        FONT_SIZE_TITLE,
        Mm(center_x - 20.0),
        Mm(y_start),
        font,
    );

    // --- Ear Titles ---
    layer.use_text(
        "OREILLE DROITE",
        FONT_SIZE_NORMAL,
        Mm(center_x - 55.0),
        Mm(y_start - 7.0),
        font,
    );
    layer.use_text(
        "OREILLE GAUCHE",
        FONT_SIZE_NORMAL,
        Mm(center_x + 25.0),
        Mm(y_start - 7.0),
        font,
    );

    let grid_y = y_start - 10.0;

    // --- Grids ---
    // Frequencies (X-axis)
    let freqs = ["125", "250", "500", "1000", "2000", "4000", "8000 Hz"];
    let x_spacing = grid_width_single / (freqs.len() - 1) as f32;

    // Right Ear Grid (left side of page)
    let left_grid_start_x = center_x - grid_width_single - 10.0;
    for (i, freq) in freqs.iter().enumerate() {
        let x = left_grid_start_x + (i as f32 * x_spacing);
        layer.use_text(*freq, FONT_SIZE_SMALL, Mm(x - 2.0), Mm(grid_y + 2.0), font);
        let line = Line {
            points: vec![
                (Point::new(Mm(x), Mm(grid_y)), false),
                (Point::new(Mm(x), Mm(grid_y - grid_height)), false),
            ],
            is_closed: false,
        };
        layer.add_line(line);
    }

    // Left Ear Grid (right side of page)
    let right_grid_start_x = center_x + 10.0;
    for (i, freq) in freqs.iter().enumerate() {
        let x = right_grid_start_x + (i as f32 * x_spacing);
        layer.use_text(*freq, FONT_SIZE_SMALL, Mm(x - 2.0), Mm(grid_y + 2.0), font);
        let line = Line {
            points: vec![
                (Point::new(Mm(x), Mm(grid_y)), false),
                (Point::new(Mm(x), Mm(grid_y - grid_height)), false),
            ],
            is_closed: false,
        };
        layer.add_line(line);
    }

    // dBHL (Y-axis) and horizontal lines
    let y_spacing = grid_height / 13.0; // 14 labels from -10 to 120
    for i in 0..14 {
        let y = grid_y - (i as f32 * y_spacing);
        let db_label = -10 + (i * 10);
        layer.use_text(
            format!("{}", db_label),
            FONT_SIZE_SMALL,
            Mm(center_x - 4.0),
            Mm(y - 1.0),
            font,
        );
        let line = Line {
            points: vec![
                (Point::new(Mm(left_grid_start_x), Mm(y)), false),
                (
                    Point::new(Mm(right_grid_start_x + grid_width_single), Mm(y)),
                    false,
                ),
            ],
            is_closed: false,
        };
        layer.add_line(line);
    }
    layer.use_text(
        "dBHL",
        FONT_SIZE_SMALL,
        Mm(center_x - 4.0),
        Mm(grid_y + 2.0),
        font,
    );

    // --- Legend in the Center ---
    let legend_x = center_x - 10.0;
    let mut legend_y = grid_y - 5.0;

    // Helper to draw a legend item
    fn draw_legend_item(
        layer: &PdfLayerReference,
        font: &IndirectFontRef,
        x: f32,
        y: &mut f32,
        symbol: &str,
        text: &str,
    ) {
        layer.use_text(symbol, FONT_SIZE_SYMBOL, Mm(x), Mm(*y), font);
        layer.use_text(text, FONT_SIZE_SMALL, Mm(x + 5.0), Mm(*y), font);
        *y -= 4.0;
    }

    layer.use_text(
        "DROITE",
        FONT_SIZE_SMALL,
        Mm(legend_x - 18.0),
        Mm(legend_y),
        font,
    );
    layer.use_text(
        "GAUCHE",
        FONT_SIZE_SMALL,
        Mm(legend_x + 5.0),
        Mm(legend_y),
        font,
    );
    legend_y -= 5.0;

    layer.use_text(
        "SEUIL AÉRIEN",
        FONT_SIZE_SMALL,
        Mm(legend_x - 10.0),
        Mm(legend_y),
        font,
    );
    legend_y -= 5.0;

    // Using text for simple symbols
    draw_legend_item(
        layer,
        font,
        legend_x - 18.0,
        &mut legend_y,
        "O",
        "Non masqué INTRA",
    );
    draw_legend_item(
        layer,
        font,
        legend_x + 5.0,
        &mut legend_y,
        "X",
        "Non masqué SUPRA",
    );

    let mut legend_y2 = legend_y + 8.0;
    draw_legend_item(
        layer,
        font,
        legend_x - 18.0,
        &mut legend_y2,
        "Δ",
        "Masqué INTRA",
    );
    draw_legend_item(
        layer,
        font,
        legend_x + 5.0,
        &mut legend_y2,
        "□",
        "Masqué SUPRA",
    );

    legend_y -= 5.0;
    layer.use_text(
        "SEUIL OSSEUX",
        FONT_SIZE_SMALL,
        Mm(legend_x - 10.0),
        Mm(legend_y),
        font,
    );
    legend_y -= 5.0;
    // ... continue for all other legend items ...
    draw_legend_item(
        layer,
        font,
        legend_x - 18.0,
        &mut legend_y,
        "<",
        "Non masqué",
    );
    draw_legend_item(layer, font, legend_x - 18.0, &mut legend_y, "[", "Masqué");
}

fn draw_evaluation_details(layer: &PdfLayerReference, font: &IndirectFontRef) {
    let y_start = 175.0;
    let x_start = MARGIN;

    // --- Validity & Stimuli ---
    layer.use_text("VALIDITÉ", FONT_SIZE_SMALL, Mm(x_start), Mm(y_start), font);
    draw_checkbox(layer, font, x_start, y_start - 5.0, "Bonne", true);
    draw_checkbox(layer, font, x_start, y_start - 10.0, "Moyenne", false);
    draw_checkbox(layer, font, x_start, y_start - 15.0, "Nulle", false);

    layer.use_text(
        "STIMULI",
        FONT_SIZE_SMALL,
        Mm(x_start + 30.0),
        Mm(y_start),
        font,
    );
    draw_checkbox(
        layer,
        font,
        x_start + 30.0,
        y_start - 5.0,
        "Sons purs",
        true,
    );
    draw_checkbox(
        layer,
        font,
        x_start + 30.0,
        y_start - 10.0,
        "Sons hululés",
        false,
    );

    // --- Evaluation Method ---
    layer.use_text(
        "MÉTHODE D'ÉVALUATION",
        FONT_SIZE_SMALL,
        Mm(x_start + 70.0),
        Mm(y_start),
        font,
    );
    draw_checkbox(
        layer,
        font,
        x_start + 70.0,
        y_start - 5.0,
        "Hughton-Westlake modifiée",
        true,
    );

    // --- Pure Tone Averages ---
    let y_pta = y_start - 20.0;
    layer.use_text(
        "MOYENNES DES SONS PURS - dB HL",
        FONT_SIZE_NORMAL,
        Mm(x_start),
        Mm(y_pta),
        font,
    );

    // Right Ear Box
    layer.add_rect(Rect::new(
        Mm(x_start),
        Mm(y_pta - 15.0),
        Mm(x_start + 80.0),
        Mm(y_pta - 2.0),
    ));
    layer.use_text(
        "MSP3: 43   MSP4: 50.00   FLCH: 35",
        FONT_SIZE_NORMAL,
        Mm(x_start + 2.0),
        Mm(y_pta - 10.0),
        font,
    );

    // Left Ear Box
    layer.add_rect(Rect::new(
        Mm(x_start + 100.0),
        Mm(y_pta - 15.0),
        Mm(x_start + 180.0),
        Mm(y_pta - 2.0),
    ));
    layer.use_text(
        "MSP3: 50   MSP4: 56.25   FLCH: 40",
        FONT_SIZE_NORMAL,
        Mm(x_start + 102.0),
        Mm(y_pta - 10.0),
        font,
    );
}

fn draw_speech_audiometry(layer: &PdfLayerReference, font: &IndirectFontRef) {
    let y_start = 145.0;
    let x_center = PAGE_WIDTH / 2.0;

    layer.use_text(
        "AUDIOMÉTRIE VOCALE",
        FONT_SIZE_TITLE,
        Mm(x_center - 22.0),
        Mm(y_start),
        font,
    );

    // Draw two identical tables for right and left ears
    draw_speech_table(layer, font, MARGIN, y_start - 5.0, "OREILLE DROITE");
    draw_speech_table(layer, font, x_center + 2.0, y_start - 5.0, "OREILLE GAUCHE");
}

fn draw_immitancemetry(layer: &PdfLayerReference, font: &IndirectFontRef) {
    let y_start = 85.0;
    let x_center = PAGE_WIDTH / 2.0;

    layer.use_text(
        "IMMITANCEMÉTRIE",
        FONT_SIZE_TITLE,
        Mm(x_center - 20.0),
        Mm(y_start),
        font,
    );

    draw_immitance_panel(layer, font, MARGIN, y_start - 5.0, "OREILLE DROITE");
    draw_immitance_panel(layer, font, x_center + 2.0, y_start - 5.0, "OREILLE GAUCHE");
}

fn draw_footer(layer: &PdfLayerReference, font: &IndirectFontRef) {
    let y_start = 25.0;

    // Notes section
    layer.use_text("Notes:", FONT_SIZE_NORMAL, Mm(MARGIN), Mm(y_start), font);
    let line = Line {
        points: vec![
            (Point::new(Mm(MARGIN + 12.0), Mm(y_start - 1.0)), false),
            (Point::new(Mm(120.0), Mm(y_start - 1.0)), false),
        ],
        is_closed: false,
    };
    layer.add_line(line);

    // Signature area
    let sig_x = 140.0;
    layer.use_text(
        "MEMBRE",
        FONT_SIZE_SMALL,
        Mm(sig_x + 8.0),
        Mm(y_start),
        font,
    );
    layer.add_rect(Rect::new(
        Mm(sig_x),
        Mm(y_start - 15.0),
        Mm(sig_x + 30.0),
        Mm(y_start - 2.0),
    )); // Logo placeholder
    layer.use_text(
        "Roxanne Bolduc, MPA",
        FONT_SIZE_NORMAL,
        Mm(sig_x),
        Mm(y_start - 20.0),
        font,
    );
    layer.use_text(
        "Audiologiste #1234",
        FONT_SIZE_NORMAL,
        Mm(sig_x),
        Mm(y_start - 25.0),
        font,
    );
}

// ---- HELPER FUNCTIONS ----

fn draw_main_borders(layer: &PdfLayerReference) {
    // A simple border around major sections can be added here if desired
    // Example: Border around the tonal audiometry section
    layer.add_rect(Rect::new(
        Mm(MARGIN),
        Mm(180.0),
        Mm(PAGE_WIDTH - MARGIN),
        Mm(260.0),
    ));
    // Add other borders as needed...
}

fn draw_simple_grid(
    layer: &PdfLayerReference,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    v_lines: usize,
    h_lines: usize,
    x_min: &str,
    x_max: &str,
    y_min: &str,
    y_max: &str,
) {
    // Boundary
    layer.add_rect(Rect::new(Mm(x), Mm(y), Mm(x + w), Mm(y + h)));

    // Vertical Lines
    for i in 1..v_lines {
        let lx = x + (i as f32 * w / v_lines as f32);
        let line = Line {
            points: vec![
                (Point::new(Mm(lx), Mm(y)), false),
                (Point::new(Mm(lx), Mm(y + h)), false),
            ],
            is_closed: false,
        };
        layer.add_line(line);
    }
    // Horizontal Lines
    for i in 1..h_lines {
        let ly = y + (i as f32 * h / h_lines as f32);
        let line = Line {
            points: vec![
                (Point::new(Mm(x), Mm(ly)), false),
                (Point::new(Mm(x + w), Mm(ly)), false),
            ],
            is_closed: false,
        };
        layer.add_line(line);
    }
}

fn draw_checkbox(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    x: f32,
    y: f32,
    text: &str,
    checked: bool,
) {
    layer.add_rect(Rect::new(Mm(x), Mm(y), Mm(x + 3.0), Mm(y + 3.0)));
    if checked {
        // Draw an 'X'
        let line1 = Line {
            points: vec![
                (Point::new(Mm(x), Mm(y)), false),
                (Point::new(Mm(x + 3.0), Mm(y + 3.0)), false),
            ],
            is_closed: false,
        };
        let line2 = Line {
            points: vec![
                (Point::new(Mm(x), Mm(y + 3.0)), false),
                (Point::new(Mm(x + 3.0), Mm(y)), false),
            ],
            is_closed: false,
        };
        layer.add_line(line1);
        layer.add_line(line2);
    }
    layer.use_text(text, FONT_SIZE_NORMAL, Mm(x + 5.0), Mm(y), font);
}

fn draw_speech_table(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    x: f32,
    y: f32,
    title: &str,
) {
    layer.use_text(title, FONT_SIZE_NORMAL, Mm(x + 30.0), Mm(y), font);
    // This is a placeholder. A full implementation would draw the full table grid and labels.
    layer.add_rect(Rect::new(Mm(x), Mm(y - 45.0), Mm(x + 90.0), Mm(y + 5.0)));
    layer.use_text(
        "SEUILS VOCAUX - dB HL",
        FONT_SIZE_SMALL,
        Mm(x + 2.0),
        Mm(y - 5.0),
        font,
    );
    layer.use_text(
        "IDENTIFICATION PAROLE",
        FONT_SIZE_SMALL,
        Mm(x + 2.0),
        Mm(y - 20.0),
        font,
    );
}

fn draw_immitance_panel(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    x: f32,
    y: f32,
    title: &str,
) {
    layer.use_text(title, FONT_SIZE_NORMAL, Mm(x + 30.0), Mm(y), font);
    // Placeholder for the full immitance panel
    layer.add_rect(Rect::new(Mm(x), Mm(y - 45.0), Mm(x + 90.0), Mm(y + 5.0)));
    layer.use_text(
        "TYMPANOMÉTRIE",
        FONT_SIZE_SMALL,
        Mm(x + 2.0),
        Mm(y - 5.0),
        font,
    );
    draw_simple_grid(
        layer,
        x + 2.0,
        y - 25.0,
        40.0,
        15.0,
        4,
        3,
        "-200",
        "200",
        "0",
        "2.5",
    );
    layer.use_text(
        "RÉFLEXE STAPÉDIEN - dB",
        FONT_SIZE_SMALL,
        Mm(x + 2.0),
        Mm(y - 30.0),
        font,
    );
}
