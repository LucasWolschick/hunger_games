use std::fmt::Display;

use crate::data::{Item, Position};

#[derive(Debug)]
pub enum WalkResult {
    Success,
    NoSpaceLeft,
    NoTimeLeft,
}

impl Display for WalkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalkResult::Success => f.write_str("Success"),
            WalkResult::NoSpaceLeft => f.write_str("Ran out of space"),
            WalkResult::NoTimeLeft => f.write_str("Ran out of time"),
        }
    }
}

pub fn walk(path: &[Position], items: &[Item], log: bool) -> WalkResult {
    let mut my_pos = Position(0, 0);
    let mut my_distance = crate::MAX_MOVES;
    let mut my_score = 0;
    let mut my_weight = crate::MAX_WEIGHT;
    for step in path {
        let item = items.iter().find(|x| x.position == *step);
        if step.distance(my_pos) <= my_distance {
            my_distance -= step.distance(my_pos);
            my_pos = *step;
            if let Some(item) = item {
                if item.weight <= my_weight {
                    my_score += item.value;
                    my_weight -= item.weight;
                } else {
                    return WalkResult::NoSpaceLeft;
                }
            }
        } else {
            return WalkResult::NoTimeLeft;
        }
        if log {
            println!(
                "({}, {}), ${}, {}/{}kg, {}m left",
                my_pos.0,
                my_pos.1,
                my_score,
                crate::MAX_WEIGHT - my_weight,
                crate::MAX_WEIGHT,
                my_distance
            );
        }
    }

    WalkResult::Success
}

pub fn collects_all_items(path: &[Position], items: &[Item]) -> bool {
    let mut my_pos = Position(0, 0);
    let mut my_distance = crate::MAX_MOVES;
    let mut my_weight = crate::MAX_WEIGHT;
    let mut collected = 0;
    for step in path {
        let item = items.iter().find(|x| x.position == *step);
        if step.distance(my_pos) <= my_distance {
            my_distance -= step.distance(my_pos);
            my_pos = *step;
            if let Some(item) = item {
                if item.weight <= my_weight {
                    collected += 1;
                    my_weight -= item.weight;
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    collected == items.len()
}
