use std::cmp::Ordering::{Less, Equal, Greater};
use std::error::Error;
use std::fs::File;
use rand::seq::SliceRandom;
use rand::thread_rng;
use csv::Reader;


#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
    current_card_index: usize
}

#[derive(Debug)]
struct Card {
    front: String,
    back: String,
    side: Side,
}

#[derive(Debug)]
enum Side {
    Front,
    Back,
}

impl Card {
    pub fn new(front: String, back: String) -> Self {
        Card { front, back, side: Side::Front }
    }

    pub fn flip(&mut self) {
        match self.side {
            Side::Front => self.side = Side::Back,
            Side::Back => self.side = Side::Front,
        };
    }

    fn get_text(&self) -> &String {
        match self.side {
            Side::Front => &self.front,
            Side::Back => &self.back,
        }
    }

    fn get_side(&self) -> &str {
        match self.side {
            Side::Front => "Front",
            Side::Back => "Back",
        }
    }
}

impl Deck {
    pub fn new(filename: String) -> Result<Self, Box<dyn Error>> {
        let mut cards: Vec<Card> = vec![];

        let file = File::open(filename)?;
        let mut reader = Reader::from_reader(file);

        for result in reader.records() {
            let record = result?;
            let front = record.get(0).unwrap_or("could not read");
            let back = record.get(1).unwrap_or("could not read");
            let card = Card::new(front.to_string(), back.to_string());
            cards.push(card);
        }
        
        Ok(Deck {
            cards,
            current_card_index: 0
        })
    }

    pub fn get_current_card_text(&self) -> &str {
        // TODO: use [] or write a get method for accessing the current card
        self.cards
            .get(self.current_card_index)
            .expect("Card was accessed by index")
            .get_text()
    }

    pub fn get_current_card_side(&self) -> &str {
        self.cards
            .get(self.current_card_index)
            .expect("Card was accessed by index")
            .get_side()
    }

    pub fn get_current_card_index(&self) -> usize {
        self.current_card_index
    }

    pub fn get_deck_size(&self) -> usize {
        self.cards.len()
    }

    pub fn flip_current_card(&mut self) {
        self.cards
            .get_mut(self.current_card_index)
            .expect("Card was accessed by index")
            .flip()
    }

    pub fn next(&mut self) {
        match self.current_card_index.cmp(&(self.cards.len() - 1)) {
            Less => self.current_card_index += 1,
            Equal => self.current_card_index = 0,
            Greater => self.current_card_index = 0,
        };
        self.cards[self.current_card_index].side = Side::Front;
    }

    pub fn prev(&mut self) {
        match &self.current_card_index.cmp(&0) {
            Less => self.current_card_index = self.cards.len() - 1,
            Equal => self.current_card_index = self.cards.len() - 1,
            Greater => self.current_card_index -= 1,
        }
        self.cards[self.current_card_index].side = Side::Front;
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}
