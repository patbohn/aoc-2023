use super::{CommandImpl, DynError};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Parser, Debug)]
pub struct Day7a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day7a {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;
        let mut hands = Vec::new();
        for line in fs::read_to_string(&self.input).unwrap().lines() {
            hands.push(Hand::from_string(line));
        }
        hands.sort_by_key(|hand| hand.power);
        for (i, hand) in hands.iter().enumerate() {
            //dbg!(&hands);
            result += (i + 1) * hand.bid;
        }
        println!("Day7a: {result}");
        Ok(())
    }
}

#[derive(Debug, EnumCountMacro, EnumIter, Eq, Hash, PartialEq, Clone, Copy)]
enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardValue {
    fn from_char(c: char) -> Result<CardValue, String> {
        match c {
            'A' => Ok(CardValue::Ace),
            'K' => Ok(CardValue::King),
            'Q' => Ok(CardValue::Queen),
            'J' => Ok(CardValue::Jack),
            'T' => Ok(CardValue::Ten),
            '9' => Ok(CardValue::Nine),
            '8' => Ok(CardValue::Eight),
            '7' => Ok(CardValue::Seven),
            '6' => Ok(CardValue::Six),
            '5' => Ok(CardValue::Five),
            '4' => Ok(CardValue::Four),
            '3' => Ok(CardValue::Three),
            '2' => Ok(CardValue::Two),
            _ => Err("No matching card value found for char {c}".to_string()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<CardValue>,
    bid: usize,
    handtype: Option<HandType>,
    power: usize,
}

impl Hand {
    pub fn from_string(line: &str) -> Self {
        let mut split = line.split_ascii_whitespace();
        let cards: Vec<CardValue> =
            split.next().unwrap().chars().map(|x| CardValue::from_char(x).unwrap()).collect();
        let bid: usize = split.next().unwrap().parse().unwrap();
        Hand::new(cards, bid).unwrap()
    }

    pub fn new(cards: Vec<CardValue>, bid: usize) -> Result<Self, String> {
        let mut new_hand = Self { cards: cards, bid: bid, handtype: None, power: 0 };
        match new_hand.assign_handtype() {
            Ok(()) => match new_hand.calculate_power() {
                Ok(()) => return Ok(new_hand),
                _ => Err("Could not calculate power".to_string()),
            },
            _ => Err("Could not calculate handtype".to_string()),
        }
    }
    fn get_handtype(&self) -> Result<HandType, String> {
        let mut occurences_per_value: Vec<usize> = vec![0; CardValue::COUNT];
        //dbg!(&self.cards);
        for card in &self.cards {
            occurences_per_value[*card as usize] += 1;
        }
        //dbg!(&occurences_per_value);
        occurences_per_value.sort();

        //dbg!(&occurences_per_value);
        match occurences_per_value.pop() {
            Some(1) => return Ok(HandType::HighCard),
            Some(2) => match occurences_per_value.pop() {
                Some(2) => return Ok(HandType::TwoPair),
                Some(1) => return Ok(HandType::OnePair),
                _ | None => {
                    Err("Hand has a match of 2, but could not determine second match".to_string())
                }
            },
            Some(3) => match occurences_per_value.pop() {
                Some(2) => return Ok(HandType::FullHouse),
                Some(1) => return Ok(HandType::ThreeKind),
                _ | None => {
                    Err("Hand has a match of 3, but could not determine second match".to_string())
                }
            },
            Some(4) => return Ok(HandType::FourKind),
            Some(5) => return Ok(HandType::FiveKind),
            _ | None => Err("Card counting didn't work".to_string()),
        }
    }
    fn assign_handtype(&mut self) -> Result<(), String> {
        self.handtype = Some(self.get_handtype().expect("Could not assign handtype"));
        Ok(())
    }

    fn calculate_power(&mut self) -> Result<(), String> {
        let mut tmp_str: String =
            (self.handtype.expect("Could not read HandType") as usize).to_string();
        //dbg!(&tmp_str);
        for card in &self.cards {
            //dbg!(card);
            tmp_str.push_str(&format!("{:02}", (*card as usize)));
        }
        //dbg!(&tmp_str);
        assert_eq!(tmp_str.len(), 11);
        self.power = tmp_str.parse().expect("Could not convert power to usize");
        Ok(())
    }
}

// Tests follow here
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn card_comparisons() {
        let result = (CardValue::Ace as i32) > (CardValue::Nine as i32);
        assert_eq!(result, true);
    }

    #[test]
    fn hands_comparisons() {
        let result = (HandType::FourKind as i32) > (HandType::HighCard as i32);
        assert_eq!(result, true);
    }

    #[test]
    fn test_handtype() {
        let cards =
            vec![CardValue::Ace, CardValue::Ace, CardValue::Eight, CardValue::Ace, CardValue::Ace];
        let hand = Hand { cards: cards, bid: 0, handtype: None, power: 0 };
        let handtype = hand.get_handtype().expect("Could not determine hand type");

        assert_eq!(handtype, HandType::FourKind);

        let cards = vec![
            CardValue::Ace,
            CardValue::Three,
            CardValue::Eight,
            CardValue::Ace,
            CardValue::Ace,
        ];
        let hand = Hand { cards: cards, bid: 0, handtype: None, power: 0 };
        let handtype = hand.get_handtype().expect("Could not determine hand type");

        assert_eq!(handtype, HandType::ThreeKind);

        let cards = vec![
            CardValue::Ace,
            CardValue::Ace,
            CardValue::Eight,
            CardValue::Eight,
            CardValue::Ace,
        ];
        let hand = Hand { cards: cards, bid: 0, handtype: None, power: 0 };
        let handtype = hand.get_handtype().expect("Could not determine hand type");

        assert_eq!(handtype, HandType::FullHouse);

        let cards = vec![
            CardValue::Ace,
            CardValue::Three,
            CardValue::Eight,
            CardValue::Eight,
            CardValue::Ace,
        ];
        let hand = Hand { cards: cards, bid: 0, handtype: None, power: 0 };
        let handtype = hand.get_handtype().expect("Could not determine hand type");

        assert_eq!(handtype, HandType::TwoPair);
    }

    #[test]
    fn test_hands() {
        let cards =
            vec![CardValue::Ace, CardValue::Ace, CardValue::Eight, CardValue::Ace, CardValue::Ace];
        let hand = Hand::new(cards, 0).expect("Could not generate new hand");
        //let assignment = hand.assign_handtype().unwrap();
        //assert_eq!(assignment, ());
        assert_eq!(hand.handtype, Some(HandType::FourKind))
    }

    #[test]
    fn test_from_string() {
        let input = "32T3K 765".to_string();
        let hand = Hand::from_string(&input);
        let cards = vec![
            CardValue::Three,
            CardValue::Two,
            CardValue::Ten,
            CardValue::Three,
            CardValue::King,
        ];
        let bid = 765_usize;
        assert_eq!(hand.cards, cards);
        assert_eq!(hand.bid, bid);
        assert_eq!(hand.handtype, Some(HandType::OnePair));
    }
}
