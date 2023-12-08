#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        return Self {
            scores: scores.to_vec(),
        };
    }

    pub fn scores(&self) -> &[u32] {
        return &self.scores;
    }

    pub fn latest(&self) -> Option<u32> {
        return self.scores.last().copied();
    }

    pub fn personal_best(&self) -> Option<u32> {
        return self.scores.iter().max().copied();
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut temp_scores = self.scores.clone();
        temp_scores.sort();
        temp_scores.reverse();
        if temp_scores.len() > 3 {
            return temp_scores.as_slice()[..3].to_vec();
        }else {
            return temp_scores
        }
    }

    // pub fn personal_top_three(&self) -> Vec<u32> {
    //     let mut top_three = self.scores.to_vec();
    //     top_three.sort_by(|l, r|r.cmp(l));
    //     top_three.truncate(3);
    //     top_three
    // }
}
