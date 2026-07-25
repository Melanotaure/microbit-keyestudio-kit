#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;

use project_01_heartbeat::led_matrix::{DISPLAY_SIGNAL, display_matrix};

use {defmt_rtt as _, panic_probe as _};

const BIG_HEART: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
];

const SMALL_HEART: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
];

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let rows = [
        Output::new(p.P0_21, Level::Low, OutputDrive::Standard),
        Output::new(p.P0_22, Level::Low, OutputDrive::Standard),
        Output::new(p.P0_15, Level::Low, OutputDrive::Standard),
        Output::new(p.P0_24, Level::Low, OutputDrive::Standard),
        Output::new(p.P0_19, Level::Low, OutputDrive::Standard),
    ];

    let cols = [
        Output::new(p.P0_28, Level::High, OutputDrive::Standard),
        Output::new(p.P0_11, Level::High, OutputDrive::Standard),
        Output::new(p.P0_31, Level::High, OutputDrive::Standard),
        Output::new(p.P1_05, Level::High, OutputDrive::Standard),
        Output::new(p.P0_30, Level::High, OutputDrive::Standard),
    ];

    spawner.spawn(display_matrix(rows, cols).expect("display_matrix error"));

    loop {
        DISPLAY_SIGNAL.signal(SMALL_HEART);
        Timer::after_millis(500).await;
        DISPLAY_SIGNAL.signal(BIG_HEART);
        Timer::after_millis(500).await;
    }
}
