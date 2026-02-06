#![no_std]
#![no_main]

use core::fmt::Write;
const VGA_BASE: usize = 0xb8000;
static mut VGA: *mut u8 = VGA_BASE as *mut u8;

struct Vga;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("hello calvin");
    panic!()
}

impl core::fmt::Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.bytes().for_each(|byte| unsafe {
            if byte == b'\n' {
                let offset = VGA as usize - VGA_BASE;
                let next_line = offset.next_multiple_of(160);
                VGA = (VGA_BASE + next_line) as *mut u8;
            } else {
                VGA.write_volatile(byte);
                VGA = VGA.add(1);
                VGA.write_volatile(0xF);
                VGA = VGA.add(1);
            }
        });
        Ok(())
    }
}

pub fn _print(args: core::fmt::Arguments) {
    Vga.write_fmt(args).unwrap();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        println!(
            "\nPANIC AT {}:{}:{}: {}",
            loc.file(),
            loc.line(),
            loc.column(),
            info.message()
        );
    } else {
        println!("\n PANIC: {}", info.message());
    }
    loop {}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::_print(core::format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    );
}
