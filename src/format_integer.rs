use ibig::IBig;
use num_traits::Signed;

#[derive(Debug, PartialEq)]
pub enum Error {
    IllegalPrimaryToken,
}

struct Picture {
    width: usize,
    thousands_separator: Option<char>,
}

enum Sign {
    OptionalDigit,
    MandatoryDigit,
    GroupSeparator,
}

fn parse_decimal_digit_pattern(pattern: &str) -> Result<Vec<Sign>, Error> {
    pattern
        .chars()
        .map(|c| match c {
            '#' => Ok(Sign::OptionalDigit),
            '0'..='9' => Ok(Sign::MandatoryDigit),
            ',' => Ok(Sign::GroupSeparator),
            _ => Err(Error::IllegalPrimaryToken),
        })
        .collect()
}

fn validate_decimal_digit_pattern(pattern: Vec<Sign>) -> Result<(), Error> {
    let mut signs = pattern.iter().peekable();

    if matches!(signs.peek(), Some(Sign::GroupSeparator)) {
        return Err(Error::IllegalPrimaryToken);
    }

    while let Some(sign) = signs.next() {
        if matches!(sign, Sign::GroupSeparator)
            && matches!(signs.peek(), Some(Sign::GroupSeparator))
        {
            return Err(Error::IllegalPrimaryToken);
        }
    }
    Ok(())
}

impl Picture {
    fn parse(picture: &str) -> Result<Self, Error> {
        let pattern = parse_decimal_digit_pattern(picture)?;
        validate_decimal_digit_pattern(pattern)?;

        let splitted = picture.split_once(',');
        if let Some((digits, _thousands_digits)) = splitted {
            let width = digits.len();
            Ok(Self {
                width,
                thousands_separator: Some(','),
            })
        } else {
            let width = picture.len();
            Ok(Self {
                width,
                thousands_separator: None,
            })
        }
    }

    fn format(&self, i: IBig) -> String {
        let width = if i.is_negative() {
            self.width + 1
        } else {
            self.width
        };

        if let Some(_thousands_separator) = self.thousands_separator {
            let s = i.to_string();
            s.chars()
                .rev()
                .enumerate()
                .fold(String::new(), |mut acc, (i, c)| {
                    if i % 3 == 0 && i != 0 {
                        acc.push(',')
                    }
                    acc.push(c);
                    acc
                })
                .chars()
                .rev()
                .collect()
        } else {
            format!("{:0width$}", i, width = width)
        }
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
    fn test_format_with_thousands_separator_large() {
        assert_eq!(
            format_integer(1_222_333.into(), "0,000").unwrap(),
            "1,222,333"
        );
    }

    #[test]
    fn test_format_with_thousands_negative() {
        assert_eq!(
            format_integer((-1_222_333).into(), "0,000").unwrap(),
            "-1,222,333"
        );
    }

    #[test]
    fn test_illegal_primary_token() {
        assert_eq!(
            format_integer(123.into(), "0b0"),
            Err(Error::IllegalPrimaryToken)
        );
    }

    #[test]
    fn test_illegal_decimal_digit_pattern_with_adjacent_grouping_separators() {
        assert_eq!(
            format_integer(234.into(), "0,,0"),
            Err(Error::IllegalPrimaryToken)
        );
    }

    #[test]
    fn test_illegal_decimal_digit_pattern_with_starting_grouping_separator() {
        assert_eq!(
            format_integer(234.into(), ",0"),
            Err(Error::IllegalPrimaryToken)
        );
    }

    // #[test]
    // fn test_illegal_decimal_digit_pattern_with_ending_grouping_separator() {
    //     assert_eq!(
    //         format_integer(345.into(), "0,"),
    //         Err(Error::IllegalPrimaryToken)
    //     );
    // }
}
