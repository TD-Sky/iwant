use crate::item::Item;
use tabled::object::{Column, Object, Rows};
use tabled::Alignment;
use tabled::Modify;
use tabled::Style;
use tabled::Table;
use tabled::TableIteratorExt;

pub fn to_table(items: &[Item<'_>]) -> Table {
    let mut table = items.table();
    table
        .with(Style::rounded())
        .with(Modify::new(Rows::first().and(Column::from(2))).with(Alignment::center()));
    table
}
