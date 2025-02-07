#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uefi::{Handle, Status};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
struct TextOutputProtocol {
    _pad: [u64; 2],
    output_string: extern "efiapi" fn(*mut TextOutputProtocol, *const u16) -> Status,
}

#[repr(C)]
struct SystemTable {
    _pad: [u64; 60],
    con_out: *mut TextOutputProtocol,
}

#[no_mangle]
extern "efiapi" fn efi_main(_image_handle: Handle, system_table: *mut SystemTable) -> Status {
    unsafe {
        let mut hello = [0u16; 13];
        "Hello World!"
            .encode_utf16()
            .enumerate()
            .for_each(|(i, c)| hello[i] = c);
        hello[12] = 0;

        let con_out = &mut *(*system_table).con_out;

        let _ = (con_out.output_string)(con_out, hello.as_ptr());
    }
    loop {}
}
