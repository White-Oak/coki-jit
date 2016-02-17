extern crate libc;

use std::mem;
use std::ops::{Index, IndexMut};

extern "C" {
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

mod memory {
    extern crate libc;
    extern crate alloc;
    
    pub unsafe fn aligned_malloc(size: libc::size_t, alignment: libc::size_t) -> *mut u8 {
        alloc::heap::allocate(size, alignment)
    }

    pub unsafe fn free_memory(addr: *mut u8, size: usize, alignment: usize) {
        alloc::heap::deallocate(addr, size, alignment)
    }

    #[cfg(unix)]
    pub unsafe fn make_executable(addr: *mut libc::c_void, size: libc::size_t) {
        libc::mprotect(addr,
                       size,
                       libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
    }

    #[cfg(windows)]
    pub unsafe fn make_executable(addr: *mut libc::c_void, size: libc::size_t) {
        extern crate winapi;
        extern crate kernel32;

        let mut _previous_protect: *mut u32 = &mut 0u32 as *mut u32;
        kernel32::VirtualProtect(addr as *mut ::std::os::raw::c_void,
                                 size as u64,
                                 winapi::winnt::PAGE_EXECUTE_READWRITE,
                                 _previous_protect as u32);
    }
}

const PAGE_SIZE: usize = 4096;

struct JitMemory {
    contents: *mut u8,
    counter: usize,
}
impl Drop for JitMemory {
    fn drop(&mut self) {
        println!("Dropping JIT");
        unsafe {
            memory::free_memory(self.contents, PAGE_SIZE, PAGE_SIZE);
        }
    }
}

impl JitMemory {
    fn new(num_pages: usize) -> JitMemory {
        let contents = unsafe {
            let size = num_pages * PAGE_SIZE;
            let void_contents = memory::aligned_malloc(size, PAGE_SIZE) as *mut libc::c_void;
            memory::make_executable(void_contents, size);

            memset(void_contents, 0xc3, size);  //prepopulate with 'RET'

            memset(void_contents.offset(VARIABLE_OFFSET as isize),
                   0,
                   VARIABLE_MEMORY_SIZE); //set variable memory area with zeros
            void_contents as *mut u8
        };

        JitMemory {
            contents: contents,
            counter: 0,
        }
    }

    fn add(&mut self, byte: u8) {
        let c = self.counter;
        self[c] = byte;
        self.counter = c + 1;
    }
}

impl Index<usize> for JitMemory {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        unsafe { &*self.contents.offset(index as isize) }
    }
}

impl IndexMut<usize> for JitMemory {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        unsafe { &mut *self.contents.offset(index as isize) }
    }
}

unsafe extern "win64" fn print_from_asm(mut a: u64){
    use libc::{c_void, write};
    //Cleaning from `ret`
    for i in 0..8 {
        let res = (a >> (i * 8)) & 0xff;
        if res == 0xcc {
            a &= !((0xff << (i * 8)) as u64);
        }
    }
    //42 to 2, 4, 0, 0 ...
    let mut result = Vec::new();
    while a > 0 {
        result.push((a % 10 + 48) as u8);
        a /= 10;
    }
    result.reverse();
    let ptr: *const c_void = result.as_ptr() as *const c_void;
    write(1, ptr, result.len());
    // \r\n
    let ptr: *const c_void = &0xd0a as *const _ as *const c_void;
    write(1, ptr, 2);
}

pub const OUTPUT_OFFSET: usize = 2000;
pub const VARIABLE_OFFSET: usize = 1000;
pub const VARIABLE_MEMORY_SIZE: usize = 1000;
pub const PRINT_FUNCTION: *const u8 = print_from_asm as *const u8;

pub fn get_jit(bytes: &[u8]) -> fn() -> u64 {
    let mut jit: JitMemory = JitMemory::new(1);
    for &byte in bytes {
        jit.add(byte);
    }
    println!("Program loaded into the memory");
    unsafe { mem::transmute(jit.contents) }
}
