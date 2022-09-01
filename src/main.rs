use core::fmt;
use std::{
    error::Error,
    fmt::Display,
    io::{self, BufRead},
};

use rand::seq::SliceRandom;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match *self {
            Suit::Spades => "Spades",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
        }
        .to_string();
        write!(f, "{}", result)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Value {
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

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match *self {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::Ace => "A",
        }
        .to_string();
        write!(f, "{}", result)
    }
}

impl From<Value> for usize {
    fn from(value: Value) -> Self {
        match value {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten | Value::Jack | Value::Queen | Value::King => 10,
            Value::Ace => 11,
        }
    }
}

const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
const VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace,
];

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Self { value, suit }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", &self.value, &self.suit,)
    }
}

fn create_deck() -> Vec<Card> {
    let mut cards = Vec::new();

    for suit in SUITS {
        for value in VALUES {
            cards.push(Card::new(value, suit));
        }
    }

    cards
}

fn card_total(cards: &[Card]) -> usize {
    cards
        .iter()
        .fold(0, |total, card| total + usize::from(card.value))
}

fn print_dealer_cards(cards: &[Card]) {
    println!("Dealer cards:");
    let mut first_hidden = false;
    cards.iter().for_each(|card| match first_hidden {
        true => println!("* {}", card),
        false => {
            println!("* ??");
            first_hidden = true;
        }
    });
    println!("")
}

fn print_player_cards(cards: &[Card]) {
    println!("Your cards:");
    for card in cards.iter() {
        println!("* {}", card);
    }
    println!("* Total: {}", card_total(&cards));
    println!("")
}

fn print_player_options() {
    println!(
        r#"
Options
1.) Hit
2.) Stay
    "#
    );
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

const MAX_VALUE: usize = 21;

fn play_round() -> Result<bool, Box<dyn Error>> {
    let mut deck = create_deck();
    deck.shuffle(&mut rand::thread_rng());

    let mut dealer_cards = Vec::new();
    dealer_cards.extend_from_slice(deck.split_off(deck.len() - 2).as_slice());

    let mut player_cards = Vec::new();
    player_cards.extend_from_slice(deck.split_off(deck.len() - 2).as_slice());

    clear_screen();
    print_dealer_cards(&dealer_cards);
    print_player_cards(&player_cards);
    print_player_options();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        clear_screen();
        print_dealer_cards(&dealer_cards);
        print_player_cards(&player_cards);
        print_player_options();

        match line.as_str() {
            "1" => {
                // Hit
                player_cards.extend_from_slice(deck.split_off(deck.len() - 1).as_slice());
                if card_total(&player_cards) > MAX_VALUE {
                    print_player_cards(&player_cards);

                    println!("You went over {}! Game over.", MAX_VALUE);
                    println!("Thanks for playing!");
                    return Ok(false);
                }
            }
            "2" => {
                // Stand
                let player_total = card_total(&player_cards);
                let dealer_total = card_total(&dealer_cards);
                if player_total < dealer_total {
                    println!(
                        "You lost! [Dealer score ({}) > Player score ({})]",
                        dealer_total, player_total
                    );
                    return Ok(false);
                } else {
                    println!(
                        "You won! [Dealer score: ({}) < Player score: ({})]",
                        dealer_total, player_total
                    );
                    return Ok(true);
                }
            }
            _ => println!("Invalid option. Please select either 'Hit' or 'Stay'."),
        }
    }

    Ok(false)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Welcome to Matt's Blackjack table! ---");
    println!("Press any key to start playing.");
    io::stdin().lines().next().unwrap()?;

    clear_screen();
    println!("--- Matt's Blackjack table ---");

    play_round()?;

    Ok(())
}
