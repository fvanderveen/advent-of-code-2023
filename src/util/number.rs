// Allow dead_code since this is a util file copied across years. Later in the AoC we might use everything, or not.
#![allow(dead_code)]

use num_traits::Num;

macro_rules! parse_int_impl {
    ($($t:ty, $name: ident, $name_radix: ident)*) => {$(
        #[allow(unused)]
        pub fn $name(input: &str) -> Result<$t, String> {
            input.to_string().parse().map_err(|e| format!("{}", e))
        }

        #[allow(unused)]
        pub fn $name_radix(input: &str, radix: u32) -> Result<$t, String> {
            <$t>::from_str_radix(input, radix).map_err(|e| format!("{}", e))
        }
    )*}
}

parse_int_impl! {
    u8, parse_u8, parse_u8_radix
    u16, parse_u16, parse_u16_radix
    u32, parse_u32, parse_u32_radix
    u64, parse_u64, parse_u64_radix
    u128, parse_u128, parse_u128_radix
    usize, parse_usize, parse_usize_radix
    i8, parse_i8, parse_i8_radix
    i16, parse_i16, parse_i16_radix
    i32, parse_i32, parse_i32_radix
    i64, parse_i64, parse_i64_radix
    i128, parse_i128, parse_i128_radix
    isize, parse_isize, parse_isize_radix
}

pub fn parse_binary(binary: &str) -> usize {
    let mut result = 0;

    for char in binary.chars() {
        result <<= 1;
        match char {
            '1' => result += 1,
            '0' => {},
            _ => panic!("Invalid binary character: {}", char)
        }
    }

    result
}

pub fn lcm<T: Num + Copy>(left: T, right: T) -> T {
    let numerator = left * right;
    let denominator = gcd(left, right);

    numerator / denominator
}

pub fn gcd<T: Num + Copy>(a: T, b: T) -> T {
    if b == T::zero() {
        return a;
    }

    return gcd(b, a % b);
}

pub trait NumberExtensions<T> {
    fn lcm(&self) -> T;
    fn gcd(&self) -> T;
}
impl<T> NumberExtensions<T> for Vec<T> where T: Num + Copy + Clone {
    fn lcm(&self) -> T {
        if let Some((first, rest)) = self.split_first() {
            rest.iter().fold(first.clone(), |acc,v| lcm(acc, v.clone()))
        } else {
            T::zero()
        }
    }

    fn gcd(&self) -> T {
        if let Some((first, rest)) = self.split_first() {
            rest.iter().fold(first.clone(), |acc,v| gcd(acc, v.clone()))
        } else {
            T::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::number::{gcd, lcm, NumberExtensions, parse_binary};

    #[test]
    fn test_parse_binary() {
        assert_eq!(parse_binary("0101"), 5);
        assert_eq!(parse_binary("1111"), 15);
        assert_eq!(parse_binary("1000000"), 64);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(4, lcm(1, 4));
        assert_eq!(12, lcm(4, 6));
        assert_eq!(96, lcm(6, 32));

        assert_eq!(12, vec![4, 6, 3].lcm());
    }

    #[test]
    fn test_gcd() {
        assert_eq!(1, gcd(32, 5));
        assert_eq!(12, gcd(36, 12));
        assert_eq!(4, gcd(36, 32));

        assert_eq!(4, vec![36, 32, 48].gcd())
    }
}