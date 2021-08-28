use hunger_games::data::Position;
use hunger_games::item_db;
use hunger_games::map_reader;
use hunger_games::pathfind;
use hunger_games::walker;

type Vars = u64;

fn mul(vars: Vars, values: &[i64]) -> i64 {
    let mut sum = 0;

    for (i, item) in values.iter().enumerate() {
        let mul = ((vars >> i) & 0b1) as i64;
        sum += mul * item;
    }

    sum
}

fn main() {
    let item_data = std::fs::read_to_string("data/items.csv").unwrap();
    let item_db = item_db::Database::from_csv(&item_data).unwrap();

    let values = item_db
        .get_values("Value")
        .unwrap()
        .iter()
        .map(|v| {
            if let item_db::Element::Full(i) = v {
                i.parse().unwrap()
            } else {
                0i64
            }
        })
        .collect::<Vec<_>>();
    let costs = item_db
        .get_values("Weight")
        .unwrap()
        .iter()
        .map(|v| {
            if let item_db::Element::Full(i) = v {
                i.parse().unwrap()
            } else {
                0i64
            }
        })
        .collect::<Vec<_>>();

    let scores = |vars: Vars| -> i64 { mul(vars, values.as_slice()) };

    let weight = |vars: Vars| -> i64 { mul(vars, costs.as_slice()) };

    #[derive(Debug, Default, Copy, Clone)]
    struct Path {
        score: i64,
        vars: u64,
    }

    // get the best possible paths
    let mut best = [Path::default(); 40];

    for set in 0..=2_u64.pow(20) {
        if weight(set) <= 30 {
            let worst_score = best[best.len() - 1];

            let score = Path {
                score: scores(set),
                vars: set,
            };

            if score.score > worst_score.score {
                // we have a new best score
                for (i, bs) in best.iter().copied().enumerate() {
                    if score.score > bs.score {
                        for j in (i + 1..best.len()).rev() {
                            best[j] = best[j - 1];
                        }
                        best[i] = score;
                        break;
                    }
                }
            }
        }
    }

    // find the first path that is valid
    let map_data = std::fs::read_to_string("data/map.txt").unwrap();
    let map = map_reader::Map::from_data(&map_data).unwrap();

    let items = hunger_games::data::all_map_items(&item_db, &map);

    for path in best {
        //println!("Evaluating path {:#b} with score {}", path.vars, path.score);
        let items: Vec<_> = items
            .iter()
            .cloned()
            .filter(|i| (path.vars >> ((i.id - 1) as u64)) & 1 == 1)
            .collect();
        let best_path = pathfind::find_best_path(Position(0, 0), &items[..], 30, 30);
        // we have our path. walk it
        if walker::collects_all_items(&best_path.positions[..], &items) {
            // we have a best path
            walker::walk(&best_path.positions[..], &items, true);
            break;
        }
    }
}
