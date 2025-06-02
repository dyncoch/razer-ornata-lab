use rusb::{DeviceHandle, UsbContext};
use std::time::Duration;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RazerReport {
    pub status: u8,
    pub transaction_id: u8,
    pub remaining_packets: u16,
    pub protocol_type: u8,
    pub data_size: u8,
    pub command_class: u8,
    pub command_id: u8,
    pub arguments: [u8; 80],
    pub crc: u8,
    pub reserved: u8,
}

// Constants needed
pub const VARSTORE: u8 = 0x01;
pub const BACKLIGHT_LED: u8 = 0x05;

pub const EXT_EFFECT_STATIC: u8 = 0x01;
pub const EXT_EFFECT_BREATHING: u8 = 0x02;
pub const EXT_EFFECT_SPECTRUM: u8 = 0x04;
pub const EXT_EFFECT_WAVE: u8 = 0x03;

pub const RAZER_CMD_SUCCESSFUL: u8 = 0x02;
pub const RAZER_CMD_NOT_SUPPORTED: u8 = 0x05;

impl RazerReport {
    pub fn new() -> Self {
        Self {
            status: 0x00,
            transaction_id: 0x1F,
            remaining_packets: 0x0000_u16.to_be(),
            protocol_type: 0x00,
            data_size: 0x00,
            command_class: 0x00,
            command_id: 0x00,
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    pub fn calculate_crc(&mut self) {
        let mut crc_val: u8 = 0;
        let report_ptr = self as *const Self as *const u8;
        for i in 2..=87 {
            crc_val ^= unsafe { *report_ptr.add(i) };
        }
        self.crc = crc_val;
    }

    pub fn static_rgb(red: u8, green: u8, blue: u8) -> Self {
        let mut report = Self::new();
        report.command_class = 0x0F;
        report.command_id = 0x02;
        report.data_size = 0x09;

        report.arguments[0] = VARSTORE;
        report.arguments[1] = BACKLIGHT_LED;
        report.arguments[2] = EXT_EFFECT_STATIC;
        report.arguments[5] = 0x01;
        report.arguments[6] = red;
        report.arguments[7] = green;
        report.arguments[8] = blue;

        report.calculate_crc();
        report
    }

    pub fn spectrum() -> Self {
        let mut report = Self::new();
        report.command_class = 0x0F;
        report.command_id = 0x02;
        report.data_size = 0x06;
        report.arguments[0] = VARSTORE;
        report.arguments[1] = BACKLIGHT_LED;
        report.arguments[2] = EXT_EFFECT_SPECTRUM;
        report.calculate_crc();
        report
    }

    pub fn breathing(red: u8, green: u8, blue: u8) -> Self {
        let mut report = Self::new();
        report.command_class = 0x0F;
        report.command_id = 0x02;
        report.data_size = 0x09;
        report.arguments[0] = VARSTORE;
        report.arguments[1] = BACKLIGHT_LED;
        report.arguments[2] = EXT_EFFECT_BREATHING;
        report.arguments[3] = 0x01;
        report.arguments[5] = 0x01;
        report.arguments[6] = red;
        report.arguments[7] = green;
        report.arguments[8] = blue;
        report.calculate_crc();
        report
    }

    pub fn wave(direction: u8, speed: u8) -> Self {
        let mut report = Self::new();
        report.command_class = 0x0F;
        report.command_id = 0x02;
        report.data_size = 0x06;
        report.arguments[0] = VARSTORE;
        report.arguments[1] = BACKLIGHT_LED;
        report.arguments[2] = EXT_EFFECT_WAVE;
        report.arguments[3] = direction;
        report.arguments[4] = speed;
        report.calculate_crc();
        report
    }

    pub fn send<T: UsbContext>(&self, handle: &mut DeviceHandle<T>) -> bool {
        assert_eq!(
            std::mem::size_of::<Self>(),
            90,
            "RazerReport struct size is not 90 bytes!"
        );

        let command_bytes =
            unsafe { std::slice::from_raw_parts(self as *const _ as *const u8, 90) };

        match handle.write_control(
            0x21,
            0x09,
            0x0300,
            0x02,
            command_bytes,
            Duration::from_secs(5),
        ) {
            Ok(_) => {
                std::thread::sleep(Duration::from_micros(600));
                let mut response_buffer = [0u8; 90];
                match handle.read_control(
                    0xA1,
                    0x01,
                    0x0300,
                    0x02,
                    &mut response_buffer,
                    Duration::from_secs(5),
                ) {
                    Ok(bytes_read) if bytes_read >= 90 => {
                        let response: RazerReport = unsafe {
                            std::ptr::read(response_buffer.as_ptr() as *const RazerReport)
                        };
                        response.status == RAZER_CMD_SUCCESSFUL
                    }
                    _ => false,
                }
            }
            Err(_) => false,
        }
    }
}
