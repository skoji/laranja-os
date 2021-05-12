use crate::debug;

mod registers;
mod simple_alloc;

use registers::*;

use self::simple_alloc::SimpleAlloc;

const MEM_POOL_SIZE: usize = 4 * 1024 * 1024;
static ALLOC: spin::Mutex<simple_alloc::SimpleAlloc<MEM_POOL_SIZE>> =
    spin::Mutex::new(SimpleAlloc::new());

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
        let alloc = ALLOC.lock();

        Controller {
            cap_regs,
            op_regs,
            doorbell_first,
        }
    }
}
