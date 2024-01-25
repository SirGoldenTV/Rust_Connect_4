pub struct Connect4 {
    grid : [[u8;7];6]
}

impl Connect4 {

    pub fn new() -> Self {
        // TODO
        let grid = [[0;7];6];

        Connect4 {grid}

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
}