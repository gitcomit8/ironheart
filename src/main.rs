#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::proto::console::text::{SimpleTextOutput, SimpleTextOutputProtocol};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "efiapi" fn efi_main(_handle: Handle,system_table: SystemTable<Boot>) -> Status {
    let stdout = system_table.stdout();

    stdout.clear().unwrap();
    stdout.output_string("Hello, UEFI!\r\n").unwrap();
    Status::SUCCESS
}
