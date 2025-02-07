Here’s a **project-driven plan** for building a kernel in Rust, designed for someone who learns by doing. Each phase focuses on building a tangible component of the kernel, with minimal theory and maximum hands-on coding. Start with tiny milestones and iterate aggressively.

---

### **Phase 1: Bare-Metal "Hello World" (Weeks 1–2)**  
**Goal**: Boot a minimal kernel that prints text to the screen.  
**What You’ll Build**:  
1. A Rust project that compiles to a freestanding binary (`no_std`).  
2. A bootloader (use `bootimage` crate or Multiboot2).  
3. A VGA text mode driver to print "Hello World".  

**Steps**:  
1. **Set up the Rust environment**:  
   ```bash
   rustup target add x86_64-unknown-none  
   cargo new --bin my_kernel  
   ```  
   Configure `Cargo.toml` for `no_std` and nightly features.  

2. **Write the kernel entry point**:  
   ```rust
   #![no_std]  
   #![no_main]  

   use core::panic::PanicInfo;  

   #[no_mangle]  
   pub extern "C" fn _start() -> ! {  
       // Write to VGA buffer here  
       loop {}  
   }  

   #[panic_handler]  
   fn panic(_info: &PanicInfo) -> ! { loop {} }  
   ```  

3. **Print to VGA buffer**:  
   - Directly write ASCII bytes to memory address `0xb8000` (VGA text buffer).  
   - Use `unsafe` Rust to manipulate raw pointers.  

**Key Learning**:  
- How Rust works without the standard library.  
- Basics of x86 boot process and memory-mapped I/O.  

**Resource**: Follow [Philipp Oppermann’s “Writing an OS in Rust”](https://os.phil-opp.com/) for code templates.  

---

### **Phase 2: Handle Interrupts and Exceptions (Weeks 3–4)**  
**Goal**: Make the kernel respond to hardware interrupts (e.g., keyboard) and CPU exceptions.  
**What You’ll Build**:  
1. An Interrupt Descriptor Table (IDT).  
2. A basic exception handler (e.g., divide-by-zero).  
3. A keyboard driver that prints keystrokes to the screen.  

**Steps**:  
1. **Set up the IDT**:  
   Use the `x86_64` crate to define interrupt handlers.  
   ```rust  
   use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};  

   lazy_static! {  
       static ref IDT: InterruptDescriptorTable = {  
           let mut idt = InterruptDescriptorTable::new();  
           idt.breakpoint.set_handler_fn(breakpoint_handler);  
           // Add more handlers  
           idt  
       };  
   }  
   ```  

2. **Handle keyboard input**:  
   - Program the Programmable Interrupt Controller (PIC).  
   - Read scancodes from port `0x60` and translate to ASCII.  

**Key Learning**:  
- CPU exception handling.  
- Hardware interrupts and PIC/APIC programming.  

---

### **Phase 3: Memory Management (Weeks 5–6)**  
**Goal**: Implement virtual memory and a heap allocator.  
**What You’ll Build**:  
1. A physical frame allocator (track free memory pages).  
2. A page table manager for virtual memory.  
3. A `GlobalAlloc` implementation to enable `Box`, `Vec`, etc.  

**Steps**:  
1. **Physical memory allocator**:  
   Use a bitmap or linked list to track free memory regions.  

2. **Set up paging**:  
   Use the `x86_64` crate’s `PageTable` structs to map virtual addresses.  

3. **Heap allocation**:  
   Implement `alloc::alloc::GlobalAlloc` using a linked list allocator.  

**Key Learning**:  
- Virtual memory and paging.  
- Rust’s allocator API.  

---

### **Phase 4: Process Scheduling (Weeks 7–8)**  
**Goal**: Run multiple user-space tasks.  
**What You’ll Build**:  
1. A task struct to track process state (registers, stack, PID).  
2. A round-robin scheduler.  
3. Context switching using assembly.  

**Steps**:  
1. **Define a task**:  
   ```rust  
   struct Task {  
       regs: Registers,  
       stack: *mut u8,  
       state: TaskState,  
   }  
   ```  

2. **Context switch**:  
   Write assembly to save/restore CPU registers.  
   ```asm  
   global switch_task  
   switch_task:  
       push rbp  
       mov rbp, rsp  
       ; Save registers  
       ; ...  
       ret  
   ```  

**Key Learning**:  
- CPU context switching.  
- Process states and scheduling.  

---

### **Phase 5: File System and Drivers (Weeks 9–12)**  
**Goal**: Read files from disk and run user programs.  
**What You’ll Build**:  
1. AATA disk driver.  
2. A FAT32 file system implementation.  
3. An ELF loader to execute binaries.  

**Steps**:  
1. **ATA driver**:  
   Read sectors by programming I/O ports (`0x1F0–0x1F7`).  

2. **FAT32 parser**:  
   Traverse directories and read files from disk.  

3. **Load ELF binaries**:  
   Parse ELF headers and map the binary into memory.  

**Key Learning**:  
- Disk I/O and file systems.  
- Executable file formats.  

---

### **Phase 6: User Space and Syscalls (Weeks 13–16)**  
**Goal**: Run a simple user program that makes syscalls.  
**What You’ll Build**:  
1. A `libc`-like library in Rust.  
2. Syscall interface (e.g., `sys_write`, `sys_exit`).  
3. A shell that can launch programs.  

**Steps**:  
1. **Syscall handler**:  
   Use `syscall`/`sysret` instructions to switch between user/kernel mode.  

2. **Build a shell**:  
   Read input from the keyboard and execute programs.  

**Key Learning**:  
- User/kernel mode isolation.  
- Syscall mechanics.  

---

### **Iteration Cycle**  
1. **Build**: Code a minimal version of each component.  
2. **Test**: Use QEMU to emulate and debug.  
3. **Break**: Debug using QEMU’s GDB stub.  
4. **Repeat**: Refactor and add features incrementally.  

---

### **Tools to Use**  
- **Emulator**: QEMU (`qemu-system-x86_64`).  
- **Debugging**: `gdb` with QEMU’s `-s -S` flags.  
- **Rust Crates**:  
  - `x86_64`: CPU structures.  
  - `bootloader`: Boot your kernel.  
  - `spin`: No-std synchronization.  

---

### **Example Project Timeline**  
| **Week** | **Task**                                      |  
|----------|-----------------------------------------------|  
| 1–2      | Boot and print "Hello World".                 |  
| 3–4      | Handle keyboard input and exceptions.         |  
| 5–6      | Implement virtual memory and heap.            |  
| 7–8      | Run two tasks with a round-robin scheduler.   |  
| 9–10     | Read files from a FAT32 disk.                 |  
| 11–12    | Launch a user program (ELF).                  |  
| 13–14    | Add `sys_write` and `sys_exit` syscalls.      |  
| 15–16    | Build a shell.                                |  

---

### **Mindset Tips**  
1. **Start small**: A kernel that crashes but prints an error is progress.  
2. **Copy-paste code**: Borrow from tutorials (e.g., Phil Oppermann’s blog) and refactor later.  
3. **Break it often**: If you’re not triggering triple faults, you’re not trying hard enough.  

By the end of this plan, you’ll have a Unix-like microkernel that can run basic programs. After that, extend it with networking, GUI, or port it to ARM!
