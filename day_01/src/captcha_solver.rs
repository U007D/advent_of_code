use consts::*;
use Error;

type Result<T> = ::std::result::Result<T, Error>;
const BASE_10: u32 = 10;

#[derive(Clone, Default, Debug)]
pub struct CaptchaSolver {}

impl CaptchaSolver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn sum_when_matches_next(&self, digits: &str) -> Result<usize> {
        let mut cyclic_iter = digits.chars()
                         .map(|c|  c.to_digit(BASE_10).expect(MSG_ERR_NONE_ERROR))
                         .cycle()
                         .skip(1);
        Ok(digits.chars()
                 .map(|c| c.to_digit(BASE_10).expect(MSG_ERR_NONE_ERROR))
                 .fold(0_usize, |acc, d| {
                     let delta = match d == cyclic_iter.next().expect(MSG_ERR_NONE_ERROR) {
                         true => d,
                         false => 0,
                     };
                     #[allow(integer_arithmetic)]
                     acc + delta as usize
                 }))
    }
}
