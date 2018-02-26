#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board_no_winners() {
        let mut ttt = TicTacToe::new();

        for row in 0 .. 3 {
            assert_eq!(true, ttt.can_win_down_diag(Piece::None));
            assert_eq!(false, ttt.can_win_down_diag(Piece::X));
            assert_eq!(false, ttt.can_win_down_diag(Piece::O));

            assert_eq!(true, ttt.can_win_up_diag(Piece::None));
            assert_eq!(false, ttt.can_win_up_diag(Piece::X));
            assert_eq!(false, ttt.can_win_up_diag(Piece::O));
        }

        for index in 0 .. 9 {
            assert_eq!(true, ttt.has_piece(Piece::None, index));
            assert_eq!(false, ttt.has_piece(Piece::X, index));
            assert_eq!(false, ttt.has_piece(Piece::O, index));

            assert_eq!(false, ttt.can_win(Piece::None, index));
            assert_eq!(false, ttt.can_win(Piece::X, index));
            assert_eq!(false, ttt.can_win(Piece::O, index));
        }
    }

    #[test]
    fn one_piece_no_winners() {
        let mut ttt = TicTacToe::new();
        ttt.set_piece(0, Piece::X);

        for row in 0 .. 3 {
            assert_eq!(false, ttt.can_win_down_diag(Piece::None));
            assert_eq!(false, ttt.can_win_down_diag(Piece::X));
            assert_eq!(false, ttt.can_win_down_diag(Piece::O));

            assert_eq!(true, ttt.can_win_up_diag(Piece::None));
            assert_eq!(false, ttt.can_win_up_diag(Piece::X));
            assert_eq!(false, ttt.can_win_up_diag(Piece::O));
        }

        assert_eq!(false, ttt.has_piece(Piece::None, 0));
        assert_eq!(true, ttt.has_piece(Piece::X, 0));
        assert_eq!(false, ttt.has_piece(Piece::O, 0));

        for index in 1 .. 9 {
            assert_eq!(true, ttt.has_piece(Piece::None, index));
            assert_eq!(false, ttt.has_piece(Piece::X, index));
            assert_eq!(false, ttt.has_piece(Piece::O, index));

            assert_eq!(false, ttt.can_win(Piece::None, index));
            assert_eq!(false, ttt.can_win(Piece::X, index));
            assert_eq!(false, ttt.can_win(Piece::O, index));
        }
    }
}

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct TicTacToeGameMap {
    id: usize,
    games: HashMap<usize, TicTacToe>
}

impl TicTacToeGameMap {
    pub fn new() -> TicTacToeGameMap {
        TicTacToeGameMap {
            id: 0,
            games: HashMap::new()
        }
    }

    pub fn move_game(&mut self, id: usize, spot: usize) -> String {
        self.games.get_mut(&id).map(|b| {
            b.player_move(spot);
            Self::to_json(id, &*b)
        }).unwrap_or("{}".to_string())
    }

    fn to_json(id: usize, game: &TicTacToe) -> String {
        match game.did_win() {
            Piece::O => format!("{{id: {}, game: \"{}\", status: \"You won!\"}}", id, game),
            Piece::X => format!("{{id: {}, game: \"{}\", status: \"You lost!\"}}", id, game),
            _ => format!("{{id: {}, game: \"{}\", status: \"Your move.\"}}", id, game),
        }
    }

    pub fn finish_game(&mut self, id: usize) -> String {
        self.games.remove(&id);
        String::from("")
    }

    pub fn create_game(&mut self) -> String {
        self.id += 1;
        let b = TicTacToe::new();
        let entry = self.games.entry(self.id);
        Self::to_json(self.id, entry.or_insert(b))
    }

}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Piece {
    None,
    X,
    O,
}

const COMPUTER_PIECE: Piece = Piece::X;
const PLAYER_PIECE: Piece = Piece::O;

#[derive(Debug)]
pub struct TicTacToe {
    board: [Piece; 9]
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "[{}]", self.board.iter().fold(String::new(), |acc, &p| {
            match p {
                Piece::X => acc + "X , ",
                Piece::O => acc + "O , ",
                Piece::None => acc + " , ",
            }
        }))
    }
}

impl TicTacToe {
    pub fn new() -> Self {

        Self {
            board: [Piece::None; 9]
        }
    }

    fn player_move(&mut self, index: usize) {
        if self.board[index] == Piece::None {
            self.board[index] = PLAYER_PIECE;
            println!("player moved at {:?}", index);
            self.computer_move();
        }
    }

    fn computer_move(&mut self) {
        for index in 0 .. 9 {
            if self.can_win(COMPUTER_PIECE, index) {
                self.board[index] = COMPUTER_PIECE;
                return;
            }
        }
        println!("computer couldn't win");
        for index in 0 .. 9 {
            if self.can_win(PLAYER_PIECE, index) {
                self.board[index] = COMPUTER_PIECE;
                println!("computer went at {:?}, {:?}", index, self.board);
                return;
            }
        }
        println!("computer couldn't block");
        if self.corner_is_open() {
            self.go_in_arbitrary_corner(COMPUTER_PIECE);
            return;
        }
        println!("computer couldn't go in corner");
        if self.board[4] == Piece::None {
            self.board[4] = COMPUTER_PIECE;
            return;
        }
        println!("computer couldn't go in middle");
        self.go_in_first_empty();
    }

    fn go_in_first_empty(&mut self) {
        for index in 1 .. 9 {
            if self.board[index] == Piece::None {
                self.board[index] = COMPUTER_PIECE;
                return;
            }
        }
    }

    fn corner_is_open(&self) -> bool {
        self.board[0] == Piece::None || self.board[2] == Piece::None || self.board[6] == Piece::None || self.board[8] == Piece::None
    }

    fn go_in_arbitrary_corner(&mut self, my_piece: Piece) {
        if self.board[0] == Piece::None {
            println!("computer went in NW corner, {:?}", self.board);
            self.board[0] = my_piece;
            println!("computer went in NW corner, {:?}", self.board);
        }
        else if self.board[2] == Piece::None {
            self.board[2] = my_piece;
            println!("computer went in NE corner");
        }
        else if self.board[6] == Piece::None {
            self.board[6] = my_piece;
            println!("computer went in SW corner");
        }
        else if self.board[8] == Piece::None {
            self.board[8] = my_piece;
            println!("computer went in SE corner");
        }
    }

    fn can_win(&self, my_piece: Piece, index: usize) -> bool {
        if my_piece == Piece::None || self.board[index] != Piece::None {
            return false;
        }

        let row_can_win = match index % 3 { // the column
            0 => self.board[index + 1] == my_piece && self.board[index + 2] == my_piece,
            1 => self.board[index - 1] == my_piece && self.board[index + 1] == my_piece,
            _ => self.board[index - 2] == my_piece && self.board[index - 1] == my_piece,
        };

        let col_can_win = match index / 3 { // the row
            0 => self.board[index + 3] == my_piece && self.board[index + 6] == my_piece,
            1 => self.board[index - 3] == my_piece && self.board[index + 3] == my_piece,
            _ => self.board[index - 6] == my_piece && self.board[index - 3] == my_piece,
        };

        let index_on_down_diag = index == 0 || index == 4 || index == 8;
        let index_on_up_diag = index == 6 || index == 4 || index == 2;

        row_can_win || col_can_win || 
            (index_on_down_diag && self.can_win_down_diag(my_piece)) || 
            (index_on_up_diag && self.can_win_up_diag(my_piece))
    }

    fn three_in_a_row(&self, index_1: usize, index_2: usize, index_3: usize) -> bool {
        Piece::None != self.board[index_1] && self.board[index_1] == self.board[index_2] && self.board[index_2] == self.board[index_3]
    }

    fn did_win(&self) -> Piece {
        if self.three_in_a_row(0, 1, 2) || self.three_in_a_row(0, 3, 6) || self.three_in_a_row(0, 4, 8) {
            self.board[0]
        }
        else if self.three_in_a_row(3, 4, 5) {
            self.board[3]
        }
        else if self.three_in_a_row(6, 7, 8) {
            self.board[6]
        }
        else if self.three_in_a_row(1, 4, 7) {
            self.board[1]
        }
        else if self.three_in_a_row(2, 5, 8) || self.three_in_a_row(6, 4, 2) {
            self.board[2]
        }
        else {
            Piece::None
        }
    }

    fn set_piece(&mut self, index: usize, piece: Piece) {
        self.board[index] = piece;
    }

    fn has_piece(&self, piece: Piece, index: usize) -> bool {
        self.board[index] == piece
    }

    fn can_win_down_diag(&self, my_piece: Piece) -> bool {
        let diag = &[self.board[0], self.board[4], self.board[8]];
        diag == &[Piece::None, my_piece, my_piece] ||
        diag == &[my_piece, Piece::None, my_piece] ||
        diag == &[my_piece, my_piece, Piece::None]
    }

    fn can_win_up_diag(&self, my_piece: Piece) -> bool {
        let diag = &[self.board[2], self.board[4], self.board[6]];
        diag == &[Piece::None, my_piece, my_piece] ||
        diag == &[my_piece, Piece::None, my_piece] ||
        diag == &[my_piece, my_piece, Piece::None]
    }
}
