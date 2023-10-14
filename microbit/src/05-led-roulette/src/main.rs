#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
// use panic_halt as _;

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        timer::Timer,
        prelude::*
    }
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        ];

    const PIXELS: [(usize, usize); 16] = [
        (0,0),(0,1),(0,2),(0,3),(0,4),(1,4),(2,4),(3,4),(4,4),(4,3),(4,2),(4,1),(4,0),(3,0),(2,0),(1,0)
    ];

    let mut last_led = (0,0);

    // infinite loop; just so we don't leave this stack frame
    loop {
        // Show all the lights for 1000ms
        for (row, row_val) in leds.iter().enumerate() {
            for (col, col_val) in row_val.iter().enumerate() {
                rprintln!("col: {}, row: {}", col, row);
                let mut new_board = leds;
                new_board[row][col] = 1;

                display.show(&mut timer, new_board, 50);
            } 
        }
        // for current_led in PIXELS.iter() {
        //     leds[last_led.0][last_led.1] = 0;
        //     leds[current_led.0][current_led.1] = 1;
        //     display.show(&mut timer, leds, 50);
        //     last_led = *current_led;
        // }


        // clear the display again
        display.clear();
        rprintln!("Clear");
        // timer.delay_ms(1_000u16);
    }
}
