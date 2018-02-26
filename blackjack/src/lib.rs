#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate rand;

use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameMap {
    id: usize,
    games: HashMap<usize, Blackjack>
}

impl GameMap {
    pub fn new() -> GameMap {
        GameMap {
            id: 0,
            games: HashMap::new()
        }
    }

    pub fn hit_game(&mut self, id: usize) -> String {
        self.games.get_mut(&id).map(|b| {
            b.player_hit();
            Self::to_json(id, &*b)
        }).unwrap_or("{}".to_string())
    }

    pub fn stand_game(&mut self, id: usize) -> String {
        self.games.get_mut(&id).map(|b| {
            b.player_stand();
            Self::to_json(id, &*b)
        }).unwrap_or("{}".to_string())
    }

    fn to_json(id: usize, game: &Blackjack) -> String {
        if !game.is_player_done() {
            format!("{{id: {}, player: \"{}\"}}", id, game.player_hand_print())
        } else {
            format!("{{id: {}, player: \"{}\", dealer: \"{}\", result: \"{}\"}}", 
                id,
                game.player_hand_print(),
                game.dealer_hand_print(),
                if game.is_player_busted() {
                    "busted"
                } else if game.player_points() == 21 {
                    "blackjack"
                } else if game.is_dealer_busted() || game.player_points() > game.dealer_points() {
                    "win"
                } else if game.player_points() == game.dealer_points() {
                    "push"
                } else {
                    "lose"
                })
        }
    }

    pub fn finish_game(&mut self, id: usize) -> String {
        self.games.remove(&id);
        String::from("")
    }

    pub fn create_game(&mut self) -> String {
        self.id += 1;
        let b = Blackjack::new();
        let entry = self.games.entry(self.id);
        Self::to_json(self.id, entry.or_insert(b))
    }

}

#[derive(Debug)]
pub struct Blackjack {
    deck: Vec<u8>,
    player_cards: Vec<u8>,
    dealer_cards: Vec<u8>
}

impl Blackjack {
    pub fn new() -> Blackjack {

        let mut rng = rand::thread_rng();
        let mut deck: Vec<u8> = (0u8..52u8).collect();
        rng.shuffle(&mut deck);

        let mut b = Blackjack {
            deck: deck,
            player_cards: Vec::new(),
            dealer_cards: Vec::new()
        };
        b.dealer_hit();
        b.player_hit();
        b.player_hit();
        b
    }

    pub fn player_hand_print(&self) -> String {
        Self::hand_print(&self.player_cards)
    }

    pub fn dealer_hand_print(&self) -> String {
        Self::hand_print(&self.dealer_cards)
    }

    fn hand_print(nums: &Vec<u8>) -> String {
        let string_vec: Vec<String> = nums.iter().map(|n| Self::to_string(*n)).collect();
        string_vec.join(", ")
    }

    pub fn player_stand(&mut self) {
        if !self.is_player_busted() {
            while self.dealer_points() <= 16 {
                self.dealer_hit();
            }
        }
    }

    pub fn player_hit(&mut self) {
        if !self.is_player_done() {
            self.player_cards.push(self.deck.pop().unwrap());
        }
    }

    pub fn dealer_hit(&mut self) {
        self.dealer_cards.push(self.deck.pop().unwrap());
    }

    pub fn is_player_done(&self) -> bool {
        self.player_points() >= 21 || self.is_dealer_done()
    }

    pub fn is_dealer_done(&self) -> bool {
        self.dealer_points() > 16
    }

    pub fn is_player_busted(&self) -> bool {
        self.player_points() > 21
    }

    pub fn is_dealer_busted(&self) -> bool {
        self.dealer_points() > 21
    }

    pub fn player_points(&self) -> u8 {
        Self::sum_points(&self.player_cards)
    }

    pub fn dealer_points(&self) -> u8 {
        Self::sum_points(&self.dealer_cards)
    }

    fn sum_points(nums: &Vec<u8>) -> u8 {
        let mut found_ace = false;
        let mut sum = 0u8;
        for c in nums.iter() {
            let card_value = (c % 13) + 1;
            sum += 
                match card_value {
                    1 => {
                        found_ace = true;
                        1
                    },
                    x if x < 10 => x,
                    _ => 10,
                };
        }
        if found_ace && sum + 10 <= 21 {
            sum + 10
        } else {
            sum
        }
    }

    fn to_string(num: u8) -> String {
        let suit = num / 13;
        let suit_str = match suit {
            0 => "S",
            1 => "D",
            2 => "C",
            _ => "H",
        };
        let rank = num % 13;
        let rank_str = match rank {
            0 => "A".to_string(),
            r if r < 10 => (r + 1).to_string(),
            10 => "J".to_string(),
            11 => "Q".to_string(),
            _ => "K".to_string(),
        };
        rank_str + suit_str
    }
}
