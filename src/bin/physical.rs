use hunger_games::data::Position;
use hunger_games::item_db;
use hunger_games::map_reader;
use hunger_games::pathfind;
use hunger_games::walker;

fn main() {
    let item_data = std::fs::read_to_string("data/items.csv").unwrap();
    let item_db = item_db::Database::from_csv(&item_data).unwrap();

    let map_data = std::fs::read_to_string("data/map.txt").unwrap();
    let map = map_reader::Map::from_data(&map_data).unwrap();

    // problem description: in only 30 moves, get the best possible score
    // by moving around the map and collecting items. there is a max of 30
    // kg that can be carried

    // the best path is the one with the largest survival score.

    let items = hunger_games::data::all_map_items(&item_db, &map);
    let best_path = pathfind::find_best_path(Position(0, 0), &items[..], hunger_games::MAX_MOVES, hunger_games::MAX_WEIGHT).positions;
    walker::walk(&best_path, &items[..], true);
}
