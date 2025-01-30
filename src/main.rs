#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi_raw::table::system::SystemTable;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "efiapi" fn efi_main(_handle: Handle, _system_table: SystemTable) -> Status {
    Status::SUCCESS
}
