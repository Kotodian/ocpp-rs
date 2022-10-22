use regex::Regex;
use validator::ValidationError;




pub fn validate_identifier_string(s: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"[a-z]|[A-Z]|[0-9]|[+]|[*]|[-]|[=]|[:]|[0]|[|]|[@]|[.]").unwrap();

    let res = re.is_match(s);

    match res {
        true => Ok(()),
        false => Err(ValidationError::new("Not a valid identifierString"))
    }
}