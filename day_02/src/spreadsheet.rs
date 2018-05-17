use super::Result;
use ::std::{num::ParseIntError,
            result::Result as StdResult};

pub type CellType = i32;

#[derive(Clone, Default, Debug)]
pub struct Spreadsheet {
    sheet: Vec<Vec<CellType>>,
}

impl Spreadsheet {
    pub fn new(data: &str) -> Result<Self> {
        Ok(Self {
            sheet: Self::make_sheet(data)?,
        })
    }

    pub fn checksum(&self) -> CellType {
        0
    }

    fn make_sheet(data: &str) -> Result<Vec<Vec<CellType>>> {
        let rows = data.split('\n')
                       .collect::<Vec<_>>();
        Ok(rows.iter()
            .map(|s| s.split('\t')
                      .map(|v| v.parse::<CellType>())   // Result<elements>
                      .collect::<StdResult<Vec<_>, ParseIntError>>()) // Result<rows>
            .collect::<StdResult<Vec<_>, ParseIntError>>()?) // Result<sheet>
    }
}
