use icu::properties::{sets::CodePointSetData, GeneralCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct AsciiDigit(char);

impl AsciiDigit {
    pub(crate) fn new(c: char) -> Self {
        debug_assert!(c.is_ascii_digit());
        AsciiDigit(c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DigitFamily(char);

impl DigitFamily {
    pub(crate) fn new(c: char) -> Option<Self> {
        let gc = icu::properties::maps::general_category();
        // decimal digits can consist in multiple unicode ranges (the ascii digits
        // versus the other ones.
        for r in gc.iter_ranges_for_value(GeneralCategory::DecimalNumber) {
            let c = c as u32;
            // if the character is in the range, we first subtract the start of the
            // range so we can do an integer division by 10, and then add the start
            // back again. This will get us the 0 digit in that range.
            if r.contains(&c) {
                let index = c - r.start();
                // we don't expect from_u32 to ever return None, but since this
                // function is fallible anyway we can just return None and avoid
                // an unwrap.
                return char::from_u32(index / 10 + r.start()).map(DigitFamily);
            }
        }
        None
    }

    pub(crate) fn digit(&self, d: AsciiDigit) -> char {
        let num = d.0 as u32 - '0' as u32;
        char::from_u32(num + self.0 as u32).unwrap()
    }
}

pub(crate) fn is_group_separator(c: char) -> bool {
    let category = icu::properties::maps::general_category().get(c);
    //  Nd, Nl, No, Lu, Ll, Lt, Lm or Lo are not allowed to be group separators
    !matches!(
        category,
        GeneralCategory::DecimalNumber
            | GeneralCategory::LetterNumber
            | GeneralCategory::OtherNumber
            | GeneralCategory::UppercaseLetter
            | GeneralCategory::LowercaseLetter
            | GeneralCategory::TitlecaseLetter
            | GeneralCategory::ModifierLetter
            | GeneralCategory::OtherLetter
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARAB_INDIC_DIGIT_ZERO: char = '٠';
    const ARAB_INDIC_DIGIT_ONE: char = '١';
    const NKO_DIGIT_ZERO: char = '߀';
    const NKO_DIGIT_FIVE: char = '߅';

    #[test]
    fn test_digit_family() {
        assert_eq!(DigitFamily::new('1'), Some(DigitFamily('0')));
        assert_eq!(
            DigitFamily::new(ARAB_INDIC_DIGIT_ONE),
            Some(DigitFamily(ARAB_INDIC_DIGIT_ZERO))
        );
        assert_eq!(
            DigitFamily::new(NKO_DIGIT_FIVE),
            Some(DigitFamily(NKO_DIGIT_ZERO))
        );
        assert_eq!(DigitFamily::new('a'), None);
    }

    #[test]
    fn test_ascii_digit_into_digit_family() {
        assert_eq!(
            DigitFamily::new(ARAB_INDIC_DIGIT_ZERO)
                .unwrap()
                .digit(AsciiDigit::new('1')),
            ARAB_INDIC_DIGIT_ONE
        );
        assert_eq!(
            DigitFamily::new(NKO_DIGIT_ZERO)
                .unwrap()
                .digit(AsciiDigit::new('5')),
            NKO_DIGIT_FIVE
        );
    }

    #[test]
    fn test_is_group_separator() {
        assert!(is_group_separator('!'));
        assert!(is_group_separator(','));
        assert!(!is_group_separator('Ⅰ'));
        assert!(!is_group_separator('1'));
        assert!(!is_group_separator('x'))
    }
}
