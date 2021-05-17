#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

pub mod cmd;
pub mod mtrf;

use anyhow::Error;
use serialport;
use serialport::SerialPortType;

pub const MANUFACTURER: &str = "FTDI";
pub const PRODUCT: &str = "FT232R USB UART";

#[derive(Debug)]
pub struct PortInfo {
    pub port_name: String,
    pub serial_number: Option<String>,
}

pub fn ports() -> Result<Vec<PortInfo>, Error> {
    Ok(serialport::available_ports()?
        .into_iter()
        .filter_map(|port| match port.port_type {
            SerialPortType::UsbPort(usb) => {
                if let Some(manufacturer) = usb.manufacturer {
                    if manufacturer != MANUFACTURER {
                        return None;
                    }
                }
                if let Some(product) = usb.product {
                    if product != PRODUCT {
                        return None;
                    }
                }
                Some(PortInfo {
                    port_name: port.port_name,
                    serial_number: usb.serial_number,
                })
            }
            _ => None,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::ports;

    // Works only when MTRF-64 is plugged in.
    #[test]
    pub fn test_ports() {
        assert_eq!(ports().unwrap().len(), 1);
    }
}
