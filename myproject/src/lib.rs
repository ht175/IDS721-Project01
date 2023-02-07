use polars::prelude::*;
use std::error::Error;
use std::fs::File;

pub fn read_header(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut rdr = csv::Reader::from_reader(file);
    {
        // We nest this call in its own scope because of lifetimes.
        let headers = rdr.headers()?;
        println!("{:?}", headers);
    }
    Ok(())
}

pub fn read_csv(file_name: &str) -> DataFrame {
    CsvReader::from_path(file_name).unwrap().finish().unwrap()
}

pub fn groupby_sum(df: &DataFrame) -> PolarsResult<DataFrame> {
    df.groupby(["Genre"])?
        .select(["Audience score %", "Profitability", "Worldwide Gross"])
        .mean()
}
