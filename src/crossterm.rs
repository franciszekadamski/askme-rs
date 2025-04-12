use color_eyre::Result;
use ratatui;

use crate::app::App;
use crate::deck::Deck;


pub fn run(filename: &str) -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(filename).run(terminal);
    ratatui::restore();
    result
}
