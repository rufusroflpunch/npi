use rand::Rng;

fn rand_digit(zero: bool) -> u32 {
    let mut rng = rand::thread_rng();
    let start = if zero { 0 } else { 1 };
    rng.gen_range(start..10)
}

fn nine_digits() -> [u32; 9] {
    [
        rand_digit(false),
        rand_digit(true),
        rand_digit(true),
        rand_digit(true),
        rand_digit(true),
        rand_digit(true),
        rand_digit(true),
        rand_digit(true),
        rand_digit(true),
    ]
}

fn round_nearest_ten(num: u32) -> u32 {
    let mut n = num;
    while n % 10 != 0 {
        n += 1;
    }
    n
}

fn mod_digits(digits: [u32; 9]) -> [u32; 9] {
    [
        digits[0] * 2,
        digits[1],
        digits[2] * 2,
        digits[3],
        digits[4] * 2,
        digits[5],
        digits[6] * 2,
        digits[7],
        digits[8] * 2,
    ]
}

fn sum_modded_digits(digits: [u32; 9]) -> u32 {
    let mut sum = 24;
    for digit in digits {
        sum += if digit < 10 { digit } else { 1 + digit % 10 }
    }
    sum
}

fn check_digit(num: u32) -> u32 {
    round_nearest_ten(num) - num
}

fn stringify(digits: [u32; 9], check_digit: u32) -> String {
    let mut s = String::with_capacity(10);
    for digit in digits {
        s.push(char::from_digit(digit, 10).unwrap());
    }
    s.push(char::from_digit(check_digit, 10).unwrap());
    s
}

pub fn npi() -> String {
    let d = nine_digits();
    let modded_digits = mod_digits(d);
    let sum = sum_modded_digits(modded_digits);
    let cd = check_digit(sum);

    stringify(d, cd)
}

#[cfg(test)]
mod tests {
    use crate::{
        check_digit, mod_digits, nine_digits, rand_digit, round_nearest_ten, stringify,
        sum_modded_digits,
    };

    #[test]
    fn test_round_nearest_ten() {
        assert_eq!(round_nearest_ten(57), 60);
        assert_eq!(round_nearest_ten(60), 60);
    }

    #[test]
    fn test_rand_digit() {
        for _ in 0..1000 {
            assert!(rand_digit(false) > 0);
        }
    }

    #[test]
    fn test_nine_digits() {
        let n = nine_digits();
        assert!(n[0] > 0);
        for n in n {
            assert!(n < 10);
        }
    }

    #[test]
    fn test_mod_digits() {
        let digits = [1, 1, 3, 4, 2, 9, 6, 0, 2];
        assert_eq!(mod_digits(digits), [2, 1, 6, 4, 4, 9, 12, 0, 4]);
    }

    #[test]
    fn test_sum_modded_digits() {
        let digits = [2, 1, 6, 4, 4, 9, 12, 0, 4];
        assert_eq!(sum_modded_digits(digits), 57);
    }

    #[test]
    fn test_check_digit() {
        assert_eq!(check_digit(57), 3);
    }

    #[test]
    fn test_stringify() {
        assert_eq!(stringify([1, 1, 3, 4, 2, 9, 6, 0, 2], 3), "1134296023");
    }
}
