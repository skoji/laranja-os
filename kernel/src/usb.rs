use crate::debug;

mod registers;
use registers::*;

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
        let doorbell_first =
            (mmio_base + (cap_regs.db_off & 0xffff_fffc) as usize) as *mut Doorbell;
        Controller {
            cap_regs,
            op_regs,
            doorbell_first,
        }
    }
}
