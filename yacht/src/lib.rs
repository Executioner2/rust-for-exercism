#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(mut dice: Dice, category: Category) -> u8 {
    dice.sort();
    match category {
        Category::Ones => dice.iter().filter(|&x| *x == 1).count() as u8,
        Category::Twos => dice.iter().filter(|&x| *x == 2).count() as u8 * 2,
        Category::Threes => dice.iter().filter(|&x| *x == 3).count() as u8 * 3,
        Category::Fours => dice.iter().filter(|&x| *x == 4).count() as u8 * 4,
        Category::Fives => dice.iter().filter(|&x| *x == 5).count() as u8 * 5,
        Category::Sixes => dice.iter().filter(|&x| *x == 6).count() as u8 * 6,
        Category::FullHouse => {
            if dice[0] == dice[2] && dice[3] == dice[4] && dice[2] != dice[3] {
                dice[0] * 3 + dice[3] * 2
            } else if dice[0] == dice[1] && dice[2] == dice[4] && dice[1] != dice[2] {
                dice[0] * 2 + dice[2] * 3
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            if dice[0] == dice[3] {
                dice[0] * 4
            } else if dice[1] == dice[4] {
                dice[1] * 4
            } else {
                0
            }
        }
        Category::LittleStraight => {
            if [1, 2, 3, 4, 5] == dice {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if [2, 3, 4, 5, 6] == dice {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if dice[0] == dice[4] {
                50
            } else {
                0
            }
        }
    }
}
