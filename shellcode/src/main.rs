#![no_std]
#![no_main]

use core::arch::asm;

#[panic_handler]
fn panic_(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const SYS_EXECVE: usize = 59;
const SHELL: &str = "/bin/sh\x00";
const ARGV: [*const &str; 2] = [&SHELL, core::ptr::null()];
const NULL_ENV: usize = 0;

unsafe fn syscall3(syscall: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    const SYS_EXECVE: usize = 59;
    const SHELL: &str = "/bin/sh\x00";
    const ARGV: [*const &str; 2] = [&SHELL, core::ptr::null()];
    const NULL_ENV: usize = 0;
}

#[no_mangle]
fn _start() {
    unsafe {
        syscall3(SYS_EXECVE, SHELL.as_ptr() as usize, ARGV.as_ptr() as usize, NULL_ENV);
    }
}