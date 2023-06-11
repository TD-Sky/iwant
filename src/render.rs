use crate::item::Item;

use tabled::settings::Settings;
use tabled::settings::Style;
use tabled::settings::{object::Rows, Alignment, Modify};
use tabled::settings::{peaker::PriorityMax, Width};
use tabled::Table;
use terminal_size::terminal_size;

#[inline]
fn get_terminal_width() -> usize {
    terminal_size().expect("STDOUT is not a tty").0 .0 as usize
}

pub fn to_table(items: &[Item<'_>]) -> Table {
    let mut table = Table::new(items);
    let settings = Settings::default()
        .with(Style::rounded())
        .with(Modify::new(Rows::first()).with(Alignment::center()))
        .with(Width::wrap(get_terminal_width()).priority::<PriorityMax>());
    table.with(settings);
    table
}
