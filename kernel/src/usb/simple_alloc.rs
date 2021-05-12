use core::ptr::{slice_from_raw_parts_mut, NonNull};

#[repr(C, align(4096))]
pub struct SimpleAlloc<const N: usize> {
    pool: [u8; N],
    current: usize,
    end: usize,
}

const BOUNDARY: usize = 4096;

impl<const N: usize> SimpleAlloc<N> {
    pub const fn new() -> Self {
        let pool = [0; N];
        Self {
            pool,
            current: 0,
            end: 0,
        }
    }

    fn do_initialize(&mut self) {
        if self.current == 0 && self.end == 0 {
            self.current = self.pool.as_ptr() as usize;
            self.end = self.current + N;
        }
    }

    // roundup to alignment; only effective when val is power of two
    fn roundup(addr: usize, alignment: usize) -> usize {
        (addr + alignment - 1) & !(alignment - 1)
    }

    pub fn alloc_mem(&mut self, size: usize, alignment: usize) -> Option<NonNull<[u8]>> {
        self.do_initialize();
        let mut ptr = Self::roundup(self.current, alignment);
        let next_boundary = Self::roundup(self.current, BOUNDARY);
        if next_boundary < ptr + size {
            ptr = next_boundary;
        }

        if self.end < ptr + size {
            // overflow
            None
        } else {
            self.current = ptr + size;
            let r = slice_from_raw_parts_mut(ptr as *mut u8, size);
            let r = unsafe { NonNull::new_unchecked(r) };
            Some(r)
        }
    }
}
