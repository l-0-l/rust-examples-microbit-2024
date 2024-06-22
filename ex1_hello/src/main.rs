#![no_std]
#![no_main]

// Import necessary crates and modules
use cortex_m as _; // For a default critical-section implementation when not using the microbit crate
use cortex_m_rt::entry;
use panic_rtt_target as _; // For a panic handler function
use rtt_target::{rprintln, rtt_init_print};

/// The entry point of the application.
///
/// Initializes RTT (Real-Time Transfer) for printing and enters an infinite loop.
#[entry]
fn main() -> ! {
    // Initialize RTT for printing debug messages
    rtt_init_print!();

    // Print a hello message to the RTT output
    rprintln!("Hello from micro:bit");

    // Enter an infinite loop, since there's no operating system to return to
    loop {}
}
