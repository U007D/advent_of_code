use consts::*;
use Error;
use extension_traits::ToDigitOrError;

type Result<T> = ::std::result::Result<T, Error>;
const BASE_10: u32 = 10;

#[derive(Clone, Debug)]
pub enum MatchDistance {
    NextElement,
    HalfCircle,
}

#[derive(Clone, Debug)]
pub struct CaptchaSolver {
    match_distance: MatchDistance,
    digits_len: Option<usize>,
}

impl CaptchaSolver {
    pub fn new(match_distance: MatchDistance) -> Self {
        Self {
            match_distance,
            digits_len: None,
        }
    }

    pub fn sum_when_matches_next(&mut self, digits: &str) -> Result<usize> {
        self.digits_len = Some(digits.len());

        let mut cyclic_iter = digits.chars()
                                    .map(|c| c.to_digit_or_error(BASE_10))
                                    .cycle()
                                    .skip(self.match_distance());
        digits.chars()
              .map(|c| c.to_digit_or_error(BASE_10))
              .fold(Ok(0_usize), |acc, r| {
                  match r {
                      Ok(d) => {
                          let delta = match r == cyclic_iter.next().expect(MSG_ERR_NONE_ERROR) {
                              true => d,
                              false => 0,
                          };
                          #[allow(integer_arithmetic)]
                              Ok(acc.expect(MSG_ERR_NONE_ERROR) + delta as usize)
                      },
                      Err(e) => Err(e),
                  }
              })
    }

    fn match_distance(&self) -> usize {
        match self.match_distance {
            MatchDistance::NextElement => 1,
            MatchDistance::HalfCircle => self.digits_len.expect(MSG_ERR_INTERNAL_DIGITS_LEN_NOT_SET) / 2,
        }
    }
}
