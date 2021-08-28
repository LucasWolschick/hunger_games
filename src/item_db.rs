#[derive(Clone, Debug)]
pub enum Element {
    Empty,
    Full(String),
}

pub struct Database {
    headers: Vec<String>,
    data: Vec<Vec<Element>>,
    // items: usize,
}

#[repr(transparent)]
pub struct DbIndex(usize);

impl Database {
    pub fn from_csv(contents: &str) -> Option<Self> {
        let contents = contents.trim();
        let mut lines = contents.lines();
        let header = lines.next()?;

        let width = lines
            .clone()
            .map(|s| s.split(',').count())
            .max()
            .unwrap_or(0);
        let height = lines.clone().count();

        let mut headers: Vec<_> = header.split(',').map(str::to_string).collect();
        if headers.len() < width {
            let delta = width - headers.len();
            headers.extend(std::iter::repeat(String::new()).take(delta));
        }

        let mut data = vec![vec![Element::Empty; height]; width];
        for (elem_id, line) in lines.enumerate() {
            for (prop_id, value) in line.trim().split(',').enumerate() {
                data[prop_id][elem_id] = Element::Full(value.into());
            }
        }

        Some(Self {
            headers,
            data,
            // items: height
        })
    }

    pub fn category(&self, name: &str) -> Option<DbIndex> {
        self.headers
            .iter()
            .enumerate()
            .find(|(_, e)| *e == name)
            .map(|(i, _)| DbIndex(i))
    }

    pub fn get_values(&self, category: &str) -> Option<&[Element]> {
        Some(self.data[self.category(category)?.0].as_slice())
    }

    // pub fn len(&self) -> usize {
    //     self.items
    // }

    pub fn index(&self, key: &str, value: &str) -> Option<usize> {
        let cat_id = self.category(key).map(|v| v.0)?;

        Some(
            self.data[cat_id]
                .iter()
                .enumerate()
                .find(|(_, e)| {
                    if let Element::Full(f) = e {
                        f.as_str() == value
                    } else {
                        false
                    }
                })?
                .0,
        )
    }

    pub fn value(&self, key: &str, index: usize) -> Option<&Element> {
        self.data.get(self.category(key).map(|v| v.0)?)?.get(index)
    }
}
