use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;


// RISC-V调用规范中，约定寄存器 a0~a6 保存系统调用的参数， a0 保存系统调用的返回值。
// 寄存器 a7 用来传递 syscall ID
// 以上超出rust语言的能力，需要使用内联汇编来实现
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        // 由于编译器的能力不足以判定插入汇编代码这个行为的安全性，所以我们需要将其包裹在 unsafe 块中自己来对它负责。
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret, // 汇编代码执行后，寄存器的值会被自动保存回Rust变量
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

/// 功能：将内存中缓冲区中的数据写入文件。
/// 参数：`fd` 表示待写入文件的文件描述符；
///      `buf` 表示内存中缓冲区的起始地址；
///      `len` 表示内存中缓冲区的长度。
/// 返回值：返回成功写入的长度。
/// syscall ID：64
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// 功能：退出应用程序并将返回值告知批处理系统。
/// 参数：`exit_code` 表示应用程序的返回值。
/// 返回值：该系统调用不应该返回。
/// syscall ID：93
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}
