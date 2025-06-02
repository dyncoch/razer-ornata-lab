// Razer Ornata V3 RGB Control

use eframe::{egui, egui_glow};
use razer_rgb_mac::emojis::*;
use razer_rgb_mac::razer_report::RazerReport;
use rusb::{Context, DeviceHandle, UsbContext};
use std::time::Duration;

const RAZER_VENDOR_ID: u16 = 0x1532;
const ORNATA_V3_PRODUCT_ID: u16 = 0x02A1;

fn find_device() -> Option<DeviceHandle<Context>> {
    let context = Context::new().ok()?;
    for device in context.devices().ok()?.iter() {
        let device_desc = device.device_descriptor().ok()?;
        if device_desc.vendor_id() == RAZER_VENDOR_ID
            && device_desc.product_id() == ORNATA_V3_PRODUCT_ID
        {
            println!(
                "{EMOJI_TARGET} Found keyboard (Vendor ID: 0x{:04X}, Product ID: 0x{:04X})",
                device_desc.vendor_id(),
                device_desc.product_id()
            );
            return device.open().ok();
        }
    }
    None
}

fn main() -> Result<(), eframe::Error> {
    println!("{EMOJI_LAMP} Razer RGB Control");
    println!("========================================================");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(500.0, 550.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Razer RGB MacOS",
        options,
        Box::new(|_cc| Ok(Box::new(RazerRGBMac::default()))),
    )
}

struct RazerRGBMac {
    device_handle: Option<DeviceHandle<Context>>,
    device_status: String,
}

impl Default for RazerRGBMac {
    fn default() -> Self {
        let (handle, status) = match find_device() {
            Some(handle) => (Some(handle), "Razer Ornata v3".to_string()),
            None => (None, "No device found".to_string()),
        };

        Self {
            device_handle: handle,
            device_status: status,
        }
    }
}

impl RazerRGBMac {
    fn static_color(&mut self, r: u8, g: u8, b: u8) {
        if let Some(ref mut handle) = self.device_handle {
            let cmd = RazerReport::static_rgb(r, g, b);
            if cmd.send(handle) {
                println!("   {EMOJI_CHECK} Color set successfully.");
            } else {
                println!("   {EMOJI_CROSS} Failed to set color.");
            }
            println!("{EMOJI_SUCCESS} RGB Control Done!");
        } else {
            println!("{EMOJI_WRONG_WAY} No device connected.");
        }
    }

    fn breathing(&mut self, r: u8, g: u8, b: u8) {
        if let Some(ref mut handle) = self.device_handle {
            println!("{EMOJI_PAINT} Setting breathing");

            let cmd = RazerReport::breathing(r, g, b);
            if cmd.send(handle) {
                println!("   {EMOJI_CHECK} Color set successfully.");
            } else {
                println!("   {EMOJI_CROSS} Failed to set color.");
            }
        } else {
            println!("{EMOJI_WRONG_WAY} No device connected.");
        }
    }

    fn spectrum(&mut self) {
        if let Some(ref mut handle) = self.device_handle {
            println!("{EMOJI_PAINT} Setting spectrum");

            let cmd = RazerReport::spectrum();
            if cmd.send(handle) {
                println!("   {EMOJI_CHECK} Color set successfully.");
            } else {
                println!("   {EMOJI_CROSS} Failed to set color.");
            }
        } else {
            println!("{EMOJI_WRONG_WAY} No device connected.");
        }
    }

    fn wave(&mut self, direction: u8, speed: u8) {
        if let Some(ref mut handle) = self.device_handle {
            println!("{EMOJI_PAINT} Setting wave");

            let cmd = RazerReport::wave(direction, speed);
            if cmd.send(handle) {
                println!("   {EMOJI_CHECK} Color set successfully.");
            } else {
                println!("   {EMOJI_CROSS} Failed to set color.");
            }
        } else {
            println!("{EMOJI_WRONG_WAY} No device connected.");
        }
    }
}

impl eframe::App for RazerRGBMac {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Title
            ui.heading("oRazer RGB Control");
            ui.separator();

            // Status
            ui.horizontal(|ui| {
                ui.label("Status:");

                // Indicator circle
                let (rect, _reponse) =
                    ui.allocate_exact_size(egui::Vec2 { x: 12.0, y: 12.0 }, egui::Sense::hover());

                let status_color = if self.device_handle.is_none() {
                    egui::Color32::RED
                } else {
                    egui::Color32::GREEN
                };
                ui.painter().circle_filled(rect.center(), 6.0, status_color);

                ui.label(&self.device_status);
            });

            ui.add_space(20.0);

            // Colour selection
            ui.horizontal(|ui| {
                ui.label("Set static color:");
                if ui.button("Green").clicked() {
                    self.static_color(0, 255, 0);
                }
                if ui.button("Blue").clicked() {
                    self.static_color(0, 0, 255);
                }
                if ui.button("Red").clicked() {
                    self.static_color(255, 0, 0);
                }
            });

            // Breathing selection
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Set breathing effect:");

                if ui.button("Green").clicked() {
                    self.breathing(0, 255, 0);
                }
                if ui.button("Blue").clicked() {
                    self.breathing(0, 0, 255);
                }
                if ui.button("Red").clicked() {
                    self.breathing(255, 0, 0);
                }
            });

            // Set Spectrum
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Set other effect:");

                if ui.button("Spectrum").clicked() {
                    self.spectrum();
                }

                if ui.button("Wave").clicked() {
                    self.wave(0x00, 0x01);
                }
            });
        });
    }
}
