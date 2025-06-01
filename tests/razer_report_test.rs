use razer_rgb_mac::razer_report::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_size() {
        assert_eq!(std::mem::size_of::<RazerReport>(), 90);
    }

    #[test]
    fn test_static_red_command_creation_and_crc() {
        let red_cmd = RazerReport::static_rgb(0xFF, 0x00, 0x00);
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
        let spectrum_cmd = RazerReport::spectrum();
        // Expected: 0x00^0x00^0x00^0x06^0x0F^0x02^0x01^0x05^0x03 = 0x0C
        assert_eq!(
            spectrum_cmd.crc, 0x0C,
            "CRC for spectrum command is incorrect"
        );
        assert_eq!(spectrum_cmd.data_size, 0x06);
        assert_eq!(spectrum_cmd.arguments[2], EXT_EFFECT_SPECTRUM);
    }
}
