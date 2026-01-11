use egui::{
    FontFamily,
    epaint::text::{FontPriority, InsertFontFamily},
};

const FONTS: &[(&str, &[u8], &[InsertFontFamily])] = &[
    (
        "ipag",
        include_bytes!("fonts/ipag.ttf"),
        &[InsertFontFamily {
            family: FontFamily::Monospace,
            priority: FontPriority::Lowest,
        }],
    ),
    (
        "ipagp",
        include_bytes!("fonts/ipagp.ttf"),
        &[InsertFontFamily {
            family: FontFamily::Proportional,
            priority: FontPriority::Lowest,
        }],
    ),
];

pub fn install(egui_ctx: &egui::Context) {
    for &font in FONTS.iter() {
        let (name, data, families) = font;
        egui_ctx.add_font(egui::epaint::text::FontInsert::new(
            name,
            egui::FontData::from_static(data),
            families.into(),
        ));
    }
}
