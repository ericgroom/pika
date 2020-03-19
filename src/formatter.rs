pub enum FormatOption {
    Sponge,
    Usa
}

pub fn format(s: &str, option: FormatOption) -> String {
    let result = match option {
        FormatOption::Sponge => sponge(s),
        FormatOption::Usa => usa(s),
    };
    result
}

fn sponge(s: &str) -> String {
    enum Case { 
        Upper, Lower
    }
    impl Case {
        fn toggle(&mut self) {
            match self {
                Self::Upper => *self = Self::Lower,
                Self::Lower => *self = Self::Upper,
            }
        }
    }

    let mut result = String::with_capacity(s.len());
    let mut next_case = Case::Lower;
    for c in s.chars() {
        if c.is_alphabetic() {
            match next_case {
                Case::Upper => result.extend(c.to_uppercase()),
                Case::Lower => result.extend(c.to_lowercase())
            }
            next_case.toggle()
        } else {
            result.push(c);
        }
    }

    result
}

fn usa(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);

    for c in s.chars() {
        result.extend(c.to_uppercase());
        result.push_str("! ");
    }
    result.pop(); // removes final space

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sponge() {
        let input = "I bet you thought I wouldn't do it";
        let expected_output = "i BeT yOu ThOuGhT i WoUlDn'T dO iT";
        assert_eq!(format(input, FormatOption::Sponge), expected_output);
    }

    #[test]
    fn test_usa() {
        let input = "wellidid";
        let expected_output = "W! E! L! L! I! D! I! D!";
        assert_eq!(format(input, FormatOption::Usa), expected_output);
    }

    #[test]
    fn test_sponge_empty() {
        assert_eq!(format("", FormatOption::Sponge), "");
    }

    #[test]
    fn test_usa_empty() {
        assert_eq!(format("", FormatOption::Usa), "");
    }
}