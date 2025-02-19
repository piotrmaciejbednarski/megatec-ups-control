use rusb::{Context, DeviceHandle, Error as UsbError, UsbContext};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpsError {
    #[error("USB error: {0}")]
    Usb(#[from] UsbError),
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Invalid time value")]
    InvalidTime,
}

pub type Result<T> = std::result::Result<T, UpsError>;

const ASCII_MIN: u8 = 32;
const ASCII_MAX: u8 = 126;
const CHAR_QUOTE: u8 = 34;
const CHAR_BACKTICK: u8 = 96;
const CHAR_PAREN: u8 = 40;

/// Main structure for interacting with a Megatec UPS device
pub struct MegatecUps {
    handle: DeviceHandle<Context>,
    context: Context,
}

impl MegatecUps {
    /// Create a new UPS connection using vendor_id and product_id
    pub fn new(vendor_id: u16, product_id: u16) -> Result<Self> {
        let context = Context::new()?;
        let handle = context
            .open_device_with_vid_pid(vendor_id, product_id)
            .ok_or(UpsError::InvalidResponse)?;

        Ok(Self { handle, context })
    }

    /// Get a string descriptor from the device
    fn get_string_descriptor(&self, index: u8, length: u16) -> Result<String> {
        let mut data = vec![0u8; length as usize];
        let result = self.handle.read_control(
            rusb::request_type(
                rusb::Direction::In,
                rusb::RequestType::Standard,
                rusb::Recipient::Device,
            ),
            rusb::constants::LIBUSB_REQUEST_GET_DESCRIPTOR,
            (rusb::constants::LIBUSB_DT_STRING as u16) << 8 | index as u16,
            0,
            &mut data,
            Duration::from_secs(1),
        )?;

        if result >= 3 {
            let filtered: String = data
                .into_iter()
                .filter(|&c| Self::is_valid_char(c))
                .map(|c| c as char)
                .collect();
            Ok(filtered)
        } else {
            Err(UpsError::InvalidResponse)
        }
    }

    /// Check if a character is valid according to protocol rules
    fn is_valid_char(c: u8) -> bool {
        c >= ASCII_MIN && c <= ASCII_MAX && c != CHAR_QUOTE && c != CHAR_BACKTICK && c != CHAR_PAREN
    }

    /// Get the UPS name
    pub fn get_name(&self) -> Result<String> {
        self.get_string_descriptor(2, 256)
    }

    /// Get the UPS status with acknowledgment
    pub fn get_status(&self) -> Result<UpsStatus> {
        // First request for acknowledgment
        let _ = self.get_string_descriptor(3, 256)?;
        std::thread::sleep(Duration::from_secs(1));

        // Second request for actual status
        let status_str = self.get_string_descriptor(3, 256)?;
        UpsStatus::from_str(&status_str)
    }

    /// Get the UPS status without acknowledgment
    pub fn get_status_no_ack(&self) -> Result<UpsStatus> {
        let status_str = self.get_string_descriptor(3, 256)?;
        UpsStatus::from_str(&status_str)
    }

    /// Test UPS for 10 seconds
    pub fn test(&self) -> Result<()> {
        self.get_string_descriptor(4, 256)?;
        Ok(())
    }

    /// Test UPS until battery is low
    pub fn test_until_battery_low(&self) -> Result<()> {
        self.get_string_descriptor(5, 256)?;
        Ok(())
    }

    /// Test UPS for specified minutes
    pub fn test_with_time(&self, minutes: u8) -> Result<()> {
        let calculated_time = Self::calculate_time(minutes)?;
        self.get_string_descriptor(6, calculated_time)?;
        Ok(())
    }

    /// Toggle UPS beep
    pub fn switch_beep(&self) -> Result<()> {
        self.get_string_descriptor(7, 256)?;
        Ok(())
    }

    /// Abort current UPS test
    pub fn abort_test(&self) -> Result<()> {
        self.get_string_descriptor(11, 256)?;
        Ok(())
    }

    /// Get UPS rating information
    pub fn get_rating(&self) -> Result<String> {
        self.get_string_descriptor(13, 256)
    }

    /// Shutdown UPS after 1 minute
    pub fn shutdown(&self) -> Result<()> {
        self.get_string_descriptor(105, 2460)?;
        Ok(())
    }

    /// Calculate the protocol-specific time value for the test duration
    fn calculate_time(minutes: u8) -> Result<u16> {
        if minutes == 0 || minutes > 99 {
            return Err(UpsError::InvalidTime);
        }

        let value = match minutes {
            1..=9 => 100 + minutes,
            10..=19 => 125 + (minutes - 19),
            20..=99 => {
                let range_start = ((minutes - 20) / 10) * 10 + 20;
                132 + ((minutes - range_start) * 7)
            }
            _ => return Err(UpsError::InvalidTime),
        };

        Ok(value as u16)
    }
}

/// Structure representing the UPS status values
#[derive(Debug, Clone)]
pub struct UpsStatus {
    pub input_voltage: f64,
    pub input_fault_voltage: f64,
    pub output_voltage: f64,
    pub output_current: f64,
    pub input_frequency: f64,
    pub battery_voltage: f64,
    pub temperature: f64,
}

impl UpsStatus {
    /// Parse status string into UpsStatus struct
    fn from_str(status: &str) -> Result<Self> {
        let values: Vec<f64> = status
            .split_whitespace()
            .take(7)
            .map(|s| s.parse::<f64>())
            .collect::<std::result::Result<Vec<f64>, _>>()
            .map_err(|_| UpsError::InvalidResponse)?;

        if values.len() != 7 {
            return Err(UpsError::InvalidResponse);
        }

        Ok(Self {
            input_voltage: values[0],
            input_fault_voltage: values[1],
            output_voltage: values[2],
            output_current: values[3],
            input_frequency: values[4],
            battery_voltage: values[5],
            temperature: values[6],
        })
    }
}

impl Drop for MegatecUps {
    fn drop(&mut self) {
        if let Ok(new_context) = Context::new() {
            let _old_context = std::mem::replace(&mut self.context, new_context);
        }
    }
}
