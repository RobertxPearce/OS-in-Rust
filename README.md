# OS in Rust
![Rust OS Hello World](https://github.com/user-attachments/assets/43639f0a-d2fd-453a-a180-3a96296e0f90)


### Build the Freestanding Binary
```bash
cargo build --target thumbv7em-none-eabihf
```
```bash
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

### Booting in QEMU
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin
```
