#![allow(dead_code)]
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
/// Ранг/Цена/Старшинство карты
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
///Масть карты
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
///Карта представляет из себя ранг и масть
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Card(Rank, Suit);
impl Card {
    ///Ранг карты
    #[inline]
    pub const fn rank(self) -> Rank {
        self.0
    }
    ///Масть карты
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
    ///### Конструктор для карты
    /// ```
    /// use Rank::*;
    /// use Suit::*;
    /// let card = Ace.of(Spades);
    /// assert_eq!(card.suit(), Spades);
    /// assert_eq!(card.rank(), Ace);
    /// ```
    #[inline]
    pub const fn of(self, suit: Suit) -> Card {
        Card(self, suit)
    }
}

impl Rank {
    fn enumerate() -> &'static [Rank; 13] {
        use Rank::*;
        &[
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ]
    }
}

impl Suit {
    fn enumerate() -> &'static [Suit; 4] {
        use Suit::*;
        &[Spades, Clubs, Diamonds, Hearts]
    }
}

pub struct Deck([Card; 52]);
use rand::prelude::*;
impl Deck {
    pub fn cards(&self) -> &[Card; 52] {
        &self.0
    }

    pub fn shuffle<R: Rng>(rand: &mut R) -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(52);
        for suit in Suit::enumerate() {
            for rank in Rank::enumerate() {
                cards.push(rank.of(*suit));
            }
        }
        cards.shuffle(rand);
        use Rank::*;
        use Suit::*;
        let mut arr: [Card; 52] = [Ace.of(Spades); 52];
        arr.copy_from_slice(cards.as_slice());
        Deck(arr)
    }
}
#[cfg(test)]
mod tests {
    use super::Rank::*;
    use super::Suit::*;
    use super::*;
    #[test]
    fn equality_of_cards() {
        assert!(Ace > King);
        assert!(Ace.of(Spades) > King.of(Spades));
        assert_eq!(Ace.of(Spades), Ace.of(Spades));
        assert_ne!(Ace.of(Spades), Ace.of(Hearts));
        assert_ne!(Ace.of(Spades), King.of(Spades));
    }

    #[test]
    fn same_addr() {
        let a = Rank::enumerate().as_ptr();
        let b = Rank::enumerate().as_ptr();
        assert!(a == b);
    }

    #[test]
    fn ok_shuffle() {
        use std::collections::HashSet;
        let mut rand = rand::thread_rng();
        let deck = Deck::shuffle(&mut rand);
        let cards = deck.cards();
        let mut set: HashSet<_> = HashSet::default();
        for card in cards.iter() {
            set.insert(card);
        }
        assert!(set.len() == cards.len());
    }
}
