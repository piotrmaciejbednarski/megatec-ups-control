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
