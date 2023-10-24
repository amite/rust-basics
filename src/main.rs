#![allow(dead_code)]

use crate::ownership::Hands;

mod ownership;

// use color_eyre::owo_colors::OwoColorize;

fn main() {
    // let hands =

    let mut hands = Hands::new();
    // Hands::report(&hands);
    hands.report();

    hands = hands.juggle();
    hands.report();
}
