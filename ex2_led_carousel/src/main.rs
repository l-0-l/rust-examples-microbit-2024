#![no_std]
#![no_main]

use panic_rtt_target as _; // For a panic handler function
use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer}; // Also exposes a critical-section
use rtt_target::rtt_init_print;

/// The 5x5 LED matrix patterns to display.
const MATRIX: [[[u8; 5]; 5]; 6] = [
    [[0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0]],
    [[0, 0, 0, 1, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 1, 0, 0, 0]],
    [[0, 0, 0, 1, 0], [0, 0, 0, 1, 0], [0, 0, 1, 0, 0], [0, 1, 0, 0, 0], [0, 1, 0, 0, 0]],
    [[0, 0, 0, 0, 0], [0, 0, 0, 1, 0], [0, 0, 1, 0, 0], [0, 1, 0, 0, 0], [0, 0, 0, 0, 0]],
    [[0, 0, 0, 0, 0], [0, 0, 0, 1, 1], [0, 0, 1, 0, 0], [1, 1, 0, 0, 0], [0, 0, 0, 0, 0]],
    [[0, 0, 0, 0, 0], [0, 0, 0, 0, 1], [0, 1, 1, 1, 0], [1, 0, 0, 0, 0], [0, 0, 0, 0, 0]],
];

/// Rotates a 5x5 matrix 90 degrees clockwise.
///
/// # Arguments
///
/// * `matrix` - A 5x5 matrix of `u8` values.
///
/// # Returns
///
/// A new 5x5 matrix rotated 90 degrees clockwise.
fn rotate_90(matrix: [[u8; 5]; 5]) -> [[u8; 5]; 5] {
    let mut rotated = [[0; 5]; 5];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            rotated[j][4 - i] = val;
        }
    }
    rotated
}

/// The entry point of the application.
///
/// Initializes the RTT (Real-Time Transfer) for printing, sets up the micro:bit board,
/// and displays the LED matrix patterns.
#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    const DELAY: u32 = 25;

    loop {
        for matrix in MATRIX.iter() {
            display.show(&mut timer, *matrix, DELAY);
        }
        for i in 0..MATRIX.len() {
            display.show(&mut timer, rotate_90(MATRIX[i]), DELAY);
        }
    }
}
