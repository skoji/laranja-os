use crate::debug;

#[repr(C, packed(4))]
struct CapabilityRegisters {
    cap_length: u8,
    reserved: u8,
    hci_version: u16,
    hcs_params1: HscParam1,
    hcs_params2: HscParam2,
    hcs_params3: u32,
    hcc_params1: HccParams1,
    db_off: u32,
    rts_off: u32,
    hcc_params2: u32,
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
            self.db_off & 0xfff0,
            self.rts_off & 0xffe0,
            self.hcc_params2
        )
    }
}

#[repr(C)]
struct HscParam1 {
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
struct HscParam2 {
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
struct HccParams1 {
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
#[derive(Debug)]
pub struct OperationalRegisters {
    usbcmd: u32,
    usbsts: u32,
    pagesize: u32,
    _rsvd_1: [u32; 2],
    dnctrl: u32,
    crcr: u64,
    _rsvd_2: [u32; 4],
    dcbaap: u64,
    config: u32,
}

pub struct Doorbell {
    data: u32,
}

impl Doorbell {
    pub fn set_db_target(&mut self, target: u8) {
        self.data = self.data & 0xfff0 | target as u32;
    }

    pub fn set_db_stream_id(&mut self, stream_id: u16) {
        self.data = self.data & 0x00ff | (stream_id as u32) << 16;
    }
}

pub struct Controller<'a> {
    cap_regs: &'a mut CapabilityRegisters,
    op_regs: &'a mut OperationalRegisters,
    doorbell_first: *mut Doorbell,
}

impl<'a> Controller<'a> {
    /// # Safety
    /// mmio_base must be a valid base address for xHCI device MMIO
    pub unsafe fn new(mmio_base: usize) -> Self {
        let cap_regs = &mut *(mmio_base as *mut CapabilityRegisters);
        debug!("cap regs: {}", cap_regs);
        let op_regs =
            &mut *((mmio_base + cap_regs.cap_length as usize) as *mut OperationalRegisters);
        debug!("op_regs: {:?}", op_regs);
        let doorbell_first = (mmio_base + (cap_regs.db_off & 0xfff0) as usize) as *mut Doorbell;
        Controller {
            cap_regs,
            op_regs,
            doorbell_first,
        }
    }
}
