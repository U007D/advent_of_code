use spreadsheet::{CellType, Spreadsheet};
use super::{given, run,
            Error, Result};

#[derive(Clone, Debug)]
struct Env {
    spreadsheet: Result<Spreadsheet>,
    result: Option<CellType>,
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
        let expected_result = Err::<Spreadsheet, Error>(Error::NoImportData);

        ctx.then("the result should be failure to instantiate due to no import data", move |env| {
            assert!(env.spreadsheet.clone().unwrap_err() == expected_result.clone().unwrap_err());
        });
    }));

    run(&given("a spreadsheet with invalid data", Env::new("hello world"), |ctx| {
        let expected_result = Err::<Spreadsheet, Error>(Error::InvalidInt("x".parse::<CellType>().unwrap_err()));

        ctx.then("the result should be failure to instantiate due to invalid integer", move |env| {
            assert!(env.spreadsheet.clone().unwrap_err() == expected_result.clone().unwrap_err());
        });
    }));

    run(&given("a spreadsheet with test data set 1", Env::new("5 1 9 5\n7 5 3\n2 4 6 8"), |ctx| {
        ctx.when("a checksum is calculated", |ctx| {
            ctx.before(|env| {
                // Create Error::InvalidInt(ParseIntError(IntErrorKind::Empty)) error
                env.result = Some(env.spreadsheet.clone().unwrap().checksum());
            });
            let expected_result = Some(18);

            ctx.then("the result should be 18", move |env| {
                assert!(env.result.unwrap() == expected_result.unwrap());
            });
        });
    }));
}

