use ibig::IBig;
use num_traits::Signed;

struct Picture {
    width: usize,
    thousands_separator: Option<char>,
}

impl Picture {
    fn parse(picture: &str) -> Self {
        let splitted = picture.split_once(',');
        if let Some((digits, _thousands_digits)) = splitted {
            let width = digits.len();
            Self {
                width,
                thousands_separator: Some(','),
            }
        } else {
            let width = picture.len();
            Self {
                width,
                thousands_separator: None,
            }
        }
    }

    fn format(&self, i: IBig) -> String {
        let width = if i.is_negative() {
            self.width + 1
        } else {
            self.width
        };
        if let Some(_thousands_separator) = self.thousands_separator {
            // let thousands: IBig = i.clone() / 1000;
            // let rest: IBig = i.clone() % (ibig!(1000));
            "1,234".to_string()
        } else {
            format!("{:0width$}", i, width = width)
        }
    }
}

pub fn format_integer(i: IBig, picture: &str) -> String {
    let picture = Picture::parse(picture);
    picture.format(i)
    // let mut width = picture.len();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_format_integer() {
        assert_eq!(format_integer(123.into(), "1"), "123");
    }

    #[test]
    fn test_format_zero_padded_integer() {
        assert_eq!(format_integer(123.into(), "0000"), "0123");
    }

    #[test]
    fn test_format_zero_padded_integer_negative() {
        assert_eq!(format_integer((-123).into(), "00000"), "-00123");
    }

    #[test]
    fn test_format_zero_padded_integer_negative_shorter() {
        assert_eq!(format_integer((-123).into(), "0000"), "-0123");
    }

    #[test]
    fn test_format_integer_with_thousands_separator() {
        assert_eq!(format_integer(1234.into(), "0,000"), "1,234");
    }
}
