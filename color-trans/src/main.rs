#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod color;

use std::sync::Arc;

use color::Color;
use eframe::egui::{self, FontDefinitions};

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_decorations(true)
      .with_inner_size([400.0, 100.0])
      .with_transparent(true),
    ..eframe::NativeOptions::default()
  };
  eframe::run_native(
    "Hello egui",
    options,
    Box::new(|cc| {
      add_chinese(&cc.egui_ctx);
      Ok(Box::new(MyApp::default()))
    }),
  )
}

struct MyApp {
  color: Color,
  update_by_hex_error: Option<String>,
  hex: String,
}
impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("hello egui");

      ui.horizontal(|ui| {
        ui.add(egui::TextEdit::singleline(&mut self.hex).desired_width(100.0))
          .highlight();

        if ui.button("转换").clicked() {
          self.update_by_hex_error = self.color.update(&self.hex).err();
        }

        if let Some(msg) = &self.update_by_hex_error {
          ui.label(egui::RichText::new(format!("{}", msg)).color(egui::Color32::RED));
        } else {
          ui.label(format!("{}", self.color.rgba));
        }
      });
    });
  }
}

fn add_chinese(ctx: &egui::Context) {
  let mut fonts = FontDefinitions::default();

  fonts.font_data.insert(
    "alibaba_puhui".to_owned(),
    Arc::new(egui::FontData::from_static(include_bytes!(
      "./AlibabaPuHuiTi-3-55-Regular.ttf"
    ))),
  );

  fonts
    .families
    .get_mut(&egui::FontFamily::Proportional)
    .unwrap()
    .insert(0, "alibaba_puhui".to_owned());

  fonts
    .families
    .get_mut(&egui::FontFamily::Monospace)
    .unwrap()
    .insert(0, "alibaba_puhui".to_owned());

  ctx.set_fonts(fonts);
}
impl Default for MyApp {
  fn default() -> Self {
    let color = Color::default();
    let hex = color.hex.clone();
    Self {
      color,
      update_by_hex_error: None,
      hex,
    }
  }
}
