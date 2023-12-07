use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, BufRead};

type Card = (String, u32);

const FIVE_OF_A_KIND: u32 = 7;
const FOUR_OF_A_KIND: u32 = 6;
const FULL_HOUSE: u32 = 5;
const THREE_OF_A_KIND: u32 = 4;
const TWO_PAIR: u32 = 3;
const ONE_PAIR: u32 = 2;
const HIGH_CARD: u32 = 1;

fn main() -> Result<()> {
    let mut cards: Vec<Card> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split_whitespace().collect();

        cards.push((tokens[0].to_owned(), tokens[1].parse()?));
    }

    cards.sort_unstable_by(|a, b| rank(&a.0, &b.0, false));

    println!(
        "Part 1: {}",
        cards
            .iter()
            .enumerate()
            .map(|(idx, c)| (idx as u32 + 1) * c.1)
            .sum::<u32>()
    );

    cards.sort_unstable_by(|a, b| rank(&a.0, &b.0, true));

    println!(
        "Part 2: {}",
        cards
            .iter()
            .enumerate()
            .map(|(idx, c)| (idx as u32 + 1) * c.1)
            .sum::<u32>()
    );

    Ok(())
}

fn rank(cards0: &str, cards1: &str, part2: bool) -> std::cmp::Ordering {
    let c0_score = score_type(cards0, part2);
    let c1_score = score_type(cards1, part2);

    if c0_score < c1_score {
        return std::cmp::Ordering::Less;
    } else if c0_score > c1_score {
        return std::cmp::Ordering::Greater;
    }

    for (c0, c1) in std::iter::zip(cards0.chars(), cards1.chars()) {
        if score_card(c0, part2) < score_card(c1, part2) {
            return std::cmp::Ordering::Less;
        } else if score_card(c0, part2) > score_card(c1, part2) {
            return std::cmp::Ordering::Greater;
        }
    }

    std::cmp::Ordering::Equal
}

fn score_type(card: &str, part2: bool) -> u32 {
    let mut counts_map = HashMap::new();

    for c in card.chars() {
        *counts_map.entry(c).or_insert(0) += 1;
    }

    let mut counts: Vec<_> = counts_map.values().collect();

    counts.sort();

    let mut score = match &counts[..] {
        [5] => FIVE_OF_A_KIND,
        [1, 4] => FOUR_OF_A_KIND,
        [2, 3] => FULL_HOUSE,
        [1, 1, 3] => THREE_OF_A_KIND,
        [1, 2, 2] => TWO_PAIR,
        [1, 1, 1, 2] => ONE_PAIR,
        [1, 1, 1, 1, 1] => HIGH_CARD,
        _ => unreachable!(),
    };

    if part2 {
        let j_count = card.chars().filter(|&c| c == 'J').count();

        score = match (score, j_count) {
            (FOUR_OF_A_KIND, 1 | 4) => FIVE_OF_A_KIND,
            (FULL_HOUSE, 2 | 3) => FIVE_OF_A_KIND,
            (THREE_OF_A_KIND, 1 | 3) => FOUR_OF_A_KIND,
            (TWO_PAIR, 1) => FULL_HOUSE,
            (TWO_PAIR, 2) => FOUR_OF_A_KIND,
            (ONE_PAIR, 1 | 2) => THREE_OF_A_KIND,
            (HIGH_CARD, 1) => ONE_PAIR,
            _ => score,
        };
    }

    score
}

fn score_card(c: char, part2: bool) -> u32 {
    match (c, part2) {
        ('A', _) => 14,
        ('K', _) => 13,
        ('Q', _) => 12,
        ('J', false) => 11,
        ('J', true) => 1,
        ('T', _) => 10,
        ('2'..='9', _) => c.to_digit(10).unwrap(),
        _ => unreachable!(),
    }
}
