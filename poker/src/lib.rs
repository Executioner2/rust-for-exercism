use std::collections::BinaryHeap;
use crate::PokerType::*;

macro_rules! ternary {
    ($condition:expr, $t:expr, $f:expr) => {
        if $condition { $t } else { $f }
    };
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut poker_hands: BinaryHeap<_> = hands.iter().map(|&hand| parse_hand(hand)).collect();
    let (prev, s) = poker_hands.pop().unwrap();
    let mut res = vec![s];
    while let Some((poker, str)) = poker_hands.pop() {
        if poker < prev {
            break;
        }
        res.push(str);
    }
    res
}

fn parse_hand(hand: &str) -> Poker {
    let cards = hand
        .split(" ")
        .map(|card| {
            let (rank, suit) = card.split_at(card.len() - 1);
            let rank = match rank.parse() { 
                Ok(rank) => rank,
                Err(_) => "JQKA".find(rank).unwrap() as u8 + 11,
            };
            (suit.chars().next().unwrap().to_ascii_uppercase(), rank)
        })
        .collect::<Vec<Card>>();

    // 解析手牌类型
    parse_poker(cards.try_into().unwrap(), hand)
}

fn parse_poker(mut cards: [Card; 5], hand: &str) -> Poker {
    cards.sort_unstable_by_key(|card| card.1);
    if (2, 3, 4, 5, 14) == (cards[0].1, cards[1].1, cards[2].1, cards[3].1, cards[4].1) {
        cards[4].1 = 1;
        cards.sort_unstable_by_key(|card| card.1);
    }

    // 花色是否相同
    let is_some_suit = cards.iter().skip(1).all(|card| card.0 == cards[0].0);

    // 是否是顺子
    let is_straight = cards
        .iter()
        .skip(1)
        .enumerate()
        .all(|(i, card)| card.1 == cards[0].1 + 1 + i as u8);

    // 是否成对
    let mut pair = Vec::with_capacity(4);
    for i in (0..4).rev() {
        if cards[i + 1].1 == cards[i].1 {
            pair.push(cards[i].1)
        }
    }

    let poker_type = match (is_some_suit, is_straight, pair.len()) {
        (true, true, _) => StraightFlush(cards[4].1),
        (_, _, 3) if pair[0] == pair[2] => FourOfAKind(pair[0], ternary!(pair[0] == cards[0].1, cards[4].1, cards[0].1),),
        (_, _, 3) => FullHouse(ternary!(pair[0] == pair[2], pair[0], pair[2]), ternary!(pair[0] == pair[2], pair[2], pair[0])),
        (true, _, _) => Flush(cards[4].1, cards[3].1, cards[2].1, cards[1].1, cards[0].1),
        (_, true, _) => Straight(cards[4].1),
        (_, _, 2) if pair[0] == pair[1] => ThreeOfAKind(cards[4].1, cards[1].1, cards[0].1),
        (_, _, 2) => TwoPair(pair[0], pair[1], cards[4].1, cards[3].1, cards[2].1, cards[1].1, cards[0].1,),
        (_, _, 1) => OnePair(pair[0], cards[4].1, cards[3].1, cards[2].1, cards[1].1, cards[0].1,),
        _ => HighCard(cards[4].1, cards[3].1, cards[2].1, cards[1].1, cards[0].1),
    };

    (poker_type, hand)
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
enum PokerType {
    HighCard(u8, u8, u8, u8, u8),        // 高牌
    OnePair(u8, u8, u8, u8, u8, u8),     // 一对
    TwoPair(u8, u8, u8, u8, u8, u8, u8), // 两对
    ThreeOfAKind(u8, u8, u8),            // 三条
    Straight(u8),                        // 顺子
    Flush(u8, u8, u8, u8, u8),           // 同花
    FullHouse(u8, u8),                   // 葫芦
    FourOfAKind(u8, u8),                 // 四条
    StraightFlush(u8),                   // 同花顺
}

/// (花色，点数)
type Card = (char, u8);
type Poker<'a> = (PokerType, &'a str);
