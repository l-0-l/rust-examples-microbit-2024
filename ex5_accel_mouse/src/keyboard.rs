/// A struct for converting accelerometer adat to keypress data.
pub struct KeyboardConverter {
    prev_left: bool,
    prev_right: bool,
    prev_z: bool,
    prev_x: bool,
}

impl KeyboardConverter {
    /// Creates a new `KeyboardConverter`.
    pub fn new() -> Self {
        KeyboardConverter {
            prev_left: false,
            prev_right: false,
            prev_z: false,
            prev_x: false,
        }
    }

    /*
     * https://www-ug.eecg.toronto.edu/msl/nios_devices/datasheets/PS2%20Keyboard%20Protocol.htm
     *
     * PS/2 Keyboard Interface
     *
     */

    /// Converts sensor data to a key press packet.
    ///
    /// # Arguments
    ///
    /// * `x` - The current X acceleration.
    /// * `buttons` - The state of the buttons.
    ///
    /// # Returns
    ///
    /// An 11-byte array representing the keys packet.
    pub fn sensor_data_to_packet(
        &mut self,
        x: i16,
        buttons: (bool, bool),
    ) -> [u8; 11] {
        let mut packet = [0u8; 11];

        match (buttons.0, self.prev_z) {
            (true, false) => {
                packet[0..1].copy_from_slice(&[0x1a]);
                self.prev_z = true;
            },
            (false, true) => {
                packet[0..2].copy_from_slice(&[0xf0, 0x1a]);
                self.prev_z = false;
            },
            _ => {},
        }

        match (buttons.1, self.prev_x) {
            (true, false) => {
                packet[2..3].copy_from_slice(&[0x22]);
                self.prev_x = true;
            },
            (false, true) => {
                packet[2..4].copy_from_slice(&[0xf0, 0x22]);
                self.prev_x = false;
            },
            _ => {},
        }

        match (x < -300, self.prev_left) {
            (true, false) => {
                packet[4..6].copy_from_slice(&[0xe0, 0x6b]);
                self.prev_left = true;
            },
            (false, true) => {
                packet[4..7].copy_from_slice(&[0xe0, 0xf0, 0x6b]);
                self.prev_left = false;
            },
            _ => {},
        }
        match (x > 300, self.prev_right) {
            (true, false) => {
                packet[7..9].copy_from_slice(&[0xe0, 0x74]);
                self.prev_right = true;
            },
            (false, true) => {
                packet[7..10].copy_from_slice(&[0xe0, 0xf0, 0x74]);
                self.prev_right = false;
            },
            _ => {},
        }

        packet
    }
}
