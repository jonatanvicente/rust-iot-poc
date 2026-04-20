
<img align="right" width="100" height="100" src="rust-logo-blk.svg">

# Rust IoT Proof-Of-Concept

This is a classic "Edge to Gateway" setup. In this proof of concept (PoC), we’ll set up the Raspberry Pi as a TCP Server (the listener) and the ESP32 as a TCP Client (the talker).
The ESP32 will connect to the Pi's IP address on a specific port and send a "Hello" message.

Hardware required:
- Raspberry Pi (any model should work, but a Pi 3 or later is recommended). We'll use DietPi as OS for RPi.
- ESP32 development board (e.g., ESP32 DevKitC)

## Project Structure

``` 
rust-iot-poc/               
├── crates/
│   ├── gateway/            <-- Server
│   │   ├── Cargo.toml      <-- Gateway-specific dependencies
│   │   └── src/
│   │       └── main.rs     
│   ├── firmware/           <-- ESP32 code
│   └── common/             <-- Shared Data Structs
└── Cargo.toml              <-- Workspace definition
```

## Raspberry Pi Server (Gateway)

The Pi will sit and wait for incoming messages. We'll use the standard library std::net for this.

### Deploy the server into DietPi

Compiling directly on a Raspberry Pi (especially older ones or Zeros) can be very slow. It is better to cross-compile from your main computer.

1. **Compile** the code:
   - Compiling with **cargo**: First **install the Target**: (Assuming a 64-bit Pi 4/5 or Zero 2 running 64-bit DietPi)

   ``` rust
            rustup target add aarch64-unknown-linux-gnu 
   ```
   - Compiling with **cross** (easiest). 

   ``` rust
            cargo install cross
            cross build --target aarch64-unknown-linux-gnu --release
   ```
              
  **Breaking down the path:**
   - **target/**: The standard folder where Rust puts all temporary and final build files.
   - **aarch64-unknown-linux-gnu/**: Since you cross-compiled, Cargo creates a subfolder specifically for that architecture. This prevents ARM binaries from getting mixed up with Fedora (x86_64) binaries.
   - **release/**: Because you used the --release flag, the optimized binary is placed here. (If you didn't use the flag, it would be in debug/).
   - **gateway**: This is the actual executable file. Its name comes from the name = "gateway" field in your crates/gateway/Cargo.toml.

2. **Transfer to DietPi**

``` bash
scp target/aarch64-unknown-linux-gnu/release/your_project_name root@<PI_IP_ADDRESS>:/home/dietpi/
``` 

3. **Folding into RPi** (optional): You can create a folder for the gateway on the Pi and move the binary there:
   
   - /usr/local/bin/iot-bridge/gateway (or any path you prefer)

4. **Set permissions**: 

```bash
sudo chown iotuser:iotuser /usr/local/bin/iot-bridge/gateway 
sudo chmod +x /usr/local/bin/iot-bridge/gateway
```

6. **Run** it. SSH into the Pi and run:

``` rust
chmod +x your_project_name
./usr/local/bin/iot-bridge/gateway
```
4. Configuration detail (Raspberry Pi):
See [README_config.md](README_config.md) for details on how to set up the server to start at boot time and run as a service.


### Tests over Gateway

* Send a message simulating a ESP32 client:

```bash
mosquitto_pub -h 192.168.1.62 -t "esp32/sensor" -m "Test-Data-123"
```

* From another cmd window, subscribe to the topic to see the message:

```bash
mosquitto_sub -h 192.168.1.62 -t "esp32/sensor"
```
You can see the message "Test-Data-123" in the subscriber window, confirming that the gateway is receiving and processing messages correctly.


## ESP32 (Edge Device)

### Configuration

* ESP32 needs to know your WiFi credentials and the Pi's IP address, we’ll use a cfg.toml file or environment variables.
* Make sure you have the ESP32 tools installed:

```bash
espup install
# Source the export file (usually in ~/export-esp.sh)
. $HOME/export-esp.sh
```

### Firmware code

- Wifi Connection
- MQTT Client setup
- Loop (Publising data)

### Flashing the ESP32

1. Connect your ESP32 to your computer via USB, and find your device (usually `/devttyUSB0` or `/dev/ttyACM0` on Linux, COM3 or similar on Windows).
2. Flash and monitor. Run this from the root of your workspace:

```bash
cargo espflash flash -p firmware --monitor # monitor opens a serial terminal to see the output from the ESP32
```

### Verify the Whole Chain

1. Check the logs. `sudo journalctl -u iot-bridge -f`
2. Check the Web API. `http:192.168.1.62:3000` shoulds show the latest count sent by the ESP32.


[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct_EN.md)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct_ES.md)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct_CA.md) 
