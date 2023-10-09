use ibig::IBig;
use num_traits::Signed;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPictureString,
}

#[derive(Debug, PartialEq)]
struct Picture {
    pattern: Vec<Sign>,
}

#[derive(Debug, PartialEq)]
enum Sign {
    OptionalDigit,
    MandatoryDigit,
    GroupSeparator(char),
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
        let pattern = parse_decimal_digit_pattern(picture)?;
        validate_decimal_digit_pattern(&pattern)?;

        Ok(Self { pattern })
    }

    fn format(&self, i: IBig) -> String {
        let is_negative = i.is_negative();
        let i = i.abs();

        let s = i.to_string();
        let mut digits = s.chars().rev();

        // we have an iterator for the pattern
        let signs = self.pattern.iter();
        let mut output = String::new();
        for sign in signs.rev() {
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
                        output.push('0')
                    }
                }
                Sign::GroupSeparator(c) => output.push(*c),
            }
        }
        for digit in digits {
            output.push(digit)
        }
        if is_negative {
            output.push('-')
        }
        output.chars().rev().collect()

        // if thousands_separator {
        //     let s = i.to_string();
        //     s.chars()
        //         .rev()
        //         .enumerate()
        //         .fold(String::new(), |mut acc, (i, c)| {
        //             if i % 3 == 0 && i != 0 {
        //                 acc.push(',')
        //             }
        //             acc.push(c);
        //             acc
        //         })
        //         .chars()
        //         .rev()
        //         .collect()
        // } else {
        //     format!("{:0width$}", i, width = width)
        // }
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

    // #[test]
    // fn test_format_with_thousands_separator_large() {
    //     assert_eq!(
    //         format_integer(1_222_333.into(), "0,000").unwrap(),
    //         "1,222,333"
    //     );
    // }

    #[test]
    fn test_format_with_thousands_separator_and_zero_prefix() {
        assert_eq!(format_integer(4321.into(), "00,000").unwrap(), "04,321");
    }

    // #[test]
    // fn test_format_with_thousands_negative() {
    //     assert_eq!(
    //         format_integer((-1_222_333).into(), "0,000").unwrap(),
    //         "-1,222,333"
    //     );
    // }

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

    // #[test]
    // fn test_irregular_grouping_separator_with_mixed_separators() {
    //     assert_eq!(
    //         format_integer(1_222_333.into(), "1,222.000").unwrap(),
    //         "1,222.333"
    //     );
    // }

    // TODO validate that mandatory digits cannot come before optional digits

    // is this allowed? #,?
}
