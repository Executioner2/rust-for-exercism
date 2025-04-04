#[derive(Debug)]
pub struct HighScores {
    top: Vec<u32>,
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let scores = scores.to_vec();
        let mut top = Vec::new();

        for &x in scores.iter() {
            top.push(x);
            top.sort_by(|a, b| b.cmp(a));
            if top.len() > 3 {
                top.pop();
            }
        }

        Self { top, scores }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.top.first().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top.to_vec()
    }
}
