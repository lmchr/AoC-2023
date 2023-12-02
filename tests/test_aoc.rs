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
}