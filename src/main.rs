#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use core::arch::asm;

#[no_mangle]
static INIT_SP: [u8; 4096 * 1024] = [0; 4096 * 1024];

#[no_mangle]
static STACK_SIZE: usize = 4096 * 1024; // 4MB

#[link_section = ".entry"]
#[no_mangle]
pub unsafe extern "C" fn _entry() {
    // NOTE: スタックポインタの初期値を設定する
    // NOTE: スタックは下位に伸びていくのでINIT_SP + STACK_SIZEを設定しSTACK_SIZE分の領域を確保
    asm!("la sp, INIT_SP", "ld a0, STACK_SIZE", "add sp, sp, a0",);
    main();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn main() {
    hello();

    loop {}
}

fn hello() {
    let hello_world = vec![
        'H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\n',
    ];

    for c in hello_world {
        print!("{}", c);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!(crate::Writer, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        print!("{}\n", format_args!($($arg)*));
    });
}

pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            unsafe {
                asm!(
                    "ecall",
                    in("a0") c,
                    in("a6") 0,
                    in("a7") 1,
                );
            }
        }
        Ok(())
    }
}

use core::{
    alloc::{GlobalAlloc, Layout},
    cell::{Cell, RefCell},
};

#[global_allocator]
static mut ALLOCATOR: BumpAllocator = BumpAllocator::new();

const ARENA_SIZE: usize = 32 * 1024 * 1024; // 32MB

pub struct BumpAllocator {
    arena: RefCell<[u8; ARENA_SIZE]>,
    next: Cell<usize>,
}

impl BumpAllocator {
    const fn new() -> Self {
        Self {
            arena: RefCell::new([0; ARENA_SIZE]),
            next: Cell::new(0),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let next = self.next.get();

        let size = layout.size();
        let align = layout.align();

        let alloc_start = aligned_addr(next, align);

        let mut arena = self.arena.borrow_mut();
        let alloc_end = alloc_start + size;

        if alloc_end > arena.len() {
            panic!("out of memory");
        }

        self.next.set(alloc_end);

        let ptr = arena.as_mut_ptr();

        return ptr.add(alloc_start);
    }

    // BumpAllocatorはメモリを開放しない
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

fn aligned_addr(addr: usize, align: usize) -> usize {
    if addr % align == 0 {
        addr
    } else {
        addr + align - (addr % align)
    }
}
