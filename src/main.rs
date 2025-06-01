// Razer Ornata V3 RGB Control

use rusb::{Context, DeviceHandle, UsbContext};
use std::time::Duration;
mod razer_report;
use razer_report::RazerReport;

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
                "🎯 Found keyboard (Vendor ID: 0x{:04X}, Product ID: 0x{:04X})",
                device_desc.vendor_id(),
                device_desc.product_id()
            );
            return device.open().ok();
        }
    }
    None
}

fn main() {
    println!("💡 Razer RGB Control");
    println!("========================================================");

    match find_device() {
        Some(mut handle) => {
            println!("✅ Connected to Ornata V3");

            println!("\n🔴 Setting static red color...");
            let red_cmd = RazerReport::static_rgb(0xFF, 0x00, 0x00);
            if red_cmd.send(&mut handle) {
                println!("   Keyboard should be RED.");
            } else {
                println!("   Failed to set red color (or command not supported/failed).");
            }
            std::thread::sleep(Duration::from_secs(3));

            println!("\n🟢 Setting static green color...");
            let green_cmd = RazerReport::static_rgb(0x00, 0xFF, 0x00);
            if green_cmd.send(&mut handle) {
                println!("   Keyboard should be GREEN.");
            } else {
                println!("   Failed to set green color.");
            }
            std::thread::sleep(Duration::from_secs(3));

            println!("\n🔵 Setting static blue color...");
            let blue_cmd = RazerReport::static_rgb(0x00, 0x00, 0xFF);
            if blue_cmd.send(&mut handle) {
                println!("   Keyboard should be BLUE.");
            } else {
                println!("   Failed to set blue color.");
            }
            std::thread::sleep(Duration::from_secs(3));

            println!("\n🌈 Setting spectrum cycling...");
            let spectrum_cmd = RazerReport::spectrum();
            if spectrum_cmd.send(&mut handle) {
                println!("   Keyboard should be in SPECTRUM mode.");
            } else {
                println!("   Failed to set spectrum mode.");
            }
            std::thread::sleep(Duration::from_secs(5));

            println!("\n💜 Setting purple breathing effect...");
            let breathing_cmd = RazerReport::breathing(0xFF, 0x00, 0xFF);
            if breathing_cmd.send(&mut handle) {
                println!("   Keyboard should be BREATHING PURPLE.");
            } else {
                println!("   Failed to set breathing mode.");
            }
            std::thread::sleep(Duration::from_secs(5));

            println!("\n🌊 Setting wave effect (direction 0, speed 0x01)...");
            let wave_cmd = RazerReport::wave(0, 0x01);
            if wave_cmd.send(&mut handle) {
                println!("   Keyboard should be in WAVE mode.");
            } else {
                println!("   Failed to set wave mode.");
            }

            println!("\n🎉 RGB Control Test Complete!");
        }
        None => {
            println!("❌ Could not find or connect to Ornata V3.");
        }
    }
}
