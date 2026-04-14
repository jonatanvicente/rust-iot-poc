
<img align="right" width="100" height="100" src="rust-logo-blk.svg">

# Rust IoT Proof-Of-Concept

This is a classic "Edge to Gateway" setup. In this proof of concept (PoC), we’ll set up the Raspberry Pi as a TCP Server (the listener) and the ESP32 as a TCP Client (the talker).
The ESP32 will connect to the Pi's IP address on a specific port and send a "Hello" message.

Hardware required:
- Raspberry Pi (any model should work, but a Pi 3 or later is recommended). We'll use DietPi as OS for RPi.
- ESP32 development board (e.g., ESP32 DevKitC)



## Raspberry Pi Server (Gateway)

The Pi will sit and wait for incoming messages. We'll use the standard library std::net for this.

#### Deploy the server into DietPi

Compiling directly on a Raspberry Pi (especially older ones or Zeros) can be very slow. It is better to cross-compile from your main computer.

1. **Install the Target**: (Assuming a 64-bit Pi 4/5 or Zero 2 running 64-bit DietPi)

``` rust
rustup target add aarch64-unknown-linux-gnu 
```

2. Use **cross**: The easiest way to handle linkers is a tool called cross.

``` rust
   cargo install cross
   cross build --target aarch64-unknown-linux-gnu --release
```
3. **Transfer to DietPi**

``` rust
scp target/aarch64-unknown-linux-gnu/release/your_project_name root@<PI_IP_ADDRESS>:/home/dietpi/
``` 
4. **Run** it. SSH into the Pi and run:


``` rust
chmod +x your_project_name
./your_project_name
```



[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct_EN.md)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct_ES.md)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct_CA.md) 
