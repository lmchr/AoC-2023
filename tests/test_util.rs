#[cfg(test)]
mod tests {
    use aoc_2023::util;
    use strum::IntoEnumIterator;

    #[test]
    fn test_read_input() {
        let days = util::Days::iter();
        for day in days {
            let out = util::read_input(&day);
            assert!(!out.is_empty())
        }
    }
}