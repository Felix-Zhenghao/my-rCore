## The repo for [chapter 2](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/index.html)

User's applications and user mode library locate in the `user/` directory:
- `user/src/bin/*.rs`: applications that will be executed by the batch system one by one, from smaller index to larger index.
  - `hello_world`: Prints the line "Hello world from user mode program!" on the screen.
  - `store_fault`: Accesses an illegal physical address to test whether the batch processing system is affected by this error.
  - `power`: Continuously alternates between privileged-level switching for computational operations and printing string operations.
- `user/src/*.rs`: user library like entry point, initialization, syscall and I/O.
- `user/src/linker.ld`: memory layout specification for the application.