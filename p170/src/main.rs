extern crate permutohedron;

use permutohedron::LexicalPermutation;

fn main() {
    // This number itself does not fit into a u32, but take one digit off and it can.
    let mut perms: &mut [u8] = &mut [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    //let mut perms: &mut [u8] = &mut [9, 7, 8, 6, 1, 0, 5, 2, 3, 4];

    loop {
        // Every way to split a 10 item array to two sub-arrays
        for split in 1..10 {
            // Each number will have at max 9 digits, which can fit into a u32
            let first_part: u32 = list_to_int(&perms[..split]);
            let second_part: u32 = list_to_int(&perms[split..]);

            let gcd = {
                // Courtesy of the `Mul` trait examples
                let mut x = first_part;
                let mut y = second_part;
                while y != 0 {
                    let t = y;
                    y = x % y;
                    x = t;
                }
                x
            };

            // For every common factor < 100...
            for factor in 2..(std::cmp::min(gcd, 1000) + 1) {
                if gcd % factor == 0 {
                    let first_number = first_part / factor;
                    let second_number = second_part / factor;

                    // One of each digit in `factor`, `first_number`, and `second_number`?
                    let mut bits = BitField(0);
                    bits.add_digits(factor);
                    bits.add_digits(first_number);
                    bits.add_digits(second_number);

                    if bits.0 == 0b0000_0011_1111_1111 {
                        println!("factor = {}", factor);
                        println!("first_number = {}", first_number);
                        println!("second_number = {}", second_number);
                        println!("perms = {:?}", perms);
                        std::process::exit(0);
                    }
                    //println!("ERROR");
                    //println!("state = {:#b}", bits.0);
                    //println!("factor = {}", factor);
                    //println!("first_number = {}", first_number);
                    //println!("first_part = {}", first_part);
                    //println!("second_number = {}", second_number);
                    //println!("second_part = {}", second_part);
                    //println!("perms = {:?}", perms);
                }
            }
        }

        //println!("{}{}{}{}--", perms[0], perms[1], perms[2], perms[3]);

        if !perms.prev_permutation() {
            break;
        }
    }

}

struct BitField(u16);

impl BitField {
    fn add_digits(&mut self, mut num: u32) {
        loop {
            let digit: usize = (num % 10) as usize;
            self.0 ^= 1 << digit;
            num /= 10;

            if num == 0 {
                return;
            }
        }
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bits_1() {
        let mut bits = BitField(0);
        bits.add_digits(1);
        assert_eq!(0b0000_0000_0000_0010, bits.0);
        bits.add_digits(1234);
        assert_eq!(0b0000_0000_0001_1100, bits.0);
        bits.add_digits(920);
        assert_eq!(0b0000_0010_0001_1001, bits.0);
        bits.add_digits(12345670);
        assert_eq!(0b0000_0010_1110_0110, bits.0);
    }
}
