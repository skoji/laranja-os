use crate::debug;

#[repr(C, packed(4))]
pub struct CapabilityRegisters {
    cap_length: u8,
    reserved: u8,
    hci_version: u16,
    hcs_params1: u32,
    hcs_params2: u32,
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
            "cap_length: {}, hci_version: 0x{:02x}, hcs_params1: 0x{:08x}, hcs_params2: 0x{:08x}, hcs_params3: 0x{:08x}, hcc_params1: 0x{:08x}, db_off: 0x{:08x}, rts_off: 0x{:08x}, hcc_params2: 0x{:08x}",
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
        .unwrap();
        Ok(())
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
