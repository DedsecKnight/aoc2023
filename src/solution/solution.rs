pub trait Solution<PartOneReturnType, PartTwoReturnType> {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> PartOneReturnType;
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> PartTwoReturnType;
}
