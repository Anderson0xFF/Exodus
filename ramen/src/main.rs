#![allow(dead_code)]

extern crate drm;

mod card;
mod logger;
mod errors;
mod display;
mod resource;
mod surface;


use std::rc::Rc;
use crate::card::Card;

#[allow(unused_unsafe)]
fn main() {
    unsafe {
        logger::Logger::init(logger::Level::Debug, "ramen.log");
        let card = Rc::new(Card::open("/dev/dri/card0").unwrap());
        let display = display::Display::new(&card).unwrap();


    }
}