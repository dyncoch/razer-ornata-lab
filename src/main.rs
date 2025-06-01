// Razer Ornata V3 RGB Control

use rusb::{Context, DeviceHandle, UsbContext};
use std::time::Duration;
mod razer_report;
use razer_report::RazerReport;

const RAZER_VENDOR_ID: u16 = 0x1532;
const ORNATA_V3_PRODUCT_ID: u16 = 0x02A1;

fn find_ornata_v3() -> Option<DeviceHandle<Context>> {
    let context = Context::new().ok()?;
    for device in context.devices().ok()?.iter() {
        let device_desc = device.device_descriptor().ok()?;
        if device_desc.vendor_id() == RAZER_VENDOR_ID
            && device_desc.product_id() == ORNATA_V3_PRODUCT_ID
        {
            println!(
                "🎯 Found Ornata V3 (Vendor ID: 0x{:04X}, Product ID: 0x{:04X})",
                device_desc.vendor_id(),
                device_desc.product_id()
            );
            return device.open().ok();
        }
    }
    None
}

// fn send_ornata_v3_command(handle: &mut DeviceHandle<Context>, command: &RazerReport) -> bool {
//     // Ensure the command struct is indeed 90 bytes
//     assert_eq!(
//         std::mem::size_of::<RazerReport>(),
//         90,
//         "RazerReport struct size is not 90 bytes!"
//     );

//     let command_bytes = unsafe { std::slice::from_raw_parts(command as *const _ as *const u8, 90) };

//     println!("📤 Sending Ornata V3 command:");
//     println!("   Transaction ID: 0x{:02X}", command.transaction_id);
//     println!(
//         "   Class: 0x{:02X}, ID: 0x{:02X}",
//         command.command_class, command.command_id
//     );
//     println!("   Data Size: 0x{:02X}", command.data_size);
//     print!("   Arguments (first 10 of 80): ");
//     for i in 0..std::cmp::min(10, command.arguments.len()) {
//         print!("{:02X} ", command.arguments[i]);
//     }
//     println!("\n   CRC: 0x{:02X}", command.crc);

//     match handle.write_control(
//         0x21,   // bmRequestType (Host to Device, Class, Interface)
//         0x09,   // bRequest (SET_REPORT)
//         0x0300, // wValue (Feature Report, ID 0x00)
//         0x02,   // wIndex (Interface 2)
//         command_bytes,
//         Duration::from_secs(5),
//     ) {
//         Ok(bytes_written) => {
//             println!(
//                 "✅ {} bytes sent successfully via write_control",
//                 bytes_written
//             );
//             std::thread::sleep(Duration::from_micros(600)); // Adjusted delay to match C lib

//             let mut response_buffer = [0u8; 90];
//             match handle.read_control(
//                 0xA1,   // bmRequestType (Device to Host, Class, Interface)
//                 0x01,   // bRequest (GET_REPORT)
//                 0x0300, // wValue (Feature Report, ID 0x00)
//                 0x02,   // wIndex (Interface 2)
//                 &mut response_buffer,
//                 Duration::from_secs(5),
//             ) {
//                 Ok(bytes_read) => {
//                     println!("📥 Response received ({} bytes)", bytes_read);
//                     if bytes_read >= std::mem::size_of::<RazerReport>() {
//                         // Check against actual struct size
//                         let response: RazerReport = unsafe {
//                             std::ptr::read(response_buffer.as_ptr() as *const RazerReport)
//                         };
//                         print!("   Response Report: Status=0x{:02X}", response.status);
//                         if response.status == RAZER_CMD_NOT_SUPPORTED {
//                             print!(" (CMD_NOT_SUPPORTED)");
//                         } else if response.status == RAZER_CMD_SUCCESSFUL {
//                             print!(" (CMD_SUCCESSFUL)");
//                         }
//                         println!();
//                         return response.status == RAZER_CMD_SUCCESSFUL;
//                     } else {
//                         println!(
//                             "⚠️ Received response is too short ({} bytes) to be a RazerReport",
//                             bytes_read
//                         );
//                     }
//                 }
//                 Err(e) => {
//                     println!("⚠️ Could not read response: {:?}", e);
//                 }
//             }
//             false // If read fails or response is too short
//         }
//         Err(e) => {
//             println!("❌ Failed to send command: {:?}", e);
//             false
//         }
//     }
// }

fn main() {
    println!("💡 Razer Ornata V3 RGB Control");
    println!("========================================================");

    match find_ornata_v3() {
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
