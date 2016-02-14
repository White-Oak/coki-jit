extern crate libc;

use std::mem;
use std::ops::{Index, IndexMut};

extern "C" {
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

#[cfg(windows)]
mod memory {
    extern crate libc;
    extern crate winapi;
    extern crate kernel32;

    extern "C" {
        fn _aligned_malloc(size: libc::size_t, alignment: libc::size_t) -> *mut libc::c_void;
    }

    pub unsafe fn aligned_malloc(size: libc::size_t, alignment: libc::size_t) -> *mut libc::c_void {
        _aligned_malloc(size, alignment)
    }

    pub unsafe fn make_executable(addr: *mut libc::c_void, size: libc::size_t) {
        let mut _previous_protect: *mut u32 = &mut 0u32 as *mut u32;
        kernel32::VirtualProtect(addr as *mut ::std::os::raw::c_void,
                                 size as u64,
                                 winapi::winnt::PAGE_EXECUTE_READWRITE,
                                 _previous_protect as u32);
    }

    pub unsafe fn free_memory(addr: *mut libc::c_void) {
        libc::free(addr);
    }
}

#[cfg(unix)]
mod memory {
    extern crate libc;
    extern crate alloc;

    pub unsafe fn aligned_malloc(size: usize, alignment: usize) -> *mut u8 {
        alloc::heap::allocate(size, alignment)
    }

    pub unsafe fn make_executable(addr: *mut libc::c_void, size: libc::size_t) {
        libc::mprotect(addr,
                       size,
                       libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
    }

    pub unsafe fn free_memory(addr: *mut u8, size: usize, alignment: usize) {
        alloc::heap::deallocate(addr, size, alignment)
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
            let _contents = memory::aligned_malloc(size, PAGE_SIZE) as *mut libc::c_void;
            memory::make_executable(_contents, size);

            memset(_contents, 0xc3, size);  //prepopulate with 'RET'

            memset(_contents.offset(VARIABLE_OFFSET as isize),
                   0,
                   VARIABLE_MEMORY_SIZE); //set variable memory area with zeros
            _contents as *mut u8
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

    fn index(&self, _index: usize) -> &u8 {
        unsafe { &*self.contents.offset(_index as isize) }
    }
}

impl IndexMut<usize> for JitMemory {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe { &mut *self.contents.offset(_index as isize) }
    }
}

pub const OUTPUT_OFFSET: usize = 2000;
pub const VARIABLE_OFFSET: usize = 1000;
pub const VARIABLE_MEMORY_SIZE: usize = 1000;
fn print_output(jit: &JitMemory) {
    let mut acc: i64 = 0;
    let mut i = 0;
    let mut ret_flag = true;
    loop {
        let value = jit[i + OUTPUT_OFFSET] as i64;
        acc += value << ((i % 8) * 8);
        if value != 0xc3 {
            ret_flag = false; //if all 8 bytes are filled with 'ret'
        }
        print!("{:x} ", value);
        if (i + 1) % 8 == 0 {
            println!(" as qword: {}", acc);
            if ret_flag {
                break; //ret
            }
            acc = 0;
            ret_flag = true;
        }
        i += 1;
    }
}
fn jit_wrap(fun: fn(), jit: &JitMemory) {
    fun();
    print_output(jit);
}

pub fn get_jit(bytes: &[u8]) -> Box<Fn()> {
    let mut jit: JitMemory = JitMemory::new(1);
    for &byte in bytes {
        jit.add(byte);
    }
    println!("Program loaded into the memory");
    let fun = unsafe { mem::transmute(jit.contents) };
    Box::new(move || jit_wrap(fun, &jit))
}
