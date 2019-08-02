use iter_try::*;

#[test]
fn test_try_find_api_usability() -> Result<(), Box<dyn std::error::Error>> {
    let a = ["1", "2"];

    let is_my_num = |s: &str, search: i32| -> Result<bool, std::num::ParseIntError> {
        Ok(s.parse::<i32>()? == search)
    };

    let val = a.iter().try_find(|&&s| is_my_num(s, 2))?;
    assert_eq!(val, Some(&"2"));

    Ok(())
}
