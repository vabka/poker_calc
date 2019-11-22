#![allow(dead_code)]
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Rank {
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

impl Rank {
    #[inline]
    fn get_symbol(self) -> &'static str {
        use Rank::*;
        match self {
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "10",
            Jack => "J",
            Queen => "Q",
            King => "K",
            Ace => "A",
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.get_symbol())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
impl Suit {
    #[inline]
    fn get_symbol(self) -> &'static str {
        use Suit::*;
        // ♧ ♤ ♡ ♢
        match self {
            Spades => "♠",
            Hearts => "♥",
            Diamonds => "♦",
            Clubs => "♣",
        }
    }
}
impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.get_symbol())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Card(Rank, Suit);
impl Card {
    #[inline]
    pub const fn rank(self) -> Rank {
        self.0
    }
    #[inline]
    pub const fn suit(self) -> Suit {
        self.1
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.rank().fmt(f)?;
        self.suit().fmt(f)
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Rank {
    #[inline]
    pub const fn of(self, suit: Suit) -> Card {
        Card(self, suit)
    }
}

#[cfg(test)]
mod tests {
    use super::Rank::*;
    use super::Suit::*;

    #[test]
    fn equality_of_cards() {
        assert!(Ace < King);
        assert_eq!(Ace.of(Spades), Ace.of(Spades));
        assert_ne!(Ace.of(Spades), Ace.of(Hearts));
        assert_ne!(Ace.of(Spades), King.of(Spades));
    }
}
