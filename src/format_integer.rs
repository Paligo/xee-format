use ibig::IBig;
use num_traits::Signed;

pub fn format_integer(i: IBig, picture: &str) -> String {
    let mut width = picture.len();
    if i.is_negative() {
        width += 1;
    }
    format!("{:0width$}", i, width = width)
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
}
