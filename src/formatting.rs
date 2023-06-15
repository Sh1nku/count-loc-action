use lazy_static::lazy_static;

pub fn abbreviate_format_number(number: usize) -> String {
    lazy_static! {
        // usize max value is 18_446_744_073_709_551_615 aka 18 quintillion
        static ref SUFFIXES: Vec<&'static str> = vec!["", "K", "M", "B", "T", "Qa", "Qi"];
    }
    let mut number = number as f32;
    let mut suffix_index = 0;
    while number >= 1000.0 {
        number /= 1000.0;
        suffix_index += 1;
    }
    if number < 100.0 && suffix_index != 0 {
        format!("{:.1}{}", number, SUFFIXES[suffix_index])
    } else {
        format!("{:.0}{}", number, SUFFIXES[suffix_index])
    }
}

pub fn pretty_format_number(number: usize) -> String {
    let mut number = number.to_string();
    let mut index = number.len();
    while index > 3 {
        index -= 3;
        number.insert(index, ' ');
    }
    number
}

#[cfg(test)]
mod tests {
    use crate::formatting::{abbreviate_format_number, pretty_format_number};

    #[test]
    fn test_abbreviate_format_number_1() {
        assert_eq!(abbreviate_format_number(1), "1");
    }

    #[test]
    fn test_abbreviate_format_number_268() {
        assert_eq!(abbreviate_format_number(268), "268");
    }

    #[test]
    fn test_abbreviate_format_number_1000() {
        assert_eq!(abbreviate_format_number(1000), "1.0K");
    }

    #[test]
    fn test_abbreviate_format_number_9800() {
        assert_eq!(abbreviate_format_number(9800), "9.8K");
    }

    #[test]
    fn test_abbreviate_format_number_54300() {
        assert_eq!(abbreviate_format_number(54_300), "54.3K");
    }

    #[test]
    fn test_abbreviate_format_number_54380() {
        assert_eq!(abbreviate_format_number(54_380), "54.4K");
    }

    #[test]
    fn test_abbreviate_format_number_100000() {
        assert_eq!(abbreviate_format_number(100_000), "100K");
    }

    #[test]
    fn test_abbreviate_format_number_985654() {
        assert_eq!(abbreviate_format_number(985_654), "986K");
    }

    #[test]
    fn test_abbreviate_format_number_1200000() {
        assert_eq!(abbreviate_format_number(1_200_000), "1.2M");
    }

    #[test]
    fn test_abbreviate_format_number_568200000() {
        assert_eq!(abbreviate_format_number(568_200_000), "568M");
    }

    #[test]
    fn test_abbreviate_format_number_1200200000() {
        assert_eq!(abbreviate_format_number(1_200_200_000), "1.2B");
    }

    #[test]
    fn test_abbreviate_format_number_usize_max() {
        assert_eq!(
            abbreviate_format_number(18_446_744_073_709_551_615),
            "18.4Qi"
        );
    }

    #[test]
    fn test_pretty_format_number_1() {
        assert_eq!(pretty_format_number(1), "1");
    }

    #[test]
    fn test_pretty_format_number_1000() {
        assert_eq!(pretty_format_number(1000), "1 000");
    }

    #[test]
    fn test_pretty_format_number_1000000() {
        assert_eq!(pretty_format_number(1_000_000), "1 000 000");
    }

    #[test]
    fn test_pretty_format_number_1000000000() {
        assert_eq!(pretty_format_number(1_000_000_000), "1 000 000 000");
    }
}
