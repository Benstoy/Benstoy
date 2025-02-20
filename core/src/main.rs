#![no_main]
#![no_std]

use uefi::{prelude::*, println};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    println!("Hello qemu!");

    boot::stall(10_000_000);
    Status::SUCCESS
}
