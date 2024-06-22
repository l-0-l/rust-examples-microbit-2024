#![no_std]
#![no_main]

mod mouse;
use mouse::MouseConverter;

mod keyboard;
use keyboard::KeyboardConverter;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use embedded_io::Write;
use lsm303agr::{AccelOutputDataRate, Lsm303agr};
use microbit::{
    board::Board,
    hal::twi,
    hal::uart,
    hal::uart::{Baudrate, Parity},
    hal::Timer,
    pac::twi0::frequency::FREQUENCY_A,
};
use panic_rtt_target as _; // For a panic_handler function
use rtt_target::{rprintln, rtt_init_print};

// Loop sleep time in milliseconds.
const DT: u32 = 17;

/// The entry point of the application.
///
/// Initializes the RTT (Real-Time Transfer) for printing, sets up the UART for serial communication,
/// and configures the I2C interface for the LSM303AGR accelerometer. It collects accelerometer data,
/// converts it to mouse or keyboard data, and sends it over the serial interface. To see the results,
/// run the following when built for mouse use:
/// sudo inputattach --microsoft /dev/ttyACM0
/// An the following for keyboard use (the keys are specifically selected to support my old SpaceGame):
/// sudo inputattach --ps2serkbd /dev/ttyACM0
#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Initialize the board and peripherals
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Set up the UART for serial communication with 1200 baud rate and no parity
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD1200,
        )
    };

    // Configure the button inputs with pull-up resistors
    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    // Set up the I2C interface for the LSM303AGR accelerometer
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    // Initialize the accelerometer and set its mode and output data rate
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer,
            lsm303agr::AccelMode::HighResolution,
            AccelOutputDataRate::Hz100,
        )
        .unwrap();

    // Create an accelerometer data to mouse moves converter (see mouse.rs)
    let mut mouse_converter = MouseConverter::new();
    let mut keyboard_converter = KeyboardConverter::new();

    loop {
        timer.delay_ms(DT);

        // Read the state of the buttons
        let buttons_state = (button_a.is_low().unwrap(), button_b.is_low().unwrap());

        // Read acceleration data from the sensor (only for x and y axis)
        let acc_x = sensor.acceleration().unwrap().x_unscaled();
        let acc_y = sensor.acceleration().unwrap().y_unscaled();

        rprintln!("acc_x: {}, acc_y: {}", acc_x, acc_y);

        // Convert sensor data to mouse movement packet
        let _mouse_packet = mouse_converter.sensor_data_to_packet(acc_x, acc_y, DT, buttons_state);

        // Convert sensor data to keypress packet
        let _keyboard_packet = keyboard_converter.sensor_data_to_packet(acc_x, buttons_state);

        // Send the packet over the serial interface
        for &byte in &_mouse_packet {
            serial.write(&[byte]).unwrap();
        }
        serial.flush().unwrap();
    }
}
