#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use embedded_io::Write; // For serial writing

use microbit::{
    board::Board,
    hal::{
        uart::{self, Baudrate, Parity},
        Timer,
    },
};

/// The entry point of the application.
///
/// Sets up the UART for serial communication and configures the button inputs,
/// then keeps sending the button status as a single character. To see the output,
/// make sure minicom is installed, and run:
/// minicom -D /dev/ttyACM0 -b 1200 -8 -o
#[entry]
fn main() -> ! {
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

    // Configure the button inputs with pull-up resistors (probably pullups aren't necessary)
    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    let mut counter = 0;
    loop {
        // Sleep for 50 milliseconds
        timer.delay_ms(50u32);

        // Determine the state of the buttons and set the corresponding symbol
        let symbol = match (button_a.is_low().unwrap(), button_b.is_low().unwrap()) {
            (false, false) => &[b'-'],
            (true, false) => &[b'A'],
            (false, true) => &[b'B'],
            (true, true) => &[b'X'],
        };

        // Write the symbol to the serial output
        serial.write(symbol).unwrap();
        serial.flush().unwrap();

        counter += 1;
        if counter > 40 {
            // Write a newline character to the serial output every 2 seconds
            write!(serial, "\n\r").unwrap();
            counter = 0;
        }
    }
}

/// A simplified panic handler
///
/// In this example there's no use of RTT which defines its own panic hanler,
/// so we must define one.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
