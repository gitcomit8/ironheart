# IronHeart (x86-64, UEFI)

This repository contains the source code for a simple x86-64 kernel written in Rust, targeting UEFI firmware. This project is a learning exercise in kernel development and aims to implement basic kernel functionality.

## Features (Work in Progress)

*   x86-64 architecture support
*   UEFI boot
*   Basic memory management
*   Basic I/O (console output)
*   Interrupt handling (basic)

## Building

1.  **Prerequisites:**
    *   Rust toolchain (nightly recommended)
    *   Cross-compilation toolchain for x86-64 UEFI (e.g., `x86_64-unknown-uefi`)
    *   UEFI development tools (e.g., EDK2)

2.  **Build Steps:**
    ```bash
    cargo build --target x86_64-unknown-uefi --release
    # (Additional steps to create the EFI application - to be detailed)
    ```

## Running

1.  **UEFI Environment:** You'll need a UEFI-compatible environment (e.g., a virtual machine like QEMU or real hardware).

2.  **Boot:** Copy the EFI application (`.efi` file) to the EFI System Partition (ESP) and configure your UEFI firmware to boot from it.

## Contributing

Contributions are welcome!  Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
