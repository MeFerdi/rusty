#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }

    pub fn read_winner(&self) -> (String, u16) {
        let (p1_name, p1_score) = &self.p1;
        let (p2_name, p2_score) = &self.p2;

        match p1_score.cmp(p2_score) {
            std::cmp::Ordering::Greater => (p1_name.clone(), *p1_score),
            std::cmp::Ordering::Less => (p2_name.clone(), *p2_score),
            std::cmp::Ordering::Equal => (String::from("Same score! tied"), *p1_score),
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        if self.is_game_finished() {
            return;
        }

        if user_name == self.p1.0 {
            self.p1.1 += 1;
        } else if user_name == self.p2.0 {
            self.p2.1 += 1;
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }

    fn is_game_finished(&self) -> bool {
        let required_wins = (self.nb_games / 2) + 1;
        self.p1.1 >= required_wins || self.p2.1 >= required_wins
    }
}