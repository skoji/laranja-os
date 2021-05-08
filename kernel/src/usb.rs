use crate::debug;

#[repr(C, packed(4))]
pub struct CapabilityRegisters {
    cap_length: u8,
    reserved: u8,
    hci_version: u16,
    hcs_params1: HscParam1,
    hcs_params2: HscParam2,
    hcs_params3: u32,
    hcc_params1: u32,
    db_off: u32,
    rts_off: u32,
    hcc_params2: u32,
}

impl core::fmt::Display for CapabilityRegisters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "cap_length: {}, hci_version: 0x{:02x}, hcs_params1: {}, hcs_params2: {}, hcs_params3: 0x{:08x}, hcc_params1: 0x{:08x}, db_off: 0x{:08x}, rts_off: 0x{:08x}, hcc_params2: 0x{:08x}",
            self.cap_length,
            self.hci_version,
            self.hcs_params1,
            self.hcs_params2,
            self.hcs_params3,
            self.hcc_params1,
            self.db_off,
            self.rts_off,
            self.hcc_params2
        )
    }
}

#[repr(C, packed(4))]
pub struct HscParam1 {
    data: u32,
}

impl HscParam1 {
    pub fn new(data: u32) -> Self {
        HscParam1 { data }
    }
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

#[repr(C, packed(4))]
pub struct HscParam2 {
    data: u32,
}

impl HscParam2 {
    pub fn new(data: u32) -> Self {
        HscParam2 { data }
    }

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

pub struct Controller {}

impl Controller {
    /// # Safety
    /// mmio_base must be a valid base address for xHCI device MMIO
    pub unsafe fn new(mmio_base: usize) -> Self {
        let cap_regs = mmio_base as *mut CapabilityRegisters;
        debug!("cap regs: {}", &*cap_regs);
        Controller {}
    }
}
