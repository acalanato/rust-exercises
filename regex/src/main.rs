fn _validate_pin(pin: &str) -> bool {
    let mut valid = 0;
    for x in pin.chars() {
	if x.is_ascii_digit() {
	    valid += 1;
	} else { continue }
    }
    if ((valid == 4) || (valid == 6)) && (pin.len().eq(&3_usize) || pin.len().eq(&5_usize)) {
	true
    } else { false }
    
}

fn validate_pin(pin: &str) -> bool {
    let mut valid = 0;
    for x in pin.chars() {
        if x.is_ascii_digit() {
            valid += 1;
        } else {
            return false;
        }
    }
    if (valid == 4) || (valid == 6) {
        true
    } else { 
        false 
    }
}

fn main() {
    //validate_pin("1234567");
    println!("{}", validate_pin("-1234"));
    
    println!("Sucess!");
}

#[cfg(test)]
mod tests {
    use super::validate_pin;
    
    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }
    
    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }
    
    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }
}
