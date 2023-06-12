use crate::item::Item;

use std::ops::{Range, RangeFrom};

use owo_colors::OwoColorize;
use tabled::settings::object::Segment;
use tabled::settings::{
    object::Rows, peaker::PriorityMax, Alignment, Format, Modify, Settings, Style, Width,
};
use tabled::Table;
use terminal_size::terminal_size;

pub fn to_table(items: &[Item<'_>]) -> Table {
    let mut table = Table::new(items);
    let settings = Settings::default()
        .with(Style::rounded())
        .with(Modify::new(Rows::first()).with(Alignment::center()))
        .with(Width::wrap(get_terminal_width()).priority::<PriorityMax>())
        .with(
            Modify::new(Rows::first())
                .with(Format::content(|s| s.bright_green().bold().to_string())),
        )
        .with(Modify::new(column_body(0)).with(Format::content(|s| s.blue().to_string())))
        .with(Modify::new(column_body(1)).with(Format::content(|s| s.yellow().to_string())))
        .with(Modify::new(column_body(3)).with(Format::content(|s| s.cyan().to_string())));
    table.with(settings);
    table
}

#[inline]
fn get_terminal_width() -> usize {
    terminal_size().expect("STDOUT is not a tty").0 .0 as usize
}

#[inline]
fn column_body(i: usize) -> Segment<Range<usize>, RangeFrom<usize>> {
    Segment::new(1.., i..i + 1)
}
