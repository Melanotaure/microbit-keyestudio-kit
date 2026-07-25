#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;

use project_02_light_a_single_led::led_matrix::{display_matrix, plot_xy, unplot_xy};

use {defmt_rtt as _, panic_probe as _};

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
        plot_xy(1, 0).await;
        Timer::after_millis(500).await;
        unplot_xy(1, 0).await;
        Timer::after_millis(500).await;
        plot_xy(3, 4).await;
        Timer::after_millis(500).await;
        unplot_xy(3, 4).await;
        Timer::after_millis(500).await;
    }
}
