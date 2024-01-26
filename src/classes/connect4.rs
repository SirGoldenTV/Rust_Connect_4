pub struct Connect4 {
    player : u8,
    grid : [[u8;7];6]
}

impl Connect4 {

    pub fn new() -> Self {
        let player = 1;
        let grid = [[0;7];6];

        Connect4 {player, grid}

    }

    pub fn show(&self) {
        for i in self.grid.iter() {
            for j in i.iter() {
                print!("{} ", j);
            }
            println!();
        }
    }

    pub fn check_win_line(&self, _line : u8) -> bool {
        return self.grid[usize::from(_line)].windows(4).any(|window| {
            window.iter().all(|&num| num == window[0] && num != 0)
        });
    }

    pub fn check_win_column(&self, _column : u8) -> bool {
        return self.grid.windows(4).any(|window| {
            window.iter().all(|row| row[usize::from(_column)] == window[0][usize::from(_column)] && row[usize::from(_column)] != 0 )
        });
    }

    pub fn check_win_diagonal(&self) {
        println!("Pas implémenté.");
    }

    pub fn check_column_full(&self, _col : u8) -> bool {
        self.grid[0][usize::from(_col)] == 0
    }

    pub fn play(&mut self, _col : u8, _string : &mut String) {
        if usize::from(_col) > self.grid[0].len() {
            _string.clear();
            _string.push_str("Must be in the array.");
            return;
        }

        // let _player_active = self.player;
        // let _column_full_message = String::from("Column full!");
        // let _game_finished_message = String::from("Game has finished!");

        self.attempt_turn(_string);
        // self.win_message(&_win_message);

        self.show();
        self.change_player();
    }

    pub fn change_player(&mut self) {
        if self.player == 1 {
            self.player += 1;
        }
        else {
            self.player -= 1;
        } 
    }

    fn attempt_turn(&self, _str : &mut String) {
        _str.clear();
        _str.push_str("Player ");
        _str.push_str(&self.player.to_string());
        _str.push_str(" has a turn");
    }

    fn win_message(&self, mut _str : &mut String) {
        _str.clear();
        _str.push_str("Player ");
        _str.push_str(&self.player.to_string());
        _str.push_str(" wins!");
    }
}