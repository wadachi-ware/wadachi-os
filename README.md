# wadachi-os

RISC-V OS

## Build
```
cargo build
```
riscv の toolchain が必要

## Running
```
cargo run
```
でカーネルが起動する。

```
cargo gdbserver
```
でgdbserverを立ち上げ、CPUを待機させる。
いずれもqemuが必要。


