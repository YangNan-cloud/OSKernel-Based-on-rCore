/*
作为 bin 目录下的源程序所依赖的用户库，等价于其他编程语言提供的标准库。
*/
#![no_std]
#![feature(linkage)]    // 支持#[linkage = "weak"]链接操作

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

// 定义了用户库的入口点
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> !{
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!")
}

/*
我们使用 Rust 的宏将其函数符号 main 标志为弱链接。这样在最后链接的时候，虽然在 lib.rs 和 bin 目录下的某个应用程序都有 main 符号，
但由于 lib.rs 中的 main 符号是弱链接，链接器会使用 bin 目录下的应用主逻辑作为 main 。
这里我们主要是进行某种程度上的保护，如果在 bin 目录下找不到任何 main ，那么编译也能够通过，但会在运行时报错。
 */
#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32{
    panic!("Cannot find main!");
}

fn clear_bss(){
    unsafe extern "C"{
        safe fn start_bss();
        safe fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe{
        (addr as *mut u8).write_volatile(0);
    });
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }