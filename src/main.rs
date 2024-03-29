use polars::prelude::*;
use chrono::prelude::*;
use std::fs::File;
use std::path::PathBuf;

fn main() {

    let mut df: DataFrame = df!(
        "integer" => &[1, 2, 3],
        "date" => &[
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        ],
        "float" => &[4.0, 5.0, 6.0],
        "string" => &["a", "b", "c"],
    )
    .unwrap();
    println!("{}", df);

    let path: PathBuf = ["data", "output.csv"].iter().collect();

    let mut file = File::create(&path).expect("could not create file");

    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df).unwrap();
    
    let df_csv = CsvReader::from_path(&path).unwrap()
        .infer_schema(None)
        .has_header(true)
        .finish().unwrap();
    println!("{}", df_csv);

}