mod egui_fonts;

use libsourceselector::{SerdeJson, Source, Sources};
use std::cell::Cell;

const ICON_SIZE: f32 = 24.0;

struct SourceSelector {
    monitor_sources: Sources,
    window_sources: Sources,
    selected_source: Cell<Option<Source>>,
}

impl SourceSelector {
    fn new(monitor_sources: Sources, window_sources: Sources) -> Self {
        Self {
            monitor_sources,
            window_sources,
            selected_source: Cell::new(None),
        }
    }

    fn add_source_table(
        &self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        id_salt: &str,
        sources: &Sources,
    ) {
        egui_extras::TableBuilder::new(ui)
            .id_salt(id_salt)
            .sense(egui::Sense::click())
            .column(egui_extras::Column::remainder())
            .body(|mut body| {
                for source in sources.iter() {
                    body.row(ICON_SIZE, |mut row| {
                        let (_, response) = row.col(|ui| {
                            ui.horizontal(|ui| Self::add_source_row_content(ui, source));
                        });
                        if response.clicked() {
                            self.selected_source.set(Some(source.to_owned()));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }
            });
    }

    fn add_source_row_content(ui: &mut egui::Ui, source: &Source) {
        let add_row_content = |ui: &mut egui::Ui, name, image| {
            const IMAGE_SIZE: egui::Vec2 = egui::Vec2 {
                x: ICON_SIZE,
                y: ICON_SIZE,
            };
            ui.add(egui::widgets::Image::new(image).fit_to_exact_size(IMAGE_SIZE));
            ui.add(egui::widgets::Label::new(name).selectable(false));
        };
        match source {
            Source::Monitor { monitor_name } => {
                let image = egui::include_image!("icons/video-display-symbolic.svg");
                add_row_content(ui, monitor_name, image);
            }
            Source::Window {
                window_name,
                icon_path,
                ..
            } => {
                let image = if let Some(icon_path) = icon_path {
                    egui::ImageSource::Uri(icon_path.into())
                } else {
                    egui::include_image!("icons/preferences-system-symbolic.svg")
                };
                add_row_content(ui, window_name, image);
            }
        }
    }
}

impl eframe::App for SourceSelector {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Monitors").heading());
                self.add_source_table(ctx, ui, "monitor-table", &self.monitor_sources);
                ui.separator();
                ui.label(egui::RichText::new("Windows").heading());
                self.add_source_table(ctx, ui, "window-table", &self.window_sources);
            });
        });
    }

    fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
        let Some(selected_source) = self.selected_source.take() else {
            eprintln!("Source wasn't selected");
            return;
        };
        match selected_source.to_json() {
            Ok(json) => println!("{json}"),
            Err(err) => eprintln!("SourceSelector error: {err}"),
        }
    }
}

pub fn run(
    monitor_sources: Sources,
    window_sources: Sources,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Select Capture Source",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            egui_fonts::install(&cc.egui_ctx);
            cc.egui_ctx.set_zoom_factor(1.3);
            Ok(Box::new(SourceSelector::new(
                monitor_sources,
                window_sources,
            )))
        }),
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        let progname = if let Some(first_arg) = args.first() {
            first_arg.split('/').next_back()
        } else {
            None
        };
        return Err(format!(
            "Usage: {} [MONITOR SOURCES] [WINDOW SOURCES]",
            progname.unwrap_or("sourceselector-ui")
        )
        .into());
    }
    let monitor_sources = &args[1];
    let window_sources = &args[2];
    run(
        Sources::from_json(monitor_sources)?,
        Sources::from_json(window_sources)?,
    )
}
