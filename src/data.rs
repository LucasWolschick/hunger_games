use crate::{item_db, map_reader};

#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub position: Position,
    pub weight: i32,
    pub value: i32,
    pub id: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn distance(&self, rhs: Position) -> i32 {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }
}

pub fn all_map_items(item_db: &item_db::Database, map: &map_reader::Map) -> Vec<Item> {
    let mut items = vec![];

    for y in 0..map.height() {
        for x in 0..map.width() {
            if let map_reader::Cell::Filled(id) = map.index(x, y) {
                if let Some(index) = item_db.index("Item", id.to_string().as_ref()) {
                    let value =
                        if let item_db::Element::Full(s) = item_db.value("Value", index).unwrap() {
                            s.parse().unwrap_or_default()
                        } else {
                            0
                        };
                    let weight = if let item_db::Element::Full(s) =
                        item_db.value("Weight", index).unwrap()
                    {
                        s.parse().unwrap_or_default()
                    } else {
                        0
                    };
                    items.push(Item {
                        position: Position(x as i32, y as i32),
                        value,
                        weight,
                        id: id as i32,
                    });
                }
            }
        }
    }

    items
}
