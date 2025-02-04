
//Execise 1
// Make it compile in unit test
// Run tests
// Hint: Convert Option to Result
fn generate_nametag_text(name: String) -> Result<String,String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        //None
        return Err("`name` was empty; it must be nonempty.".to_string());
    } else {
        let res = format!("Hi! My name is {}", name); 
        return Ok(res);
        //Some(format!("Hi! My name is {}", name))
    }
}
// Exercise 2
// Make it compile in unit test
// Run tests
// Hint: &str to integer conversion by using parse method and return Result
fn parse_number(s: &str) -> Result<i32, String> {
    let my_int:Result<i32, String> = match s.parse::<i32>() {
        Ok(n) => return  Ok(n),
        Err(_) => {
            return Err("invalid digit found in string".parse().unwrap());
        },
    };
}

// Exercise 3
// Make it compile in unit test
// Run tests
// Hint: Custom Error

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        }
        else if value < 0 {
            Err(CreationError::Negative)
        }
        else {
            Err(CreationError::Zero)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test for exercise 1
    #[test]
    fn exercise1_should_work() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );

        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }

    /// Test for exercise 2
    #[test]
    fn exercise2_should_work() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(
            parse_number("invalid"),
            Err("invalid digit found in string".parse().unwrap())
        );
    }

    /// Test for exercise 3
    #[test]
    fn exercise3_should_work() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}
