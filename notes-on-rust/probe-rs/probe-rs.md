# Probe-RS

Probe-RS is an embedded debugging toolkit written in Rust.

## Tools Introduction

- cargo-flash is a cargo extension that replaces `cargo run`
- cargo-embed is a more advanced version of cargo-flash supporting RTT terminal and GDB server
- probe-rs VS Code extension enabling interactive debugging

### Cargo Embed

`cargo-embed` is a crate that provides cargo subcommands to work with Rust on embedded targets.

The basic functionality is to flash targets via a debug probe. Probes supported are for example DAPLink, ST-Link, and J-link. Targets supported are for example nRF5x, STM32, and LPC800.
