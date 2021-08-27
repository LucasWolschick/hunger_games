use crate::data::Position;

type Id = u32;

#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Empty,
    Filled(Id)
}

pub struct Map {
    data: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn from_data(str: &str) -> Option<Self> {
        let mut data = vec![];
        let height = str.lines().count();
        let mut width = None;

        for line in str.lines() {
            let mut i = 0;
            let chars: Vec<_> = line.trim().chars().collect();
            let mut w = 0;
            while i < chars.len() {
                let c = chars[i];
                if c.is_numeric() {
                    // chomp chomp til we done
                    let mut to = i+1;
                    while to < chars.len() && chars[to].is_numeric() {
                        to += 1;
                    }
                    let num: Id = chars[i..to].iter().collect::<String>().as_str().parse().unwrap();
                    data.push(Cell::Filled(num));
                    w += 1;
                    i = to;
                } else if !c.is_whitespace() {
                    data.push(Cell::Empty);
                    w += 1;
                    i += 1;
                } else {
                    i += 1;
                }
            }
            
            if let Some(width) = width {
                if width != w {
                    eprintln!("Invalid map input: differing lengths!");
                    return None;
                }
            } else {
                width = Some(w)
            }
        }

        Some(Self {
            data,
            width: width.unwrap_or(0),
            height,
        })
    }

    pub fn index(&self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            Cell::Empty
        } else {
            self.data[y*self.width + x]
        }
    }

    pub fn find_coords(&self, item_id: Id) -> Option<Position> {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Cell::Filled(id) = self.data[y*self.width + x] {
                    if id == item_id {
                        return Some(Position(x as _, y as _))
                    }
                }
            }
        }
        None
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}