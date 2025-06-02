// Razer Ornata V3 RGB Control

use eframe::egui;
use razer_rgb_mac::emojis::*;
use razer_rgb_mac::razer_report::RazerReport;
use rusb::{Context, DeviceHandle, UsbContext};

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
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(500.0, 600.0)),
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
    show_about: bool,
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
            show_about: false,
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

    fn render_section<F, R>(ui: &mut egui::Ui, title: &str, content: F) -> R
    where
        F: FnOnce(&mut egui::Ui) -> R,
    {
        egui::Frame::new()
            .fill(egui::Color32::from_gray(35))
            .corner_radius(10.0)
            .inner_margin(20.0)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(60)))
            .show(ui, |ui| {
                let section_title = egui::RichText::new(title)
                    .size(18.0)
                    .color(egui::Color32::from_rgb(200, 200, 255));
                ui.label(section_title);
                ui.add_space(15.0);
                content(ui)
            })
            .inner
    }
}

impl eframe::App for RazerRGBMac {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set a dark theme
        ctx.set_visuals(egui::Visuals::dark());

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);

            // Title
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                let title = egui::RichText::new(&format!("{EMOJI_GAMEPAD} Razer RGB Control"))
                    .size(28.0)
                    .color(egui::Color32::from_rgb(0, 255, 100));
                ui.label(title);
                ui.add_space(5.0);

                let subtitle = egui::RichText::new("Ornata V3 Controller")
                    .size(14.0)
                    .color(egui::Color32::GRAY);
                ui.label(subtitle);
            });

            ui.add_space(10.0);

            // Status section
            egui::Frame::new()
                .fill(egui::Color32::from_gray(40))
                .corner_radius(8.0)
                .inner_margin(15.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Status indicator
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());

                        let status_color = if self.device_handle.is_some() {
                            egui::Color32::from_rgb(0, 255, 100)
                        } else {
                            egui::Color32::from_rgb(255, 80, 80)
                        };

                        ui.painter().circle_filled(rect.center(), 8.0, status_color);

                        ui.add_space(10.0);

                        let status_text = egui::RichText::new(&self.device_status)
                            .size(16.0)
                            .color(egui::Color32::WHITE);
                        ui.label(status_text);
                    });
                });

            ui.add_space(10.0);

            // Static color section
            let (green_clicked, blue_clicked, red_clicked) =
                Self::render_section(ui, "🎨 Static Colors", |ui| {
                    ui.columns(3, |cols| {
                        let green_clicked = cols[0]
                            .add_sized(
                                [80.0, 40.0],
                                egui::Button::new(egui::RichText::new("Green").size(14.0))
                                    .fill(egui::Color32::from_rgb(40, 120, 40)),
                            )
                            .clicked();

                        let blue_clicked = cols[1]
                            .add_sized(
                                [80.0, 40.0],
                                egui::Button::new(egui::RichText::new("Blue").size(14.0))
                                    .fill(egui::Color32::from_rgb(40, 40, 120)),
                            )
                            .clicked();

                        let red_clicked = cols[2]
                            .add_sized(
                                [80.0, 40.0],
                                egui::Button::new(egui::RichText::new("Red").size(14.0))
                                    .fill(egui::Color32::from_rgb(120, 40, 40)),
                            )
                            .clicked();

                        (green_clicked, blue_clicked, red_clicked)
                    })
                });

            if green_clicked {
                self.static_color(0, 255, 0);
            }
            if blue_clicked {
                self.static_color(0, 0, 255);
            }
            if red_clicked {
                self.static_color(255, 0, 0);
            }

            ui.add_space(10.0);

            // Breathing effects section
            let (green_breathing, blue_breathing, red_breathing) =
                Self::render_section(ui, &format!("{EMOJI_PUFF} Breathing Effects"), |ui| {
                    ui.columns(3, |cols| {
                        let green_clicked = cols[0]
                            .add_sized(
                                [80.0, 40.0],
                                egui::Button::new(egui::RichText::new("Green").size(14.0))
                                    .fill(egui::Color32::from_rgb(30, 90, 30)),
                            )
                            .clicked();

                        let blue_clicked = cols[1]
                            .add_sized(
                                [80.0, 40.0],
                                egui::Button::new(egui::RichText::new("Blue").size(14.0))
                                    .fill(egui::Color32::from_rgb(30, 30, 90)),
                            )
                            .clicked();

                        let red_clicked = cols[2]
                            .add_sized(
                                [80.0, 40.0],
                                egui::Button::new(egui::RichText::new("Red").size(14.0))
                                    .fill(egui::Color32::from_rgb(90, 30, 30)),
                            )
                            .clicked();

                        (green_clicked, blue_clicked, red_clicked)
                    })
                });

            if green_breathing {
                self.breathing(0, 255, 0);
            }
            if blue_breathing {
                self.breathing(0, 0, 255);
            }
            if red_breathing {
                self.breathing(255, 0, 0);
            }

            ui.add_space(10.0);

            // Special effects section
            let (spectrum_clicked, wave_clicked) =
                Self::render_section(ui, &format!("{EMOJI_STARS} Special Effects"), |ui| {
                    ui.horizontal(|ui| {
                        let spectrum_clicked = ui
                            .add_sized(
                                [120.0, 45.0],
                                egui::Button::new(
                                    egui::RichText::new(&format!("{EMOJI_RAINBOW} Spectrum"))
                                        .size(14.0),
                                )
                                .fill(egui::Color32::from_rgb(80, 40, 120)),
                            )
                            .clicked();

                        ui.add_space(20.0);

                        let wave_clicked = ui
                            .add_sized(
                                [120.0, 45.0],
                                egui::Button::new(
                                    egui::RichText::new(&format!("{EMOJI_WAVE} Wave")).size(14.0),
                                )
                                .fill(egui::Color32::from_rgb(40, 80, 120)),
                            )
                            .clicked();

                        (spectrum_clicked, wave_clicked)
                    })
                })
                .inner;

            if spectrum_clicked {
                self.spectrum();
            }
            if wave_clicked {
                self.wave(0x00, 0x01);
            }
        });

        // About window (shows when button is clicked)
        if self.show_about {
            egui::Window::new("About Razer RGB Control")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new(&format!("{EMOJI_GAMEPAD} Razer RGB Control"))
                                .size(20.0),
                        );
                        ui.add_space(10.0);
                        ui.label("Version 0.1.0");
                        ui.label("Control your Razer Ornata V3 keyboard lighting");
                        ui.add_space(15.0);
                        ui.label("@author Lucas F.Martins");
                        ui.add_space(15.0);
                        if ui.button("Close").clicked() {
                            self.show_about = false;
                        }
                    });
                });
        }
    }
}
