use crate::{bit_getter, bit_setter, bitwise_macro};

#[repr(C, packed(4))]
pub struct CapabilityRegisters {
    pub cap_length: u8,
    reserved: u8,
    pub hci_version: u16,
    pub hcs_params1: HscParam1,
    pub hcs_params2: HscParam2,
    pub hcs_params3: u32,
    pub hcc_params1: HccParams1,
    pub db_off: u32,
    pub rts_off: u32,
    pub hcc_params2: u32,
}

impl core::fmt::Display for CapabilityRegisters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "cap_length: {}, hci_version: 0x{:02x}, hcs_params1: {}, hcs_params2: {}, hcs_params3: 0x{:08x}, hcc_params1: {}, db_off: 0x{:08x}, rts_off: 0x{:08x}, hcc_params2: 0x{:08x}",
            self.cap_length,
            self.hci_version,
            self.hcs_params1,
            self.hcs_params2,
            self.hcs_params3,
            self.hcc_params1,
            self.db_off & 0xffff_fffc,
            self.rts_off & 0xffff_ffe0,
            self.hcc_params2
        )
    }
}

#[repr(C)]
pub struct HscParam1 {
    data: u32,
}

impl HscParam1 {
    pub fn max_device_slots(&self) -> u8 {
        (self.data & 0xff) as u8
    }

    pub fn max_ports(&self) -> u8 {
        (self.data >> 24 & 0xff) as u8
    }
}

impl core::fmt::Display for HscParam1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "0x{:08x} (slots: {} ports: {})",
            self.data,
            self.max_device_slots(),
            self.max_ports()
        )
    }
}

#[repr(C)]
pub struct HscParam2 {
    data: u32,
}

impl HscParam2 {
    pub fn max_scratchpad_buf(&self) -> usize {
        let hi = (self.data >> 21 & 0b11111) as usize;
        let lo = (self.data >> 27 & 0b11111) as usize;
        (hi << 5) | lo
    }
}

impl core::fmt::Display for HscParam2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "0x{:08x} (max_scratchpad_buf: {})",
            self.data,
            self.max_scratchpad_buf()
        )
    }
}

#[repr(C)]
pub struct HccParams1 {
    data: u32,
}

impl HccParams1 {
    pub fn xecp(&self) -> u16 {
        (self.data >> 16) as u16
    }
}

impl core::fmt::Display for HccParams1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:08x} (xECP: 0x{:08x})", self.data, self.xecp())
    }
}

#[repr(C, packed(4))]
pub struct OperationalRegisters {
    pub usbcmd: UsbCmd,
    pub usbsts: UsbSts,
    pub pagesize: u32,
    pub _rsvd_1: [u32; 2],
    pub dnctrl: u32,
    pub crcr: u64,
    pub _rsvd_2: [u32; 4],
    pub dcbaap: u64,
    pub config: u32,
}

#[repr(C)]
pub struct UsbCmd {
    data: u32,
}

impl UsbCmd {
    bit_setter!(data: u32; 0, pub set_run_stop);
    bit_getter!(data: u32; 0, pub run_stop);
    bit_setter!(data: u32; 1, pub set_host_controller_reset);
    bit_getter!(data: u32; 1, pub host_controller_reset);
    bit_setter!(data: u32; 2, pub set_intterupt_enable);
    bit_getter!(data: u32; 2, pub intterupt_enable);
    bit_setter!(data: u32; 3, pub set_host_system_error_enable);
    bit_getter!(data: u32; 3, pub host_system_error_enable);
    bit_setter!(data: u32; 10, pub set_enable_wrap_event);
    bit_getter!(data: u32; 10, pub enable_wrap_event);
}

#[repr(C)]
pub struct UsbSts {
    data: u32,
}

impl UsbSts {
    bit_getter!(data:u32; 0, pub hc_halted);
}

pub struct Doorbell {
    data: u32,
}

impl Doorbell {
    pub fn set_db_target(&mut self, target: u8) {
        self.data = self.data & 0xffff_fff0 | target as u32;
    }

    pub fn set_db_stream_id(&mut self, stream_id: u16) {
        self.data = self.data & 0x0000_ffff | (stream_id as u32) << 16;
    }
}
