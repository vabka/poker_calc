#![allow(dead_code)]
use super::deck::{
    Card,
    Rank::{self, *},
};

use std::collections::HashMap;
/// 5 Карт
pub struct Hand([Card; 5]);

use core::fmt::{self, Display, Formatter};
impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut cards = self.cards().iter();
        if let Some(card) = cards.next() {
            card.fmt(f)?;
            for card in cards {
                f.write_str(" ")?;
                card.fmt(f)?;
            }
        };
        Ok(())
    }
}

pub enum PokerCombo {
    Top(Rank),
    Pair(Rank),
    TwoPairs([Rank; 2]),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank),
    FourOfAKind(Rank),
    StraightFlush(Rank),
}

use PokerCombo::*;
impl Hand {
    #[inline]
    pub fn new(cards: [Card; 5]) -> Hand {
        let mut cards = cards;
        cards.sort();
        Hand(cards)
    }
    #[inline]
    pub fn cards(&self) -> &[Card; 5] {
        &self.0
    }
    fn count_ranks(&self) -> HashMap<Rank, u8> {
        let mut map: HashMap<Rank, u8> = HashMap::new();
        for rank in self.cards().iter().map(|x| x.rank()) {
            let count = map.get(&rank);
            if let Some(count) = count {
                let count = *count;
                map.insert(rank, count + 1);
            } else {
                map.insert(rank, 0);
            }
        }
        map
    }

    fn get_card_groups(&self, cards_count_in_group: u8) -> Vec<Rank> {
        let map = self.count_ranks();
        let grs: Vec<_> = map
            .iter()
            .filter(|&(_, cnt)| *cnt == cards_count_in_group)
            .map(|x| x.0.to_owned())
            .collect();
        grs
    }

    fn top_rank(&self) -> Rank {
        self.cards().iter().map(|x| x.rank()).max().unwrap()
    }

    fn straight_rank(&self) -> Option<Rank> {
        let cards = self.cards();
        let hand: Vec<_> = cards.iter().map(|x| x.rank()).collect();
        match hand.as_slice() {
            [Two, Three, Four, Five, Ace] => Some(Five),
            [Two, Three, Four, Five, Six] => Some(Six),
            [Three, Four, Five, Six, Seven] => Some(Seven),
            [Four, Five, Six, Seven, Eight] => Some(Eight),
            [Five, Six, Seven, Eight, Nine] => Some(Nine),
            [Six, Seven, Eight, Nine, Ten] => Some(Ten),
            [Seven, Eight, Nine, Ten, Jack] => Some(Jack),
            [Eight, Nine, Ten, Jack, Queen] => Some(Queen),
            [Nine, Ten, Jack, Queen, King] => Some(King),
            [Ten, Jack, Queen, King, Ace] => Some(Ace),
            _ => None,
        }
    }

    fn flush_rank(&self) -> Option<Rank> {
        let cards = self.cards();
        let suit = cards[0].suit();
        let is_flush = cards.iter().skip(1).map(|x| x.suit()).all(|x| x == suit);
        if is_flush {
            Some(self.top_rank())
        } else {
            None
        }
    }

    fn four_of_a_kind_rank(&self) -> Option<Rank> {
        let quads = self.get_card_groups(4);
        if let [quad_rank] = quads.as_slice() {
            Some(*quad_rank)
        } else {
            None
        }
    }

    fn three_of_a_kind_rank(&self) -> Option<Rank> {
        let triplets = self.get_card_groups(3);
        if let [triplet_rank] = triplets.as_slice() {
            Some(*triplet_rank)
        } else {
            None
        }
    }

    fn get_pairs(&self) -> Vec<Rank> {
        self.get_card_groups(2)
    }

    fn pair_rank(&self) -> Option<Rank> {
        let pairs = self.get_pairs();
        if let [pair_rank] = *pairs.as_slice() {
            Some(pair_rank)
        } else {
            None
        }
    }

    fn two_pairs_ranks(&self) -> Option<[Rank; 2]> {
        let pairs = self.get_pairs();
        if pairs.len() == 2 {
            let first_pair_rank = pairs[0];
            let second_pair_rank = pairs[1];
            Some([first_pair_rank.to_owned(), second_pair_rank.to_owned()])
        } else {
            None
        }
    }

    fn full_house_rank(&self) -> Option<Rank> {
        let triplet_rank = self.three_of_a_kind_rank()?;
        self.pair_rank()?;
        Some(triplet_rank)
    }
    fn is_flush(&self) -> bool {
        self.flush_rank().is_some()
    }
    ///Получить текущее комбо в руке
    pub fn get_combo(&self) -> PokerCombo {
        if let Some(straight_rank) = self.straight_rank() {
            if self.is_flush() {
                StraightFlush(straight_rank)
            } else {
                Straight(straight_rank)
            }
        } else if let Some(flush_rank) = self.flush_rank() {
            Flush(flush_rank)
        } else if let Some(four_of_a_kind_rank) = self.four_of_a_kind_rank() {
            FourOfAKind(four_of_a_kind_rank)
        } else if let Some(full_house_rank) = self.full_house_rank() {
            FullHouse(full_house_rank)
        } else if let Some(three_of_a_kind_rank) = self.three_of_a_kind_rank() {
            ThreeOfAKind(three_of_a_kind_rank)
        } else if let Some(two_pairs_ranks) = self.two_pairs_ranks() {
            TwoPairs(two_pairs_ranks)
        } else if let Some(pair_rank) = self.pair_rank() {
            Pair(pair_rank)
        } else {
            Top(self.top_rank())
        }
    }
}
