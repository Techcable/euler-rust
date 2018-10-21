use std::ops::{Add, BitOr, BitAnd, BitOrAssign};
use std::cmp::Ordering;
use std::fmt::{self, Formatter, Display, Write};

use failure::Error;

use super::EulerProblem;

const POKER_HANDS_TEXT: &str = include_str!("poker.txt");

pub fn solve() -> Result<i32, Error> {
    let mut hands = Vec::new();
    for line in POKER_HANDS_TEXT.lines() {
        let mut cards = Vec::with_capacity(10);
        for card in line.split_whitespace() {
            cards.push(PokerCard::parse(card)?);
        }
        ensure!(cards.len() == 10, "Expected 10 cards: {:?}", line);
        hands.push((PokerHand::new(&cards[..5]), PokerHand::new(&cards[5..])));
    }
    assert_eq!(hands.len(), 1000);
    let mut wins = 0;
    for &(ref first, ref second) in hands.iter() {
        match first.determine_winner(second) {
            Ordering::Greater => {
                wins += 1;
            },
            Ordering::Less => {}
            Ordering::Equal => {
                panic!("Determined hands equal for {} and {}, with rank {:?}", first, second, first.rank())
            }
        }
    }
    Ok(wins)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PokerHand {
    cards: [PokerCard; 5]
}
impl PokerHand {
    #[inline]
    fn new(cards: &[PokerCard]) -> PokerHand {
        assert_eq!(cards.len(), 5);
        let mut result = PokerHand { cards: [cards[0], cards[1], cards[2], cards[3], cards[4]] };
        result.cards.sort_by_key(|card| card.value);
        result
    }
    #[inline]
    fn set(&self) -> PokerSet {
        let mut result = PokerSet::new();
        for card in &self.cards {
            result.insert(*card)
        }
        result
    }
    #[inline]
    fn determine_winner(&self, other: &PokerHand) -> Ordering {
        self.rank().cmp(&other.rank())
            .then_with(|| self.cards.iter().rev().cmp(other.cards.iter().rev()))
    }
    fn rank(&self) -> PokerRank {
        let set = self.set();
        let first: PokerCard = self.cards[0];
        if first.straight_flush() == Some(set) {
            // Check for a royal or straight flush
            return if first.value == 10 {
                PokerRank::RoyalFlush
            } else {
                PokerRank::StraightFlush(first.value)
            }
        }
        /*
         * Search for four of a kind, three of a kind, and pairs.
         * We only have to check the first four cards in order to do this.
         * We break early if we see a four of a kind,
         since that's better than anything lower.
         */
        let mut three_of_a_kind = None;
        let mut pairs = Vec::new();
        for card in &self.cards[..4] {
            match (card.value.kinds() & set).len() {
                1 => {},
                2 => {
                    if !pairs.contains(&card.value) {
                        pairs.push(card.value);
                    }
                },
                3 => {
                    if three_of_a_kind.is_some() {
                        debug_assert_eq!(three_of_a_kind, Some(card.value));
                    }
                    three_of_a_kind = Some(card.value);
                },
                4 => {
                    return PokerRank::FourOfAKind(card.value)
                }
                _ => unreachable!()
            }
        }
        if let (Some(three_of_a_kind), Some(&pair)) = (three_of_a_kind, pairs.first()) {
            // Check for a full house (what an awesome show)
            PokerRank::FullHouse { pair, three_of_a_kind }
        } else if first.suit.cards().contains_all(set) {
            PokerRank::Flush
        } else if first.value.flush().map_or(false, |flush| flush.contains_all(set)) {
            PokerRank::Straight(first.value)
        } else if let Some(three_of_a_kind) = three_of_a_kind {
            PokerRank::ThreeOfAKind(three_of_a_kind)
        } else {
            match pairs.len() {
                0 => PokerRank::HighCard(self.high_card()),
                1 => PokerRank::OnePair(pairs[0]),
                2 => {
                    let highest = pairs[0].max(pairs[1]);
                    let lowest = pairs[0].min(pairs[1]);
                    PokerRank::TwoPairs(highest, lowest)
                },
                _ => unreachable!()
            }
        }
    }
    #[inline]
    pub fn high_card(&self) -> PokerValue {
        self.cards[4].value
    }
}
impl Display for PokerHand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_char('[')?;
        let mut first = true;
        for card in &self.cards {
            if !first {
                f.write_str(", ")?;
            }
            write!(f, "{}", card)?;
            first = false
        }
        f.write_char(']')
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum PokerRank {
    HighCard(PokerValue),
    OnePair(PokerValue),
    TwoPairs(PokerValue, PokerValue),
    ThreeOfAKind(PokerValue),
    Straight(PokerValue),
    Flush,
    FullHouse {
        three_of_a_kind: PokerValue,
        pair: PokerValue
    },
    FourOfAKind(PokerValue),
    StraightFlush(PokerValue),
    RoyalFlush
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub struct PokerCard {
    suit: PokerSuit,
    value: PokerValue
}
impl PokerCard {
    #[inline]
    pub fn id(self) -> u8 {
        self.suit.id() * 16 + self.value.id()
    }
    #[inline]
    pub fn from_id(id: u8) -> Option<PokerCard> {
        let suit = PokerSuit::from_id(id / 16)?;
        let value = PokerValue::from_id(id % 16)?;
        Some(PokerCard { suit, value })
    }
    pub fn parse(text: &str) -> Result<PokerCard, Error> {
        if text.len() == 2 {
            let bytes = text.as_bytes();
            if let Some(value) = PokerValue::parse(bytes[0] as char) {
                if let Some(suit) = PokerSuit::parse(bytes[1] as char) {
                    return Ok(PokerCard { suit, value })
                }
            }
        }
        bail!("Invalid card: {:?}", text)
    }
    pub fn until(self, end: PokerValue) -> PokerSet {
        assert!(self.value <= end);
        let mut result = PokerSet::new();
        for id in self.value.id()..=end.id() {
            result.insert(PokerCard {
                value: PokerValue::from_id(id).unwrap(),
                suit: self.suit
            })
        }
        result
    }
    /// Create a `PokerSet` of a flush starting with this card,
    /// or `None` if the card is too high to start a flush.
    #[inline]
    pub fn straight_flush(self) -> Option<PokerSet> {
        self.value.checked_add(4)
            .map(|end| self.until(end))
    }
}
impl Display for PokerCard {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_char(self.value.print())?;
        f.write_char(self.suit.print())
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum PokerSuit {
    Diamonds,
    Hearts,
    Clubs,
    Spades
}
impl PokerSuit {
    const ALL: [PokerSuit; 4] = [
        PokerSuit::Diamonds, PokerSuit::Hearts,
        PokerSuit::Clubs, PokerSuit::Spades
    ];
    #[inline]
    pub fn id(self) -> u8 {
        self as u8
    }
    #[inline]
    pub fn from_id(id: u8) -> Option<PokerSuit> {
        Some(match id {
            0 => PokerSuit::Diamonds,
            1 => PokerSuit::Hearts,
            2 => PokerSuit::Clubs,
            3 => PokerSuit::Hearts,
            _ => return None
        })
    }
    #[inline]
    pub fn parse(c: char) -> Option<PokerSuit> {
        Some(match c {
            'D' => PokerSuit::Diamonds,
            'H' => PokerSuit::Hearts,
            'C' => PokerSuit::Clubs,
            'S' => PokerSuit::Spades,
            _ => return None
        })
    }
    #[inline]
    pub fn print(self) -> char {
        match self {
            PokerSuit::Diamonds => 'D',
            PokerSuit::Hearts => 'H',
            PokerSuit::Clubs => 'C',
            PokerSuit::Spades => 'S',
        }
    }
    /// The set of all possible cards in this suit
    #[inline]
    pub fn cards(self) -> PokerSet {
        PokerSet(((1u64 << 13) - 1) << (16 * self.id()))
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum PokerValue {
    One,
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
    King,
    Queen,
    Ace,
}
impl PokerValue {
    #[inline]
    pub fn parse(c: char) -> Option<PokerValue> {
        Some(match c {
            'A' => PokerValue::Ace,
            'K' => PokerValue::King,
            'Q' => PokerValue::Queen,
            'J' => PokerValue::Jack,
            'T' => PokerValue::Ten,
            '0'...'9' => PokerValue::from_value((c as u8) - ('0' as u8)).unwrap(),
            _ => return None
        })
    }
    pub fn print(self) -> char {
        match self {
            PokerValue::Ace => 'A',
            PokerValue::King => 'K',
            PokerValue::Queen => 'Q',
            PokerValue::Jack => 'J',
            PokerValue::Ten => 'T',
            _ => (('0' as u8) + self.value()) as char
        }
    }
    #[inline]
    pub fn id(self) -> u8 {
        self as u8
    }
    #[inline]
    pub fn value(self) -> u8 {
        self.id() + 1
    }
    #[inline]
    pub fn from_value(value: u8) -> Option<PokerValue> {
        PokerValue::from_id(value.wrapping_sub(1))
    }
    #[inline]
    pub fn from_id(value: u8) -> Option<PokerValue> {
        Some(match value {
            0 => PokerValue::One,
            1 => PokerValue::Two,
            2 => PokerValue::Three,
            3 => PokerValue::Four,
            4 => PokerValue::Five,
            5 => PokerValue::Six,
            6 => PokerValue::Seven,
            7 => PokerValue::Eight,
            8 => PokerValue::Nine,
            9 => PokerValue::Ten,
            10 => PokerValue::Jack,
            11 => PokerValue::Queen,
            12 => PokerValue::King,
            13 => PokerValue::Ace,
            _ => return None
        })
    }
    #[inline]
    pub fn checked_add(self, value: u8) -> Option<PokerValue> {
        let result = self.id() + value;
        if result < 14 && result >= self.id() {
            Some(PokerValue::from_id(result).unwrap())
        } else {
            None
        }
    }
    #[inline]
    pub fn kinds(self) -> PokerSet {
        let mut result = PokerSet::new();
        for &suit in &PokerSuit::ALL {
            result.insert(PokerCard { value: self, suit })
        }
        result
    }
    /// The set of possible values that could make up a flush,
    /// or `None` if this card can't start a flush
    pub fn flush(&self) -> Option<PokerSet> {
        let end = self.checked_add(4)?;
        let mut result = PokerSet::new();
        for id in self.id()..=end.id() {
            result |= PokerValue::from_id(id).unwrap().kinds();
        }
        Some(result)
    }
}
impl Add<u8> for PokerValue {
    type Output = PokerValue;

    #[inline]
    fn add(self, rhs: u8) -> PokerValue {
        self.checked_add(rhs)
            .unwrap_or_else(|| panic!("Unable to add {} to {:?}", rhs, self))
    }
}
impl PartialEq<u8> for PokerValue {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        // We check for value and not id, per the principle of least suprise
        self.value() == *other
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct PokerSet(u64);
impl PokerSet {
    #[inline]
    pub const fn new() -> PokerSet {
        PokerSet(0)
    }
    #[inline]
    pub fn contains_all(self, other: PokerSet) -> bool {
        (self.0 & other.0) == other.0
    }
    #[inline]
    pub fn insert(&mut self, card: PokerCard) {
        self.0 |= 1 << card.id();
    }
    #[inline]
    pub fn len(self) -> u32 {
        self.0.count_ones()
    }
}
impl BitOr for PokerSet {
    type Output = PokerSet;

    #[inline]
    fn bitor(self, rhs: PokerSet) -> Self::Output {
        PokerSet(self.0 | rhs.0)
    }
}
impl BitAnd for PokerSet {
    type Output = PokerSet;

    #[inline]
    fn bitand(self, rhs: PokerSet) -> Self::Output {
        PokerSet(self.0 & rhs.0)
    }
}
impl BitOrAssign for PokerSet {
    #[inline]
    fn bitor_assign(&mut self, rhs: PokerSet) {
        self.0 |= rhs.0;
    }
}
