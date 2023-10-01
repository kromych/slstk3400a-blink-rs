# Example applications for SLSTK3400A

This example shows how to log via RTT and blink an LED on
[the SLSTK3400A starter kit board](https://www.silabs.com/development-tools/mcu/32-bit/efm32hg-starter-kit)

## Prepare

One-time tool installation:

```sh
cargo install flip-link
# Wait for the updated version with support for EFM32 Happy Gecko to be released
# or use the main branch of https://github.com/probe-rs/probe-rs.
cargo install cargo-embed
cargo install cargo-flash
cargo install probe-rs-cli
cargo install probe-rs-debugger
cargo install probe-run
```

## Enjoy

Run with

```sh
cargo run --release
```

In the terminal, there should be something resembling this:

```text
     Running `probe-run --chip EFM32HG322F64 target/thumbv6m-none-eabi/release/slstk3400a-blink-rs`
(HOST) INFO  flashing program (5 pages / 5.00 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
INFO  Hello, world!
└─ slstk3400a_blink_rs::__cortex_m_rt_main @ src/main.rs:21
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
