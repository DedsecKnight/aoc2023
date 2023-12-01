pub trait Solution<DayOneReturnType, DayTwoReturnType> {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> DayOneReturnType;
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> DayTwoReturnType;
}
