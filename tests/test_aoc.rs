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
    #[test]
    fn test_aoc_05() {
        let inputs = util::read_input(&Days::FIVE);
        assert_eq!(aoc_2023::aoc_05::part1(&inputs), 484023871);
        // assert_eq!(aoc_2023::aoc_05::part2(&inputs), 46294175);
    }

    #[test]
    fn test_aoc_06() {
        let inputs = util::read_input(&Days::SIX);
        assert_eq!(aoc_2023::aoc_06::part1(&inputs), 1159152);
        assert_eq!(aoc_2023::aoc_06::part2(&inputs), 41513103);
    }

    #[test]
    fn test_aoc_07() {
        let inputs = util::read_input(&Days::SEVEN);
        assert_eq!(aoc_2023::aoc_07::part1(&inputs), 246409899);
        assert_eq!(aoc_2023::aoc_07::part2(&inputs), 244848487);
    }

    #[test]
    fn test_aoc_08() {
        let inputs = util::read_input(&Days::EIGHT);
        assert_eq!(aoc_2023::aoc_08::part1(&inputs), 19783);
        assert_eq!(aoc_2023::aoc_08::part2(&inputs), 9177460370549);
    }

    #[test]
    fn test_aoc_09() {
        let inputs = util::read_input(&Days::NINE);
        assert_eq!(aoc_2023::aoc_09::part1(&inputs), 1953784198);
        assert_eq!(aoc_2023::aoc_09::part2(&inputs), 957);
    }

    #[test]
    fn test_aoc_10() {
        let inputs = util::read_input(&Days::TEN);
        assert_eq!(aoc_2023::aoc_10::part1(&inputs), 6864);
        assert_eq!(aoc_2023::aoc_10::part2(&inputs), 0);
    }

    #[test]
    fn test_aoc_11() {
        let inputs = util::read_input(&Days::ELEVEN);
        assert_eq!(aoc_2023::aoc_11::part1(&inputs), 9769724);
        assert_eq!(aoc_2023::aoc_11::part2(&inputs), 603020563700);
    }

    #[test]
    fn test_aoc_13() {
        let inputs = util::read_input(&Days::THIRTEEN);
        assert_eq!(aoc_2023::aoc_13::part1(&inputs), 39939);
        assert_eq!(aoc_2023::aoc_13::part2(&inputs), 32069);
    }

    #[test]
    fn test_aoc_14() {
        let inputs = util::read_input(&Days::FOURTEEN);
        assert_eq!(aoc_2023::aoc_14::part1(&inputs), 108935);
        //assert_eq!(aoc_2023::aoc_14::part2(&inputs), 0);
    }

    #[test]
    fn test_aoc_15() {
        let inputs = util::read_input(&Days::FIFTEEN);
        assert_eq!(aoc_2023::aoc_15::part1(&inputs), 503154);
        assert_eq!(aoc_2023::aoc_15::part2(&inputs), 251353);
    }

}