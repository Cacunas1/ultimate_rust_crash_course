use unicode_segmentation::UnicodeSegmentation;
use std::convert::TryFrom;

pub fn is_armstrong_number(num: u32) -> bool {
    let str_num : String = num.to_string();
    let n_digits = str_num.len();
    let mut ans: u128 = 0;
    for digit in str_num.graphemes(true).collect::<Vec<&str>>() {
        let num_digit: u128 = digit.trim().parse().expect("Número no válido");
        // ans += num_digit.pow(n_digits as u32);
        ans += num_digit.pow(u32::try_from(n_digits).expect("Número no válido"));
    }
    ans == num.into()
}
