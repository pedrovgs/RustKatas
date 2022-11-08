fn fizz_buzz(value: i32) -> String {
    let is_mod_of_3 = value % 3 == 0;
    let is_mod_of_5 = value % 5 == 0;
    if is_mod_of_3 && is_mod_of_5 {
        "FizzBuzz".to_string()
    } else if is_mod_of_3 {
        "Fizz".to_string()
    } else if is_mod_of_5 {
        "Buzz".to_string()
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::fizz_buzz;

    #[test]
    fn returns_fizz_if_value_is_3() {
        assert_eq!("Fizz", fizz_buzz(3));
    }

    #[test]
    fn returns_fizz_if_value_is_multiple_of_3_but_not_multiple_of_5() {
        assert_eq!("Fizz", fizz_buzz(9));
    }

    #[test]
    fn returns_buzz_if_value_is_5() {
        assert_eq!("Buzz", fizz_buzz(20));
    }

    #[test]
    fn returns_buzz_if_value_is_multiple_of_5_but_not_multiple_of_3() {
        assert_eq!("Buzz", fizz_buzz(20));
    }

    #[test]
    fn returns_fizz_buzz_if_value_is_multiple_of_5_and_3() {
        assert_eq!("FizzBuzz", fizz_buzz(15));
    }

    #[test]
    fn returns_the_number_as_string_if_is_not_multiple_of_3_or_5() {
        assert_eq!("7", fizz_buzz(7));
    }
}
