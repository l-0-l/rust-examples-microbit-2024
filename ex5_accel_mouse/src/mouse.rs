use fixed::types::I32F32;

/// A struct for converting accelerometer data to mouse movement data.
pub struct MouseConverter {
    prev_x: I32F32,
    prev_y: I32F32,
    prev_velocity_x: I32F32,
    prev_velocity_y: I32F32,
    max_accel: I32F32,
    max_movement_unit: I32F32,
    scaling_factor: I32F32,
    y_axis_direction: i16,
    delta_x: i8,
    delta_y: i8,
}

impl MouseConverter {
    /// Creates a new `MouseConverter`.
    pub fn new() -> Self {
        MouseConverter {
            prev_x: I32F32::from_num(0),
            prev_y: I32F32::from_num(0),
            prev_velocity_x: I32F32::from_num(0),
            prev_velocity_y: I32F32::from_num(0),
            max_accel: I32F32::from_num(4096),
            max_movement_unit: I32F32::from_num(127),
            scaling_factor: I32F32::from_num(5000),
            y_axis_direction: -1, // Useful if you like your Y asis swapped :)
            delta_x: 0,
            delta_y: 0,
        }
    }

    /*
     * https://roborooter.com/post/serial-mice/
     *
     * Microsoft 2-button serial mouse
     *
     *         D7      D6      D5      D4      D3      D2      D1      D0
     * Byte 1  X       1       LB      RB      Y7      Y6      X7      X6
     * Byte 2  X       0       X5      X4      X3      X2      X1      X0
     * Byte 3  X       0       Y5      Y4      Y3      Y2      Y1      Y0
     *
     * LB is the state of the left button (1 means down)
     * RB is the state of the right button (1 means down)
     * X7-X0 movement in X direction since last packet (signed byte)
     * Y7-Y0 movement in Y direction since last packet (signed byte)
     */

    /// Construct a packet by the Microsoft 2-button serial mouse protocol
    fn construct_packet(&mut self, buttons: (bool, bool)) -> [u8; 3] {
        let mut packet = [0u8; 3];

        packet[0] = 0x40
            | match buttons {
                (false, false) => 0x00,
                (false, true) => 0x10,
                (true, false) => 0x20,
                (true, true) => 0x30,
            };

        packet[0] |= ((self.delta_x as u8 & 0xC0) >> 6) | ((self.delta_y as u8 & 0xC0) >> 4);
        packet[1] = self.delta_x as u8 & 0x3F;
        packet[2] = self.delta_y as u8 & 0x3F;

        packet
    }

    /// Converts acceleration data to distance.
    ///
    /// # Arguments
    ///
    /// * `current_x` - The current X acceleration.
    /// * `current_y` - The current Y acceleration.
    /// * `dt` - The time delta in milliseconds.
    fn accel_to_distance(&mut self, current_x: i16, current_y: i16, dt: u32) {
        let current_x = I32F32::from_num(current_x);
        let current_y = I32F32::from_num(self.y_axis_direction * current_y);
        let dt = I32F32::from_num(dt) / 1000; // milliseconds to seconds

        // Calculate the change in accelerationpacket[0],
        let accel_x = current_x - self.prev_x;
        let accel_y = current_y - self.prev_y;

        // Update previous acceleration values
        self.prev_x = current_x;
        self.prev_y = current_y;

        // Update velocities
        self.prev_velocity_x += accel_x * dt;
        self.prev_velocity_y += accel_y * dt;

        // Calculate distances
        let distance_x = self.prev_velocity_x * dt + accel_x * (dt * dt) / 2;
        let distance_y = self.prev_velocity_y * dt + accel_y * (dt * dt) / 2;

        // Scale distances to mouse movement units and clamp it to [-127, 127]
        self.delta_x = (distance_x * self.scaling_factor / self.max_accel * self.max_movement_unit)
            .to_num::<i32>()
            .clamp(-127, 127) as i8;
        self.delta_y = (distance_y * self.scaling_factor / self.max_accel * self.max_movement_unit)
            .to_num::<i32>()
            .clamp(-127, 127) as i8;
    }

    /// Converts sensor data to a mouse movement packet.
    ///
    /// # Arguments
    ///
    /// * `x` - The current X acceleration.
    /// * `y` - The current Y acceleration.
    /// * `dt` - The time delta in milliseconds.
    /// * `buttons` - The state of the buttons.
    ///
    /// # Returns
    ///
    /// A 3-byte array representing the mouse movement packet.
    pub fn sensor_data_to_packet(
        &mut self,
        x: i16,
        y: i16,
        dt: u32,
        buttons: (bool, bool),
    ) -> [u8; 3] {
        self.accel_to_distance(x, y, dt);
        self.construct_packet(buttons)
    }
}
