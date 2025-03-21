use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const MAX_FRAMES: usize = 10;
const MAX_PINS: u16 = 10;

enum RoundState {
    Normal(u16, u16),
    Spare(u16),
    Strike(u16),
}

impl RoundState {
    fn is_spare(&self) -> bool {
        matches!(self, RoundState::Spare(_))
    }

    fn is_strike(&self) -> bool {
        matches!(self, RoundState::Strike(_))
    }

    fn first(&self) -> u16 {
        match self {
            RoundState::Normal(first, _) => *first,
            RoundState::Spare(first) => *first,
            RoundState::Strike(first) => *first,
        }
    }
}

pub struct BowlingGame {
    pitch: Option<u16>,
    score: Option<u16>,
    rounds: Vec<RoundState>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            pitch: None,
            score: None,
            rounds: vec![],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.score.is_some() {
            return Err(Error::GameComplete);
        }
        if pins > MAX_PINS {
            return Err(Error::NotEnoughPinsLeft);
        }

        match self.pitch.take() {
            None => {
                if pins == MAX_PINS {
                    self.rounds.push(RoundState::Strike(pins))
                } else {
                    self.pitch = Some(pins);
                }
            }
            Some(prev) => {
                let sum = prev + pins;
                let round = match sum.cmp(&MAX_PINS) {
                    Ordering::Equal => RoundState::Spare(prev),
                    Ordering::Greater => return Err(Error::NotEnoughPinsLeft),
                    Ordering::Less => RoundState::Normal(prev, pins),
                };
                self.rounds.push(round);
            }
        }

        self.calculate_score();

        Ok(())
    }

    fn calculate_score(&mut self) {
        if self.rounds.len() == MAX_FRAMES && self.rounds[MAX_FRAMES - 1].is_spare() {
            if let Some(pins) = self.pitch.take() {
                self.rounds.push(RoundState::Normal(pins, 0))
            }
        }

        let n = self.rounds.len();
        if n < MAX_FRAMES || match self.rounds[MAX_FRAMES - 1] {
            RoundState::Normal(_, _) => false,
            RoundState::Spare(_) => n < MAX_FRAMES + 1,
            RoundState::Strike(_) => {
                n < MAX_FRAMES + 1 || (self.rounds[MAX_FRAMES].is_strike() && n < MAX_FRAMES + 2)
            }
        } {
            return;
        }

        let mut score = 0;
        for (i, round) in self.rounds.iter().rev().skip(n - MAX_FRAMES).enumerate() {
            let i = MAX_FRAMES - i - 1;
            score += match round {
                RoundState::Normal(first, second) => first + second,
                RoundState::Spare(_) => MAX_PINS + self.rounds[i + 1].first(),
                RoundState::Strike(_) => {
                    MAX_PINS + match self.rounds[i + 1] {
                        RoundState::Normal(first, second) => first + second,
                        RoundState::Spare(_) => MAX_PINS,
                        RoundState::Strike(_) => MAX_PINS + self.rounds[i + 2].first(),
                    }
                }
            }
        }

        self.score = Some(score);
    }

    pub fn score(&self) -> Option<u16> {
        self.score
    }
}
