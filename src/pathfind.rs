use crate::data::{Item, Position};

type Score = i32;

pub struct PathResult {
    pub positions: Vec<Position>,
    pub score: Score,
}

pub fn find_best_path(start: Position, available_items: &[Item], max_steps: i32, max_weight: i32) -> PathResult {
    fn recurse(start: Position, items: &[Item], max_steps: i32, max_weight: i32, start_score: Score, mut history: Vec<Position>) -> (Position, Score, Vec<Position>) {
        let reachable: Vec<_> = items.iter().filter(|i| i.position.distance(start) <= max_steps && i.weight <= max_weight).collect();
    
        history.push(start);

        let mut best = (start, start_score, history.clone());
        
        for (i, item) in reachable.iter().enumerate() {
            let items_minus_item: Vec<_> = reachable.iter().enumerate().filter(|(ind,_)| *ind != i).map(|(_,i)| **i).collect();
            let result = recurse(item.position, items_minus_item.as_slice(), max_steps - item.position.distance(start), max_weight - item.weight, start_score + item.value, history.clone());
            if result.1 > best.1 {
                best = result;
            }
        }

        best
    }
    
    let (_, s, p) = recurse(start, available_items, max_steps, max_weight, 0, vec![]);
    PathResult { positions: p, score: s }
}