use ibig::IBig;
use num_traits::Signed;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Sign {
    OptionalDigit,
    MandatoryDigit,
    GroupSeparator(char),
}

#[derive(Debug, PartialEq)]
struct NonRegular(Vec<Sign>);
impl NonRegular {
    fn signs(&self) -> impl Iterator<Item = Sign> + '_ {
        self.0.iter().copied().rev()
    }

    fn mandatory_digit_max_count(&self) -> usize {
        self.0
            .iter()
            .filter(|s| matches!(s, Sign::MandatoryDigit))
            .count()
    }
}

#[derive(Debug, PartialEq)]
struct Regular {
    group_separator: char,
    count: usize,
    mandatory_digit_max_count: usize,
}

impl Regular {
    fn signs(&self) -> impl Iterator<Item = Sign> + '_ {
        RegularIterator::new(self.group_separator, self.count)
    }

    fn mandatory_digit_max_count(&self) -> usize {
        self.mandatory_digit_max_count
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPictureString,
}

#[derive(Debug, PartialEq)]
struct Picture {
    pattern: Pattern,
}

#[derive(Debug, PartialEq)]
enum Pattern {
    NonRegular(NonRegular),
    Regular(Regular),
}

struct RegularIterator {
    position: usize,
    group_separator: char,
    grouping_size: usize,
}

impl RegularIterator {
    fn new(group_separator: char, grouping_size: usize) -> Self {
        Self {
            position: 0,
            grouping_size,
            group_separator,
        }
    }
}

impl Iterator for RegularIterator {
    type Item = Sign;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.grouping_size {
            self.position += 1;
            Some(Sign::MandatoryDigit)
        } else {
            self.position = 0;
            Some(Sign::GroupSeparator(self.group_separator))
        }
    }
}

impl Pattern {
    fn new(signs: Vec<Sign>) -> Self {
        let regular = Self::create_regular(&signs);
        if let Some(regular) = regular {
            Self::Regular(regular)
        } else {
            Self::NonRegular(NonRegular(signs))
        }
    }

    fn create_regular(signs: &[Sign]) -> Option<Regular> {
        let mut last_separator = None;
        let mut last_count = None;
        let mut count = 0;
        let mut mandatory_digit_max_count = 0;

        for sign in signs.iter().rev() {
            match sign {
                Sign::GroupSeparator(c) => {
                    if let Some(last_separator) = last_separator {
                        if last_separator != *c {
                            return None;
                        }
                    } else {
                        last_separator = Some(*c);
                    }
                    if let Some(last_count) = last_count {
                        if count != last_count {
                            return None;
                        }
                    } else {
                        last_count = Some(count);
                    }
                    count = 0;
                }
                Sign::MandatoryDigit { .. } => {
                    mandatory_digit_max_count += 1;
                    count += 1;
                }
                Sign::OptionalDigit => {
                    count += 1;
                }
            }
        }

        last_separator.map(|last_separator| Regular {
            group_separator: last_separator,
            count: last_count.unwrap(),
            mandatory_digit_max_count,
        })
    }

    fn signs(&self) -> Box<dyn Iterator<Item = Sign> + '_> {
        match self {
            Self::NonRegular(p) => Box::new(p.signs()),
            Self::Regular(p) => Box::new(p.signs()),
        }
    }

    fn mandatory_digit_max_count(&self) -> usize {
        match self {
            Self::NonRegular(p) => p.mandatory_digit_max_count(),
            Self::Regular(p) => p.mandatory_digit_max_count(),
        }
    }
}

fn parse_decimal_digit_pattern(pattern: &str) -> Result<Vec<Sign>, Error> {
    pattern
        .chars()
        .map(|c| match c {
            '#' => Ok(Sign::OptionalDigit),
            '0'..='9' => Ok(Sign::MandatoryDigit),
            ',' | '.' => Ok(Sign::GroupSeparator(c)),
            _ => Err(Error::InvalidPictureString),
        })
        .collect()
}

fn validate_decimal_digit_pattern(pattern: &[Sign]) -> Result<(), Error> {
    let mut signs = pattern.iter().peekable();

    if matches!(signs.peek(), Some(Sign::GroupSeparator(_))) {
        return Err(Error::InvalidPictureString);
    }

    while let Some(sign) = signs.next() {
        match sign {
            Sign::OptionalDigit => {
                if !matches!(
                    signs.peek(),
                    Some(Sign::OptionalDigit) | Some(Sign::MandatoryDigit)
                ) {
                    return Err(Error::InvalidPictureString);
                }
            }
            Sign::GroupSeparator(_) => {
                if matches!(signs.peek(), Some(Sign::GroupSeparator(_)) | None) {
                    return Err(Error::InvalidPictureString);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

impl Picture {
    fn parse(picture: &str) -> Result<Self, Error> {
        let signs = parse_decimal_digit_pattern(picture)?;
        validate_decimal_digit_pattern(&signs)?;

        Ok(Self {
            pattern: Pattern::new(signs),
        })
    }

    fn format(&self, i: IBig) -> String {
        let is_negative = i.is_negative();
        let i = i.abs();

        let s = i.to_string();
        let mut digits = s.chars().rev().peekable();

        // we have an iterator for the pattern
        let mut output = String::new();
        let mandatory_digit_max_count = self.pattern.mandatory_digit_max_count();
        let mut mandatory_digit_count = 0;

        for sign in self.pattern.signs().peekable() {
            match sign {
                Sign::OptionalDigit => {
                    if let Some(digit) = digits.next() {
                        output.push(digit)
                    }
                }
                Sign::MandatoryDigit => {
                    if let Some(digit) = digits.next() {
                        output.push(digit)
                    } else {
                        if mandatory_digit_count >= mandatory_digit_max_count {
                            break;
                        }
                        output.push('0')
                    }
                    mandatory_digit_count += 1;
                }
                Sign::GroupSeparator(c) => {
                    if digits.peek().is_none() {
                        break;
                    }
                    output.push(c)
                }
            }
        }
        for digit in digits {
            output.push(digit)
        }
        if is_negative {
            output.push('-')
        }
        output.chars().rev().collect()
    }
}

pub fn format_integer(i: IBig, picture: &str) -> Result<String, Error> {
    let picture = Picture::parse(picture)?;
    Ok(picture.format(i))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_format_integer() {
        assert_eq!(format_integer(123.into(), "1").unwrap(), "123");
    }

    #[test]
    fn test_format_zero_padded_integer() {
        assert_eq!(format_integer(123.into(), "0000").unwrap(), "0123");
    }

    #[test]
    fn test_format_zero_padded_integer_negative() {
        assert_eq!(format_integer((-123).into(), "00000").unwrap(), "-00123");
    }

    #[test]
    fn test_format_zero_padded_integer_negative_shorter() {
        assert_eq!(format_integer((-123).into(), "0000").unwrap(), "-0123");
    }

    #[test]
    fn test_format_with_thousands_separator() {
        assert_eq!(format_integer(1234.into(), "0,000").unwrap(), "1,234");
    }

    #[test]
    fn test_format_with_thousands_separator2() {
        assert_eq!(format_integer(4321.into(), "0,000").unwrap(), "4,321");
    }

    #[test]
    fn test_format_with_thousands_separator_and_zero_prefix() {
        assert_eq!(format_integer(4321.into(), "00,000").unwrap(), "04,321");
    }

    #[test]
    fn test_illegal_primary_token() {
        assert_eq!(Picture::parse("0b0"), Err(Error::InvalidPictureString));
    }

    #[test]
    fn test_illegal_decimal_digit_pattern_with_adjacent_grouping_separators() {
        assert_eq!(Picture::parse("0,,0"), Err(Error::InvalidPictureString));
    }

    #[test]
    fn test_illegal_decimal_digit_pattern_with_starting_grouping_separator() {
        assert_eq!(Picture::parse(",0"), Err(Error::InvalidPictureString));
    }

    #[test]
    fn test_illegal_decimal_digit_pattern_with_ending_grouping_separator() {
        assert_eq!(Picture::parse("0,"), Err(Error::InvalidPictureString));
    }

    #[test]
    fn test_optional_digit_sign_before_mandatory_digit_sign() {
        assert!(Picture::parse("#0").is_ok());
    }

    #[test]
    fn test_optional_digit_by_itself_is_illegal() {
        assert_eq!(Picture::parse("#"), Err(Error::InvalidPictureString));
    }

    #[test]
    fn test_format_grouping_separator_with_irregular_separators() {
        assert_eq!(
            format_integer(1_222_333.into(), "1,222.000").unwrap(),
            "1,222.333"
        );
    }

    #[test]
    fn test_format_with_thousands_separator_large_regular() {
        assert_eq!(
            format_integer(1_222_333.into(), "0,000").unwrap(),
            "1,222,333"
        );
    }

    // #[test]
    // fn test_format_with_thousands_negative_regular() {
    //     assert_eq!(
    //         format_integer((-1_222_333).into(), "0,000").unwrap(),
    //         "-1,222,333"
    //     );
    // }

    // TODO validate that mandatory digits cannot come before optional digits

    // is this allowed? #,?

    // Next steps:

    // * Detect regular patterns and produce a Regular
    // * Stopping condition for the formatting loop: when we find a
    // GroupSeparator but there are no digits left, we should stop
}
