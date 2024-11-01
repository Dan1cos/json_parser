use anyhow::Result;
use json_parser::*;
use pest::Parser;

pub fn main() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::number, "123.321")?;
    println!("{:?}", parsed_data.as_str());

    let parsed_data_2 = Grammar::parse(Rule::string, "\"string\"")?;
    println!("{:?}", parsed_data_2.as_str().trim_matches('"'));

    Ok(())
}
