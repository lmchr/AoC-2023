#[cfg(test)]
mod tests {
    use aoc_2023::util;
    use aoc_2023::util::Days;

    #[test]
    fn test_aoc_01() {
        let inputs = util::read_input(&Days::ONE);
        assert_eq!(aoc_2023::aoc_01::part1(&inputs), 54081);
        assert_eq!(aoc_2023::aoc_01::part2(&inputs), 54649);
    }

    #[test]
    fn test_aoc_02() {
        let inputs = util::read_input(&Days::TWO);
        assert_eq!(aoc_2023::aoc_02::part1(&inputs), 2285);
        assert_eq!(aoc_2023::aoc_02::part2(&inputs), 77021);
    }

    #[test]
    fn test_aoc_03() {
        assert!(aoc_2023::aoc_03::is_symbol_except_dot(&'*'));
        assert!(aoc_2023::aoc_03::is_symbol_except_dot(&'#'));
        assert!(!aoc_2023::aoc_03::is_symbol_except_dot(&'.'));
        assert!(!aoc_2023::aoc_03::is_symbol_except_dot(&'a'));
        assert!(!aoc_2023::aoc_03::is_symbol_except_dot(&'Z'));
        assert!(!aoc_2023::aoc_03::is_symbol_except_dot(&'4'));

        let inputs = util::read_input(&Days::THREE);
        assert_eq!(aoc_2023::aoc_03::part1(&inputs), 535235);
        assert_eq!(aoc_2023::aoc_03::part2(&inputs), 79844424);
    }

    #[test]
    fn test_aoc_04() {
        let inputs = util::read_input(&Days::FOUR);
        assert_eq!(aoc_2023::aoc_04::part1(&inputs), 25183);
        assert_eq!(aoc_2023::aoc_04::part2(&inputs), 5667240);
    }

}