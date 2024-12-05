// public modules for bencher
pub mod files;
pub mod input_finder;
pub mod misc;

use std::cmp::Ordering;
use std::ops::Index;

use misc::AResult;

#[derive(Debug)]
struct OrderingRule {
    lesser: i32,
    greater: i32,
}
#[derive(Debug)]
struct MatchedOrderingRule<'a> {
    rule: &'a OrderingRule,
    left_idx: usize,
    right_idx: usize,
}
type Update = Vec<i32>;

pub fn run(filename: &str, part: u8) -> AResult<i32> {
    let input = files::load_full_input_as_string(filename)?;
    run_on_string(&input, part)
}

pub fn run_on_string(input: &str, part: u8) -> AResult<i32> {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let rules = sections[0]
        .lines()
        .map(|line| {
            let pair = line
                .split('|')
                .map(|v| v.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;

            match pair.len() {
                2 => Ok(OrderingRule {
                    lesser: pair[0],
                    greater: pair[1],
                }),
                e => Err(format!("Expected two elements per line but got {e}")),
            }
        })
        .collect::<Result<Vec<OrderingRule>, _>>()?;

    let updates = sections[1]
        .lines()
        .map(|l| {
            l.split(',')
                .map(|p| -> Result<i32, String> {
                    Ok(p.parse::<i32>().map_err(|e| e.to_string())?)
                })
                .collect::<Result<Update, _>>()
        })
        .collect::<Result<Vec<Update>, _>>()?;

    let matching_rules = |update: &Update| {
        rules
            .iter()
            .map(|r| {
                (
                    r,
                    update.iter().position(|p| *p == r.lesser),
                    update.iter().position(|p| *p == r.greater),
                )
            })
            .filter(|r| r.1.is_some() && r.2.is_some())
            .map(|v| MatchedOrderingRule {
                rule: v.0,
                left_idx: v.1.unwrap(),
                right_idx: v.2.unwrap(),
            })
            .collect::<Vec<MatchedOrderingRule>>()
    };

    struct UpdateAndMatchingRules<'a> {
        update: &'a Update,
        matched_rules: Vec<MatchedOrderingRule<'a>>,
    }

    let relevant_updates: (Vec<UpdateAndMatchingRules>, Vec<UpdateAndMatchingRules>) = updates
        .iter()
        .map(|update| UpdateAndMatchingRules {
            update,
            matched_rules: matching_rules(update),
        })
        .partition(|u| u.matched_rules.iter().all(|r| r.left_idx < r.right_idx));

    let valid_updates = match part {
        1 => relevant_updates
            .0 // valid updates
            .iter()
            .map(|u| (*u.update).clone()) // only because part 2 needs this of us
            .collect::<Vec<Update>>(),
        2 => relevant_updates
            .1 // rejected updates
            .iter()
            .map(|update| {
                let applicable_rules = &update.matched_rules;
                let mut updated = update.update.clone();
                updated.sort_by(|a, b| {
                    for r in applicable_rules {
                        if *a == r.rule.lesser && *b == r.rule.greater {
                            return Ordering::Less;
                        } else if *b == r.rule.lesser && *a == r.rule.greater {
                            return Ordering::Greater;
                        }
                    }
                    Ordering::Equal
                });
                updated
            })
            .collect::<Vec<Update>>(),
        _ => panic!("uknown part"),
    };

    Ok(valid_updates
        .iter()
        .map(|u: &Update| {
            let update = u;
            if update.len() % 2 == 0 {
                panic!("what even is the middle?")
            }
            update.index(update.len() / 2)
        })
        .sum())
}
