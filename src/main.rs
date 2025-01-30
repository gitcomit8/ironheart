#![no_std]
#![no_main]

extern crate uefi;
extern crate uefi_services;

use core::fmt::Write;
use uefi::prelude::*;

#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initialize UEFI services
    uefi_services::init(&mut system_table).unwrap();

    // Get the console
    let stdout = system_table.stdout();

    // Clear the screen
    stdout.clear().unwrap();

    // Print a test message
    writeln!(stdout, "Hello from UEFI Rust Kernel!").unwrap();
    writeln!(stdout, "Boot services are active").unwrap();
    writeln!(
        stdout,
        "System table revision: {}",
        system_table.header.revision
    )
    .unwrap();

    // Don't exit boot services yet - just loop
    loop {
        // CPU halt instruction - better than busy loop
        x86_64::instructions::hlt();
    }
}

// Required for panic situations
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
