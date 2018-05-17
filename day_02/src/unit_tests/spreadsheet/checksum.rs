use spreadsheet::{CellType, Spreadsheet};
use super::{given, run,
            Error, Result};

#[derive(Clone, Debug)]
struct Env {
    spreadsheet: Result<Spreadsheet>,
    result: Option<Result<CellType>>,
}

impl Env {
    fn new(data: &str) -> Self {
        Self {
            spreadsheet: Spreadsheet::new(data),
            result: None,
        }
    }
}
#[test]
fn tests() {
    run(&given("a spreadsheet with no data", Env::new(""), |ctx| {
        let expected_result = Err::<Spreadsheet, Error>(Error::InvalidInt("".parse::<CellType>().unwrap_err()));

        ctx.then("the result should be failure to instantiate", move |env| {
            assert!(env.spreadsheet.clone().unwrap_err() == expected_result.clone().unwrap_err());
        });

/*        ctx.when("a checksum is calculated", |ctx| {
            ctx.before(|env| {
                // Create Error::InvalidInt(ParseIntError(IntErrorKind::Empty)) error
                env.constructor_result = env.spreadshee;
            });
            let expected_result = Some(Err(Error::InvalidInt("".parse().unwrap_err())));

            ctx.then("the result should be failure to instantiate", move |env| {
                assert!(env.constructor_result.unwrap().unwrap_err() == expected_result.unwrap().unwrap_err());
            });
        });
*/    }));
}

