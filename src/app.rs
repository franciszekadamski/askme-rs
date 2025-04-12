use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::deck::Deck;

#[derive(Debug)]
pub struct App {
    running: bool,
    deck: Deck
}

impl App {
    pub fn new(filename: &str) -> Self {
        let deck = Deck::new(filename.to_string()).expect("Could not open the deck");
        App { running: true, deck }
        
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let side = self.deck.get_current_card_side();
        let card_number = self.deck.get_current_card_index() + 1;
        let deck_size = self.deck.get_deck_size();
        let title = format!(" {card_number}/{deck_size} {side} ");
        let title = Line::from(title)
            .bold()
            .blue()
            .centered();
        let text = self.deck.get_current_card_text();
        let formatted_text = format!("\n{text}");
        frame.render_widget(
            Paragraph::new(formatted_text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char('l')) => self.deck.next(),
            (_, KeyCode::Char('h')) => self.deck.prev(),
            (_, KeyCode::Char('k')) => self.deck.flip_current_card(),
            (_, KeyCode::Char('s')) => self.deck.shuffle(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
