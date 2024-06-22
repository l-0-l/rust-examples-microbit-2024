#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_io::Write;
use panic_rtt_target as _; // For a panic_handler function
use rtt_target::{rprintln, rtt_init_print};

use lsm303agr::{AccelOutputDataRate, Lsm303agr};

use microbit::{
    board::Board,
    hal::twi,
    hal::uart,
    hal::uart::{Baudrate, Parity},
    hal::Timer,
    pac::twi0::frequency::FREQUENCY_A,
};

/// The entry point of the application.
///
/// Initializes the RTT (Real-Time Transfer) for printing, sets up the UART for serial communication,
/// and configures the I2C interface for the LSM303AGR accelerometer. It collects and displays the
/// accelerometer data via both serial and RTT. To see the output, make sure minicom is installed, and run:
/// minicom -D /dev/ttyACM0 -b 1200 -8 -o
#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Initialize the board and peripherals
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Set up the UART for serial communication with 1200 baud rate and no parity
    let mut serial = uart::Uart::new(
        board.UART0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD1200,
    );

    // Set up the I2C interface for the LSM303AGR accelerometer
    let i2c = twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    // Initialize the accelerometer and set its mode and output data rate
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer,
            lsm303agr::AccelMode::HighResolution,
            AccelOutputDataRate::Khz1_344,
        )
        .unwrap();

    loop {
        // Delay for 50 milliseconds
        timer.delay_ms(50_u32);

        // Read acceleration data from the sensor
        let data = sensor.acceleration().unwrap();

        // Write the acceleration data to the serial output
        write!(
            serial,
            "X: {:>5}, Y: {:>5}, Z: {:>5}\n\r",
            data.x_unscaled(),
            data.y_unscaled(),
            data.z_unscaled()
        )
        .unwrap();

        // Print the acceleration data to the RTT output
        rprintln!(
            "X: {:>5}, Y: {:>5}, Z: {:>5}",
            data.x_unscaled(),
            data.y_unscaled(),
            data.z_unscaled()
        );
    }
}
