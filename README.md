# Megatec UPS Control Library (Rust)

A Rust library for interfacing with Megatec protocol-compatible UPS (Uninterruptible Power Supply) devices via USB. This library provides a safe, high-level interface for monitoring and controlling UPS devices.

The library was created from scratch, initially in C, but rewritten in Rust. Through reverse engineering and in-depth analysis of the UPSilon 2000 program, I discovered how the program communicates with the UPS device.

This project is the only library in the world that has collected all the functionalities of the UPSilon 2000 program, making them open-source. The library supports the Mega(USB) protocol created by Mega System Technologies, Inc.

## Features

- USB device connection management
- Real-time UPS status monitoring
- Battery testing capabilities
- Beep control
- Rating information retrieval
- Emergency shutdown control

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
megatec-ups-control = "0.1.0"
```

## Links

- You can find more information at [Crates.io](https://crates.io/crates/megatec-ups-control)
- See full documentation [Docs.rs](https://docs.rs/megatec-ups-control/0.1.0/megatec_ups_control/)

## Usage

Basic example:

```rust
use megatec_ups_control::{MegatecUps, Result, UpsStatus};

fn main() -> Result<()> {
    // Create a new UPS connection
    // Replace these with your actual vendor and product IDs
    let ups = match MegatecUps::new(0x0001, 0x0000) {
        Ok(ups) => {
            println!("Successfully connected to UPS device");
            ups
        }
        Err(e) => {
            println!("Failed to connect to UPS device: {:?}", e);
            return Err(e);
        }
    };

    // Get UPS name
    let name: String = ups.get_name()?;
    println!("UPS Name: {}", name);

    // Get UPS status
    let status: UpsStatus = ups.get_status()?;
    println!("UPS Status:");
    println!("  Input Voltage: {} V", status.input_voltage);
    println!("  Input Fault Voltage: {} V", status.input_fault_voltage);
    println!("  Output Voltage: {} V", status.output_voltage);
    println!("  Output Current: {}%", status.output_current);
    println!("  Input Frequency: {} Hz", status.input_frequency);
    println!("  Battery Voltage: {} V", status.battery_voltage);
    println!("  Temperature: {} °C", status.temperature);

    // Perform a 10-second test
    println!("Performing 10-second test...");
    ups.test()?;

    Ok(())
}
```

## API Reference

### Main Types

#### `MegatecUps`
Main structure for interacting with the UPS device.

```rust
let ups = MegatecUps::new(vendor_id, product_id)?;
```

#### `UpsStatus`
Structure containing UPS status information:
- `input_voltage`: Input voltage (V)
- `input_fault_voltage`: Input fault voltage (V)
- `output_voltage`: Output voltage (V)
- `output_current`: Output current (%)
- `input_frequency`: Input frequency (Hz)
- `battery_voltage`: Battery voltage (V)
- `temperature`: Temperature (°C)

### Key Methods

#### Device Information
- `get_name()` - Get UPS name
- `get_rating()` - Get UPS rating information
- `get_status()` - Get UPS status with acknowledgment
- `get_status_no_ack()` - Get UPS status without acknowledgment

#### Testing Functions
- `test()` - Perform 10-second test
- `test_until_battery_low()` - Test until battery is low
- `test_with_time(minutes)` - Test for specified duration
- `abort_test()` - Abort current test

#### Control Functions
- `switch_beep()` - Toggle UPS beep
- `shutdown()` - Initiate UPS shutdown (1-minute delay)

## Error Handling

The library uses a custom error type `UpsError` with the following variants:
- `Usb(UsbError)` - USB communication errors
- `InvalidResponse` - Invalid or unexpected device response
- `InvalidTime` - Invalid time value for testing

## Test Duration Calculation

The library includes a special algorithm for calculating test durations:
- 1-9 minutes: values 101-109
- 10-19 minutes: values 125-134
- 20-99 minutes: calculated using range-based formula

## Building from Source

```bash
# Clone the repository
git clone https://github.com/piotrmaciejbednarski/megatec-ups-control
cd megatec-ups-control

# Build the library
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
