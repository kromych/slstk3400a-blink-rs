# Example application for SLSTK3400A

This example shows how to log via RTT and blink an LED on
[the SLSTK3400A starter kit board](https://www.silabs.com/development-tools/mcu/32-bit/efm32hg-starter-kit)

## Prepare

One-time tool installation:

```sh
cargo install flip-link
# Wait for the updated version with support for EFM32 Happy Gecko
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
