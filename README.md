# Project Title

Winky-rp-rs

## Description

A cargo project to blink the onboard LED on a Pi Pico H and an external LED alternately.

## Getting Started

### Dependencies

* Requires cargo and rustup to be installed
* Requires a Pi Pico H, connected either to a probe or via Micro USB to a computer.

### Installing

* Clone the repository using git clone
* The project requires the Arm Cortex M0/1 cross compilation target, you can install this using:

```
rustup target add thumbv6m-none-eabi
```

### Executing program

* To flash the Pi Pico H (Using a micro-USB cable), plug it into the computer whilst holding down the BOOTSEL button
* Then in your terminal run `cargo run --release --bin winky`

## Authors

Contributors names and contact info

Jared Forte 
[@V4UIDev](https://github.com/V4UIDev)

## Acknowledgments

* Based on the example file from the [Embassy](https://github.com/embassy-rs/embassy) project