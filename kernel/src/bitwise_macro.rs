#[macro_export]
macro_rules! bit_setter {
    ($field:tt : $field_type:ty; $bit:literal, $vis:vis $name:ident) => {
        #[allow(dead_code)]
        $vis fn $name(&mut self, value: bool) {
            let b: $field_type = 1 << $bit;
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
    ($field:tt : $field_type:ty; $bit:literal, $vis:vis $name:ident) => {
        #[allow(dead_code)]
        $vis fn $name(&self) -> bool {
            let b: $field_type = 1 << $bit;
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
        bit_setter!(data: u32; 4, pub set_data_4);
        bit_setter!(data2: u8; 5, pub set_data2_5);
        bit_getter!(data: u32; 7, pub get_data_7);
        bit_getter!(data: u32; 11, pub get_data_11);
        bit_getter!(data2: u8; 7, pub get_data2_7);
        bit_getter!(data2: u8; 3, pub get_data2_3);
    }

    #[test_case]
    fn test_u32_set() {
        let mut t = Tester { data: 0, data2: 0 };
        t.set_data_4(true);
        assert_eq!(t.data, 0b10000);
    }
    #[test_case]
    fn test_u8_set() {
        let mut t = Tester { data: 0, data2: 0 };
        t.set_data2_5(true);
        assert_eq!(t.data2, 0b100000);
    }
    #[test_case]
    fn test_u32_get() {
        let mut t = Tester {
            data: 0b1111_0101_1011_1000_0001_0100_1100_0111,
            data2: 0b1010_0111,
        };
        assert!(t.get_data_7());
        assert!(!t.get_data_11());
    }
    #[test_case]
    fn test_u8_get() {
        let mut t = Tester {
            data: 0b1111_0101_1011_1000_0001_0100_1100_0111,
            data2: 0b1010_0111,
        };
        assert!(t.get_data2_7());
        assert!(!t.get_data2_3());
    }
}
