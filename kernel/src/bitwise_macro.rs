#[macro_export]
macro_rules! bit_setter {
    ($field:tt : $field_type:ty; $vis:vis $name:ident) => {
        $vis fn $name(&mut self, bit: usize, value: bool) {
            let b: $field_type = 1 << bit;
            if (value) {
                self.$field |= b;
            } else {
                self.$field &= !b;
            }
        }
    }
}

#[macro_export]
macro_rules! bit_getter {
    ($field:tt : $field_type:ty; $vis:vis $name:ident) => {
        $vis fn $name(&mut self, bit: usize) -> bool {
            let b: $field_type = 1 << bit;
            (self.$field & b) == b
        }
    }
}

#[cfg(test)]
mod test {
    struct Tester {
        pub data: u32,
        pub data2: u8,
    }

    impl Tester {
        bit_setter!(data: u32; pub set_data);
        bit_setter!(data2: u8; pub set_data2);
        bit_getter!(data: u32; pub get_data);
        bit_getter!(data2: u8; pub get_data2);
    }

    #[test_case]
    fn test_u32_set() {
        let mut t = Tester { data: 0, data2: 0 };
        t.set_data(4, true);
        assert_eq!(t.data, 0b10000);
    }
    #[test_case]
    fn test_u8_set() {
        let mut t = Tester { data: 0, data2: 0 };
        t.set_data2(5, true);
        assert_eq!(t.data2, 0b100000);
    }
    #[test_case]
    fn test_u32_get() {
        let mut t = Tester {
            data: 0b1111_0101_1011_1000_0001_0100_1100_0111,
            data2: 0b1010_0111,
        };
        assert!(t.get_data(7));
        assert!(!t.get_data(11));
    }
    #[test_case]
    fn test_u8_get() {
        let mut t = Tester {
            data: 0b1111_0101_1011_1000_0001_0100_1100_0111,
            data2: 0b1010_0111,
        };
        assert!(t.get_data2(7));
        assert!(!t.get_data2(3));
    }
}
