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
    todo!()
}

fn usa(s: &str) -> String {
    todo!()
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
        assert_eq!(format(input, FormatOption::Sponge), expected_output);
    }
}