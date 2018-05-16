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
        Ok(digits.chars()
                 .fold(Ok((0, None)), |acc: Result<(usize, Option<usize>)>, c: char| {
                    acc.and_then(|acc_inner| {
                        let (sum, prev_d) = acc_inner;
                        let curr_d = c.to_digit(BASE_10).ok_or_else(|| Error::InvalidDigit(c))? as usize;
                        let delta = match Some(curr_d) == prev_d {
                            true => curr_d,
                            false => 0,
                        };
                        #[allow(integer_arithmetic)]
                        Ok((sum + delta, Some(curr_d)))
                    })
                 })?
                 .0)
    }
}
