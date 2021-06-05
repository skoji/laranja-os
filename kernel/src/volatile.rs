#[repr(transparent)]
pub struct Volatile<T>(T);

impl<T> Volatile<T> {
    pub fn read(&self) -> T {
        unsafe { (self as *const Self as *const T).read_volatile() }
    }

    pub fn write(&mut self, val: T) {
        unsafe { (self as *const Self as *mut T).write_volatile(val) }
    }

    pub fn modify<F: FnOnce(&mut T)>(&mut self, f: F) {
        let mut val = self.read();
        f(&mut val);
        self.write(val);
    }
}
