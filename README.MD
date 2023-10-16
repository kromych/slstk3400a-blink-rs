# Example applications for SLSTK3400A

This example shows how to log via RTT and blink an LED on
[the SLSTK3400A starter kit board](https://www.silabs.com/development-tools/mcu/32-bit/efm32hg-starter-kit)

## Prepare

One-time tool installation:

```sh
cargo install flip-link
cargo install probe-rs --features="cli"
```

It is possible to process the compiled binary with `objcopy` and copy it over to the `STK3400A` disk
that the board presensts when connected through the mini-USB port (the debug one). For that, can do

```sh
# One-time `objdump` and other tools installation
cargo install cargo-binutils
# Cross-platform copy; could use the platform-specific copy utility.
cargo install coreutils

# Every time to deploy
cargo objcopy --release -p blink -- -O binary blink.bin
# Adjust the path to the STK34400 for your system
coreutils cp ./blink.bin /Volumes/STK3400
```

## Enjoy

Run with

```sh
cargo run --release
```

or (for multi-channel RTT logging)

```sh
cargo embed --release
```

In the terminal, there should be something resembling this:

```text
     Running `probe-rs run --chip EFM32HG322F64 target/thumbv6m-none-eabi/release/slstk3400a-blink-rs`
(HOST) INFO  flashing program (5 pages / 5.00 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
INFO  Hello, world!
└─ slstk3400a_blink_rs::__cortex_m_rt_main @ src/main.rs:21
```

To atach to the RTT logging after detaching:

```sh
probe-rs attach --chip EFM32HG322F64 target/thumbv6m-none-eabi/release/slstk3400a-blink-rs
```

## SVD packs

https://www.keil.arm.com/packs/

* [EFM32HG](https://www.silabs.com/documents/public/cmsis-packs/SiliconLabs.GeckoPlatform_EFM32HG_DFP.4.3.0.pack)

## Thank you

It is speculated that if one wants to build something from scratch, they ought to start from creating the Universe itself :)
That said, this project would not have been possible without many great folks on whose efforts this repo has relied.

1. The infrastructure parts of `support/efm32hg322_hal` borrow GPIO infra code from [`efm32hg-hal`](https://github.com/fudanchii/efm32hg-hal).
   Despite its name, it supports just one MCU of the EFM32HG family (EFM32HG309F64), and the means of extending weren't obvious to me.
   As the cleanest approach, I copied as much code as needed, complied with the licensing terms, and added support for EFM32322F64's GPIO.
   Support for the GPIO interrupts has been fixed.

2. The idea of generating the register description comes from [`efm32hg-pac`](https://github.com/em32-rs/efm32hg-pac). Again,
   the project supports just one MCU (EFM32HG309F64) so I procured the SVD file, patched it, and generated the definitions for
   EFM32HG322F64 myself.

3. Incredibly awesome folks who created [`probe-rs`](https://github.com/probe-rs/probe-rs) merged my pull request to support
   EFM32HG322F64.
