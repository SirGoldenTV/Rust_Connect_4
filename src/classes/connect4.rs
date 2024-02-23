#[derive(Debug)] 
pub struct Connect4 {
    player : u8,
    grid : [[u8;7];6],
    pub finish : bool,
    message : String
}

impl Connect4 {

    pub fn new() -> Self {
        let player = 1;
        let mut grid = [[0;7];6];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                grid[i][j] = 0;
            }
        }
        let finish = false;
        let message = String::new();
        Connect4 {player, grid, finish, message}

    }

    /// Cette fonction permet d'afficher la grille durant la partie.
    pub fn show(&self) {
        for i in self.grid.iter() {
            for j in i.iter() {
                print!("{} ", j);
            }
            println!();
        }
    }

    /// Cette fonction vérifie si une ligne contient 4 pions à la suite d'une même couleur.
    ///
    /// # Arguments
    ///
    /// * `_line` - La ligne à vérifier.
    fn check_win_line(&self, _line : u8) -> bool {
        return self.grid[usize::from(_line)].windows(4).any(|window| {
            window.iter().all(|&num| num == window[0] && num != 0)
        });
    }

    /// Cette fonction vérifie si une colonne contient 4 pions à la suite d'une même couleur.
    ///
    /// # Arguments
    ///
    /// * `_column` - La colonne à vérifier.
    fn check_win_column(&self, _column : u8) -> bool {
        return self.grid.windows(4).any(|window| {
            window.iter().all(|row| row[usize::from(_column)] == window[0][usize::from(_column)] && row[usize::from(_column)] != 0 )
        });
    }

    /// Cette fonction vérifie si une diagonale descendante contient 4 pions à la suite de la même couleur.
    ///
    /// # Arguments
    ///
    /// * `_start_row` - L'indice de la ligne de départ.
    /// * `_start_col` - L'indice de la colonne de départ.
    fn check_win_diagonal_desc_left(&self, _start_row: usize, _start_col: usize) -> bool {
        let mut diagonal = Vec::with_capacity(4);
        
        for _i in 0..4 {
            let row = _start_row.checked_add(_i);
            let col = _start_col.checked_sub(_i);
            
            if let (Some(r), Some(c)) = (row, col) { // Permet de vérifier que row et col sont pas à None.
                if r > self.grid.len() - 1 || c > self.grid[0].len() - 1 {
                    return false
                }
                diagonal.push(self.grid[r][c]);
                
            } else {
                return false; // Hors limites de la grille
            }
        }
        
        if diagonal.iter().count() < 4 
        {
            return false;
        }
        diagonal.iter().all(|&num| num == diagonal[0] && num != 0)
    }

    /// Cette fonction vérifie si une diagonale descendante contient 4 pions à la suite de la même couleur.
    ///
    /// # Arguments
    ///
    /// * `_start_row` - L'indice de la ligne de départ.
    /// * `_start_col` - L'indice de la colonne de départ.
    fn check_win_diagonal_desc_right(&self, _start_row: usize, _start_col: usize) -> bool {
        let mut diagonal = Vec::with_capacity(4);
        
        for _i in 0..4 {
            let row = _start_row.checked_add(_i);
            let col = _start_col.checked_add(_i);
            
            if let (Some(r), Some(c)) = (row, col) { // Permet de vérifier que row et col sont pas à None.
                if r > self.grid.len() - 1 || c > self.grid[0].len() - 1 {
                    return false
                }
                diagonal.push(self.grid[r][c]);
                
            } else {
                return false; // Hors limites de la grille
            }
        }
        
        if diagonal.iter().count() < 4 
        {
            return false;
        }
        diagonal.iter().all(|&num| num == diagonal[0] && num != 0)
    }

    /// Cette fonction vérifie si une colonne est complète.
    ///
    /// # Arguments
    ///
    /// * `_col` - La colonne à vérifier.
    fn check_column_full(&self, _col : usize) -> bool {
        self.grid[0][_col] != 0
    }

    /// Cette fonction vérifie la grille pour savoir si elle possède 4 pions à la suite d'une même couleur.
    /// Elle vérifie les colonnes, les lignes et les diagonales.
    fn check_grid(&self) -> bool
    {
        // Return true si on trouve une ligne ou une colonne qui possède 4 de suite.
        for i in 0..self.grid.len() {
            if self.check_win_line(i as u8) || self.check_win_column(i as u8) {
                return true;
            }
        }

        // Return true si on trouve une diagonale qui possède 4 de suite.
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {                
                if self.check_win_diagonal_desc_left(i,j)
                || self.check_win_diagonal_desc_right(i,j) {
                    return true;
                }
            }
        
        }
        return false;
    }

    /// Cette fonction applique un coup à la grille effectué par le joueur.
    ///
    /// # Arguments
    ///
    /// * `_col` - La colonne à compléter avec le coup du joueur.
    fn apply_play(&mut self, _col : u8) -> bool {
        let _attempt : usize = usize::from(_col);

        if !self.check_column_full(_attempt) {
            let last_index = self.get_last_index(_attempt); // Permet d'obtenir le dernier élément vide dans la grille.
            self.grid[last_index][_attempt] = self.player;
            return true;
        } else { self.column_full_message() }
        false
    }

    /// Cette fonction permet d'obtenir le dernier élément vide pour une colonne.
    ///
    /// # Arguments
    ///
    /// * `_col` - La colonne à parcourir.
    fn get_last_index(&self, _col : usize) -> usize {
        for (index_line, line) in self.grid.iter().enumerate() {
            if line[_col] != 0 
                { return index_line.checked_sub(1).unwrap(); } // -1 car on veut le précédent du coup.
        }
        self.grid.len() - 1
    }

    /// Cette fonction permet au joueur actif de jouer un coup.
    /// Elle vérifie également si la partie est terminée.
    /// # Arguments
    ///
    /// * `_col` - La colonne à compléter avec le coup du joueur.
    pub fn play(&mut self, _col : u8) {
        if self.finish {
            self.game_finished_message();
            return;
        }

        if usize::from(_col) > self.grid[0].len() {
            self.message.clear();
            self.message.push_str("Must be in the array.");
            println!("{}", self.message);
            return;
        }

        if self.apply_play(_col) {
            self.attempt_turn_message(); 
        } else {
            return;
        }
            
        
        if self.check_grid() {
            self.win_message();
            self.finish = true;
            self.show();
            println!(); // faire de l'espace pour une autre grille.
            return;
        }

        self.change_player();
    }

    /// Cette fonction permet de changer de joueur actif entre 1 et 2.
    fn change_player(&mut self) {
        if self.player == 1 {
            self.player += 1;
        }
        else {
            self.player -= 1;
        } 
    }

    // Section Messages
    /// Cette fonction affiche un message et modifie celui de la partie pour indiquer qu'un joueur à jouer.
    fn attempt_turn_message(&mut self) {
        self.message.clear();
        self.message.push_str("Player ");
        self.message.push_str(&self.player.to_string());
        self.message.push_str(" has a turn");
        println!("{}",self.message);
    }

    /// Cette fonction affiche un message et modifie celui de la partie pour indiquer qu'un joueur a gagné.
    fn win_message(&mut self) {
        self.message.clear();
        self.message.push_str("Player ");
        self.message.push_str(&self.player.to_string());
        self.message.push_str(" wins!");
        println!("{}",self.message);
    }

    /// Cette fonction affiche un message et modifie celui de la partie pour indiquer que la partie est finie.
    fn game_finished_message(&mut self) {
        self.message.clear();
        self.message.push_str("Game has finished!");
        println!("{}",self.message);
    }

    /// Cette fonction affiche un message et modifie celui de la partie pour indiquer qu'une colonne est complète.
    fn column_full_message(&mut self) {
        self.message.clear();
        self.message.push_str("Column full!");
        println!("{}",self.message);
    }

    // 
}