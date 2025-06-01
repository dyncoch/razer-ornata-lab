// Razer Ornata V3 RGB Control

use rusb::{Context, DeviceHandle, UsbContext};
use std::time::Duration;

const RAZER_VENDOR_ID: u16 = 0x1532;
const ORNATA_V3_PRODUCT_ID: u16 = 0x02A1;

const RAZER_CMD_SUCCESSFUL: u8 = 0x02;
const RAZER_CMD_NOT_SUPPORTED: u8 = 0x05;

// Constants from razercommon.h and razerchromacommon.h
const VARSTORE: u8 = 0x01;
const BACKLIGHT_LED: u8 = 0x05; // Common led_id for full keyboard effects in extended matrix

// Extended Matrix Effect IDs (Command Class 0x0F, Command ID 0x02)
const EXT_EFFECT_STATIC: u8 = 0x01;
const EXT_EFFECT_BREATHING: u8 = 0x02;
const EXT_EFFECT_SPECTRUM: u8 = 0x03;
const EXT_EFFECT_WAVE: u8 = 0x04;

#[repr(C)] // Ensure C-compatible layout
#[derive(Debug, Clone, Copy)]
struct RazerReport {
    status: u8,             // Byte 0
    transaction_id: u8,     // Byte 1
    remaining_packets: u16, // Byte 2-3 (unsigned short in C). Comment "Big Endian" in C.
    protocol_type: u8,      // Byte 4
    data_size: u8,          // Byte 5
    command_class: u8,      // Byte 6
    command_id: u8,         // Byte 7
    arguments: [u8; 80],    // Bytes 8-87
    crc: u8,                // Byte 88
    reserved: u8,           // Byte 89
} // Total size should now be 90 bytes

impl RazerReport {
    fn new() -> Self {
        Self {
            status: 0x00,
            transaction_id: 0x1F, // Specific for Ornata V3 as per C tests
            remaining_packets: 0x0000_u16.to_be(), // Initialize as u16, handle Big Endian. For 0, endianness doesn't change bytes.
            protocol_type: 0x00,
            data_size: 0x00,     // To be set by specific command
            command_class: 0x00, // To be set by specific command
            command_id: 0x00,    // To be set by specific command
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    fn calculate_crc(&mut self) {
        let mut crc_val: u8 = 0;
        // The CRC is calculated over bytes 2 through 87 (inclusive) of the 90-byte report.
        let report_ptr = self as *const Self as *const u8;

        for i in 2..=87 {
            // XOR bytes from the first byte of remaining_packets to the last byte of arguments
            crc_val ^= unsafe { *report_ptr.add(i) };
        }
        self.crc = crc_val;
    }

    // Corrected static RGB command
    fn create_ornata_v3_static_rgb(red: u8, green: u8, blue: u8) -> Self {
        let mut report = Self::new();

        report.command_class = 0x0F; // Extended matrix commands
        report.command_id = 0x02; // Matrix effect command
        report.data_size = 0x09; // Data size for extended static effect

        report.arguments[0] = VARSTORE;
        report.arguments[1] = BACKLIGHT_LED;
        report.arguments[2] = EXT_EFFECT_STATIC; // Effect ID for static
        report.arguments[3] = 0x00; // Reserved/default
        report.arguments[4] = 0x00; // Reserved/default
        report.arguments[5] = 0x01; // Parameter specific to extended static
        report.arguments[6] = red;
        report.arguments[7] = green;
        report.arguments[8] = blue;

        report.calculate_crc();
        report
    }

    // Corrected spectrum effect
    fn create_ornata_v3_spectrum() -> Self {
        let mut report = Self::new();

        report.command_class = 0x0F;
        report.command_id = 0x02;
        report.data_size = 0x06;

        report.arguments[0] = VARSTORE;
        report.arguments[1] = BACKLIGHT_LED;
        report.arguments[2] = EXT_EFFECT_SPECTRUM;
        // arguments[3-5] default to 0

        report.calculate_crc();
        report
    }

    // Corrected breathing effect (single color)
    fn create_ornata_v3_breathing(red: u8, green: u8, blue: u8) -> Self {
        let mut report = Self::new();

        report.command_class = 0x0F;
        report.command_id = 0x02;
        report.data_size = 0x09;

        report.arguments[0] = VARSTORE;
        report.arguments[1] = BACKLIGHT_LED;
        report.arguments[2] = EXT_EFFECT_BREATHING;
        report.arguments[3] = 0x01; // Breathing type: single color
                                    // report.arguments[4] = 0x00;
        report.arguments[5] = 0x01; // Color count / flag
        report.arguments[6] = red;
        report.arguments[7] = green;
        report.arguments[8] = blue;

        report.calculate_crc();
        report
    }

    // Corrected wave effect
    fn create_ornata_v3_wave(direction: u8, speed_param: u8) -> Self {
        let mut report = Self::new();

        report.command_class = 0x0F;
        report.command_id = 0x02;
        report.data_size = 0x06;

        report.arguments[0] = VARSTORE;
        report.arguments[1] = BACKLIGHT_LED;
        report.arguments[2] = EXT_EFFECT_WAVE;
        report.arguments[3] = direction;
        report.arguments[4] = speed_param;
        // report.arguments[5] = 0x00;

        report.calculate_crc();
        report
    }
}

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

fn send_ornata_v3_command(handle: &mut DeviceHandle<Context>, command: &RazerReport) -> bool {
    // Ensure the command struct is indeed 90 bytes
    assert_eq!(
        std::mem::size_of::<RazerReport>(),
        90,
        "RazerReport struct size is not 90 bytes!"
    );

    let command_bytes = unsafe { std::slice::from_raw_parts(command as *const _ as *const u8, 90) };

    println!("📤 Sending Ornata V3 command:");
    println!("   Transaction ID: 0x{:02X}", command.transaction_id);
    println!(
        "   Class: 0x{:02X}, ID: 0x{:02X}",
        command.command_class, command.command_id
    );
    println!("   Data Size: 0x{:02X}", command.data_size);
    print!("   Arguments (first 10 of 80): ");
    for i in 0..std::cmp::min(10, command.arguments.len()) {
        print!("{:02X} ", command.arguments[i]);
    }
    println!("\n   CRC: 0x{:02X}", command.crc);

    match handle.write_control(
        0x21,   // bmRequestType (Host to Device, Class, Interface)
        0x09,   // bRequest (SET_REPORT)
        0x0300, // wValue (Feature Report, ID 0x00)
        0x02,   // wIndex (Interface 2)
        command_bytes,
        Duration::from_secs(5),
    ) {
        Ok(bytes_written) => {
            println!(
                "✅ {} bytes sent successfully via write_control",
                bytes_written
            );
            std::thread::sleep(Duration::from_micros(600)); // Adjusted delay to match C lib

            let mut response_buffer = [0u8; 90];
            match handle.read_control(
                0xA1,   // bmRequestType (Device to Host, Class, Interface)
                0x01,   // bRequest (GET_REPORT)
                0x0300, // wValue (Feature Report, ID 0x00)
                0x02,   // wIndex (Interface 2)
                &mut response_buffer,
                Duration::from_secs(5),
            ) {
                Ok(bytes_read) => {
                    println!("📥 Response received ({} bytes)", bytes_read);
                    if bytes_read >= std::mem::size_of::<RazerReport>() {
                        // Check against actual struct size
                        let response: RazerReport = unsafe {
                            std::ptr::read(response_buffer.as_ptr() as *const RazerReport)
                        };
                        print!("   Response Report: Status=0x{:02X}", response.status);
                        if response.status == RAZER_CMD_NOT_SUPPORTED {
                            print!(" (CMD_NOT_SUPPORTED)");
                        } else if response.status == RAZER_CMD_SUCCESSFUL {
                            print!(" (CMD_SUCCESSFUL)");
                        }
                        println!();
                        return response.status == RAZER_CMD_SUCCESSFUL;
                    } else {
                        println!(
                            "⚠️ Received response is too short ({} bytes) to be a RazerReport",
                            bytes_read
                        );
                    }
                }
                Err(e) => {
                    println!("⚠️ Could not read response: {:?}", e);
                }
            }
            false // If read fails or response is too short
        }
        Err(e) => {
            println!("❌ Failed to send command: {:?}", e);
            false
        }
    }
}

fn main() {
    println!("💡 Razer Ornata V3 RGB Control");
    println!("========================================================");

    match find_ornata_v3() {
        Some(mut handle) => {
            println!("✅ Connected to Ornata V3");

            println!("\n🔴 Setting static red color...");
            let red_cmd = RazerReport::create_ornata_v3_static_rgb(0xFF, 0x00, 0x00);
            if send_ornata_v3_command(&mut handle, &red_cmd) {
                println!("   Keyboard should be RED.");
            } else {
                println!("   Failed to set red color (or command not supported/failed).");
            }
            std::thread::sleep(Duration::from_secs(3));

            println!("\n🟢 Setting static green color...");
            let green_cmd = RazerReport::create_ornata_v3_static_rgb(0x00, 0xFF, 0x00);
            if send_ornata_v3_command(&mut handle, &green_cmd) {
                println!("   Keyboard should be GREEN.");
            } else {
                println!("   Failed to set green color.");
            }
            std::thread::sleep(Duration::from_secs(3));

            println!("\n🔵 Setting static blue color...");
            let blue_cmd = RazerReport::create_ornata_v3_static_rgb(0x00, 0x00, 0xFF);
            if send_ornata_v3_command(&mut handle, &blue_cmd) {
                println!("   Keyboard should be BLUE.");
            } else {
                println!("   Failed to set blue color.");
            }
            std::thread::sleep(Duration::from_secs(3));

            println!("\n🌈 Setting spectrum cycling...");
            let spectrum_cmd = RazerReport::create_ornata_v3_spectrum();
            if send_ornata_v3_command(&mut handle, &spectrum_cmd) {
                println!("   Keyboard should be in SPECTRUM mode.");
            } else {
                println!("   Failed to set spectrum mode.");
            }
            std::thread::sleep(Duration::from_secs(5));

            println!("\n💜 Setting purple breathing effect...");
            let breathing_cmd = RazerReport::create_ornata_v3_breathing(0xFF, 0x00, 0xFF);
            if send_ornata_v3_command(&mut handle, &breathing_cmd) {
                println!("   Keyboard should be BREATHING PURPLE.");
            } else {
                println!("   Failed to set breathing mode.");
            }
            std::thread::sleep(Duration::from_secs(5));

            println!("\n🌊 Setting wave effect (direction 0, speed 0x01)...");
            let wave_cmd = RazerReport::create_ornata_v3_wave(0, 0x01);
            if send_ornata_v3_command(&mut handle, &wave_cmd) {
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

// Convenience functions for library-like use (optional)
pub fn set_color(red: u8, green: u8, blue: u8) -> Result<(), String> {
    match find_ornata_v3() {
        Some(mut handle) => {
            let cmd = RazerReport::create_ornata_v3_static_rgb(red, green, blue);
            if send_ornata_v3_command(&mut handle, &cmd) {
                Ok(())
            } else {
                Err(
                    "Failed to set static color (device reported not supported or other error)"
                        .to_string(),
                )
            }
        }
        None => Err("Razer Ornata V3 not found".to_string()),
    }
}

pub fn set_spectrum_effect() -> Result<(), String> {
    match find_ornata_v3() {
        Some(mut handle) => {
            let cmd = RazerReport::create_ornata_v3_spectrum();
            if send_ornata_v3_command(&mut handle, &cmd) {
                Ok(())
            } else {
                Err(
                    "Failed to set spectrum effect (device reported not supported or other error)"
                        .to_string(),
                )
            }
        }
        None => Err("Razer Ornata V3 not found".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_size() {
        assert_eq!(std::mem::size_of::<RazerReport>(), 90);
    }

    #[test]
    fn test_static_red_command_creation_and_crc() {
        let mut red_cmd = RazerReport::create_ornata_v3_static_rgb(0xFF, 0x00, 0x00);
        // Manually calculate expected CRC for this specific known command
        // Bytes 2-87 for XORing
        // remaining_packets (00 00), proto (00), data_size (09), cmd_class (0F), cmd_id (02)
        // args[0-8]: 01 05 01 00 00 01 FF 00 00 ... (rest are 0)
        // Expected: 0x00^0x00^0x00^0x09^0x0F^0x02^0x01^0x05^0x01^0x00^0x00^0x01^0xFF^0x00^0x00 = 0xFF
        assert_eq!(red_cmd.crc, 0xFF, "CRC for static red command is incorrect");

        assert_eq!(red_cmd.transaction_id, 0x1F);
        assert_eq!(red_cmd.command_class, 0x0F);
        assert_eq!(red_cmd.command_id, 0x02);
        assert_eq!(red_cmd.data_size, 0x09);

        assert_eq!(red_cmd.arguments[0], VARSTORE);
        assert_eq!(red_cmd.arguments[1], BACKLIGHT_LED);
        assert_eq!(red_cmd.arguments[2], EXT_EFFECT_STATIC);
        assert_eq!(red_cmd.arguments[5], 0x01);
        assert_eq!(red_cmd.arguments[6], 0xFF);
        assert_eq!(red_cmd.arguments[7], 0x00);
        assert_eq!(red_cmd.arguments[8], 0x00);
    }

    #[test]
    fn test_spectrum_command_creation_and_crc() {
        let spectrum_cmd = RazerReport::create_ornata_v3_spectrum();
        // Expected: 0x00^0x00^0x00^0x06^0x0F^0x02^0x01^0x05^0x03 = 0x0C
        assert_eq!(
            spectrum_cmd.crc, 0x0C,
            "CRC for spectrum command is incorrect"
        );
        assert_eq!(spectrum_cmd.data_size, 0x06);
        assert_eq!(spectrum_cmd.arguments[2], EXT_EFFECT_SPECTRUM);
    }
}
