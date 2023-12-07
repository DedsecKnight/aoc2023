use std::{cmp::Ordering, collections::HashMap};

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day7;

type HandType = u32;
const FIVE_KIND: HandType = 7;
const FOUR_KIND: HandType = 6;
const FULL_HOUSE: u32 = 5;
const THREE_KIND: u32 = 4;
const TWO_PAIR: u32 = 3;
const ONE_PAIR: u32 = 2;
const HIGH_CARD: u32 = 1;

impl Day7 {
    fn is_five_kind(card_freq: &HashMap<char, u32>) -> bool {
        card_freq.iter().any(|x| x.1 == &5)
    }
    fn is_four_kind(card_freq: &HashMap<char, u32>) -> bool {
        card_freq.iter().any(|x| x.1 == &4)
    }
    fn is_three_kind(card_freq: &HashMap<char, u32>) -> bool {
        card_freq.iter().any(|x| x.1 == &3)
    }
    fn is_one_pair(card_freq: &HashMap<char, u32>) -> bool {
        card_freq.iter().any(|x| x.1 == &2)
    }
    fn is_full_house(card_freq: &HashMap<char, u32>) -> bool {
        Day7::is_one_pair(card_freq) && Day7::is_three_kind(card_freq)
    }
    fn is_two_pair(card_freq: &HashMap<char, u32>) -> bool {
        card_freq.iter().filter(|x| x.1 == &2).count() == 2
    }
    fn calculate_card_freq(hand: &str) -> HashMap<char, u32> {
        let mut ret: HashMap<char, u32> = HashMap::new();
        for card in hand.chars() {
            ret.insert(card, (*ret.get(&card).as_ref().unwrap_or(&&0)) + 1);
        }
        ret
    }
    fn evaluate_hand(hand: &str) -> u32 {
        let card_freq = Self::calculate_card_freq(hand);
        if Day7::is_five_kind(&card_freq) {
            FIVE_KIND
        } else if Day7::is_four_kind(&card_freq) {
            FOUR_KIND
        } else if Day7::is_full_house(&card_freq) {
            FULL_HOUSE
        } else if Day7::is_three_kind(&card_freq) {
            THREE_KIND
        } else if Day7::is_two_pair(&card_freq) {
            TWO_PAIR
        } else if Day7::is_one_pair(&card_freq) {
            ONE_PAIR
        } else {
            HIGH_CARD
        }
    }
    fn find_best_hand(hand_data: &str) -> HandType {
        Vec::from(['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'])
            .iter()
            .fold(HIGH_CARD, |acc, curr| {
                acc.max(Self::evaluate_hand(
                    hand_data
                        .clone()
                        .chars()
                        .map(|x| if x == 'J' { *curr } else { x })
                        .collect::<String>()
                        .as_str(),
                ))
            })
    }
    fn parse_hand(hand_data: &str, j_as_joker: bool) -> (HandType, &str, u32) {
        let splitted_hand_data = hand_data.split(' ').collect::<Vec<_>>();
        (
            if j_as_joker {
                Day7::find_best_hand(splitted_hand_data[0])
            } else {
                Day7::evaluate_hand(splitted_hand_data[0])
            },
            splitted_hand_data[0],
            splitted_hand_data[1].parse::<u32>().unwrap(),
        )
    }
    fn compare_hand(lhs: &str, rhs: &str, card_order: &Vec<char>) -> Ordering {
        assert!(lhs.len() == rhs.len());
        let lhs_l = lhs.chars().collect::<Vec<_>>();
        let rhs_l = rhs.chars().collect::<Vec<_>>();
        for i in 0..lhs.len() {
            if lhs_l[i] != rhs_l[i] {
                let lhs_index = card_order.iter().position(|&x| x == lhs_l[i]).unwrap();
                let rhs_index = card_order.iter().position(|&x| x == rhs_l[i]).unwrap();
                return lhs_index.partial_cmp(&rhs_index).unwrap();
            }
        }
        Ordering::Equal
    }
}

impl Solution<u32, u32> for Day7 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut hands = lines
            .map(|line| Day7::parse_hand(line, false))
            .collect::<Vec<_>>();
        let card_order: Vec<char> = Vec::from([
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]);
        hands.sort_by(|a, b| {
            if a.0 != b.0 {
                a.0.partial_cmp(&b.0).unwrap()
            } else {
                Day7::compare_hand(a.1, b.1, &card_order)
            }
        });
        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, curr| acc + ((curr.0 + 1) as u32) * curr.1 .2)
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut hands = lines
            .map(|line| Day7::parse_hand(line, true))
            .collect::<Vec<_>>();
        let card_order: Vec<char> = Vec::from([
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ]);
        hands.sort_by(|a, b| {
            if a.0 != b.0 {
                a.0.partial_cmp(&b.0).unwrap()
            } else {
                Day7::compare_hand(a.1, b.1, &card_order)
            }
        });
        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, curr| acc + ((curr.0 + 1) as u32) * curr.1 .2)
    }
}
