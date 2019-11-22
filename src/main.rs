mod cards;
fn main() {
    use cards::{poker, Rank::*, Suit::*};
    let hand = poker::Hand::new([
        Ace.of(Spades),
        Five.of(Hearts),
        Six.of(Clubs),
        King.of(Spades),
        Two.of(Diamonds),
    ]);
    println!("{}", hand);
}
