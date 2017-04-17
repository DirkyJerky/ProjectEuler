struct DigitIterator10(u32);

impl Iterator for DigitIterator10 {
    type Item = u8;
    
    fn next(&mut self) -> Option<u8> {
        if self.0 == 0 {
            return None
        }

        let ret = self.0 % 10;
        self.0 /= 10;

        Some(ret as u8)
    }
    
    // Many provided methods can be overrode for greater efficiency.
}

fn main() {
    println!("Hello, world!");
}

// Will overflow if too many digits given, no warning.
fn list_to_int(xs: &[u8]) -> u32 {
    let mut ret: u32 = 0;
    for x in xs {
        ret = ret.wrapping_mul(10);
        ret = ret.wrapping_add(*x as u32);
    }
    
    ret
}

fn is_pandigital(a: u32, b: u32) -> bool {
    let digits = ((a as f32).log10().floor() + (b as f32).log10().floor()) as u8;

    if digits != 8 { // log10 subtracts one from digits count, want ten digits -> = 8
        return false;
    }

    let mut bit_field: u16 = 0;

    for i in DigitIterator10(a) {
        bit_field |= 1u16.wrapping_shl(i as u32)
    }

    for i in DigitIterator10(b) {
        bit_field |= 1u16.wrapping_shl(i as u32)
    }

    println!("bit_field is {:b}", bit_field);

    bit_field == 0b0000_0011_1111_1111u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pandigital_not_enough_digits() {
        assert!(!is_pandigital(1, 1));
    }

    #[test]
    fn pandigital_nan() {
        assert!(!is_pandigital(0, 0));
    }

    #[test]
    fn pandigital_good() {
        assert!(is_pandigital(1, 234567890));
        assert!(is_pandigital(12, 34567890));
        assert!(is_pandigital(123, 4567890));
        assert!(is_pandigital(1234, 567890));
        assert!(is_pandigital(12345, 67890));
        assert!(is_pandigital(123456, 7890));
        assert!(is_pandigital(1234567, 890));
        assert!(is_pandigital(12345678, 90));
    }

    #[test]
    fn list_to_int_test() {
        assert_eq!(list_to_int(&[1,2,3]), 123);
        assert_eq!(list_to_int(&[0]), 0);
        assert_eq!(list_to_int(&[]), 0);
        assert_eq!(list_to_int(&[0,0,0]), 0);
        assert_eq!(list_to_int(&[1,0,0]), 100);
        assert_eq!(list_to_int(&[0,0,1]), 1);
    }
}
