use x86_64::instructions::port::{PortReadOnly, PortWriteOnly};

const MAX_DEVICES: u8 = 32;
const MAX_FUNCTIONS: u8 = 8;

const INVALID_VENDOR_ID: u16 = 0xffff;

// TODO; should be thread safe
static mut PCI_CONFIG: PciConfig = PciConfig::new();

struct PciConfig {
    address_port: PortWriteOnly<u32>,
    data_port: PortReadOnly<u32>,
}

impl PciConfig {
    const fn new() -> Self {
        Self {
            address_port: PortWriteOnly::new(0xcf8),
            data_port: PortReadOnly::new(0xcfc),
        }
    }

    fn make_address(bus: u8, device: u8, function: u8, reg_addr: u8) -> u32 {
        1u32 << 31
            | u32::from(bus) << 16
            | u32::from(device) << 11
            | u32::from(function) << 8
            | u32::from(reg_addr & 0xfc)
    }

    pub fn read(&mut self, bus: u8, device: u8, function: u8, reg_addr: u8) -> u32 {
        let addr = PciConfig::make_address(bus, device, function, reg_addr);
        unsafe {
            self.address_port.write(addr);
            self.data_port.read()
        }
    }
}

pub fn read_vendor_id(bus: u8, device: u8, function: u8) -> u16 {
    unsafe { (PCI_CONFIG.read(bus, device, function, 0) & 0xffff) as u16 }
}
