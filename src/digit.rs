use icu::properties::{sets::CodePointSetData, GeneralCategory};

fn is_digit(c: char) -> bool {
    let category = icu::properties::maps::general_category().get(c);
    category == GeneralCategory::DecimalNumber
}

// a digit family is indicated by the '0' character in that
// family
fn digit_family(c: char) -> Option<char> {
    let gc = icu::properties::maps::general_category();
    for r in gc.iter_ranges_for_value(GeneralCategory::DecimalNumber) {
        let c = c as u32;
        if r.contains(&c) {
            let index = c - r.start();
            return char::from_u32(index / 10 + r.start());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_digit() {
        assert!(is_digit('1'));
        assert!(!is_digit('a'));
        assert!(is_digit('١'));
    }

    #[test]
    fn test_digit_family() {
        assert_eq!(digit_family('1'), Some('0'));
        assert_eq!(digit_family('١'), Some('٠'));
        assert_eq!(digit_family('a'), None);
    }
}
