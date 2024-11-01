use anyhow::Result;
use json_parser::*;
use pest::Parser;

#[test]
fn number_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::number, "12331.321")?;
    assert_eq!(parsed_data.as_str(), "12331.321");

    Ok(())
}

#[test]
fn string_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::string, "\"Hello\"")?;
    assert_eq!(parsed_data.as_str().trim_matches('"'), "Hello");

    Ok(())
}

#[test]
#[should_panic]
fn error_test() {
    assert!(!Grammar::parse(Rule::number, "qwe.-rty").is_err())
}
