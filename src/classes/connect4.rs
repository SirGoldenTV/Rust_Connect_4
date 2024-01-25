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

    pub fn check_line(&self, _line : u8) {
        println!("Pas implémenté.");
    }

    pub fn check_column(&self, _column : u8) {
        println!("Pas implémenté.");
    }

    pub fn check_diagonal(&self) {
        println!("Pas implémenté.");
    }

    pub fn play(&self, _col : u8) {
        let _player_active = self.player;
    }

    pub fn change_player(&mut self) {
        if self.player == 1 {
            self.player += 1;
        }
        else {
            self.player -= 1;
        } 
    }
}