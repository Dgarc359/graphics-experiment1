pub mod game {

    // TODO: private vs public methods in private module

    use std::collections::HashMap;

    pub trait ExternalMinesweeper {
        fn get_width(&self) -> usize;
        fn get_height(&self) -> usize;
        /// Toggle the presence of a flag on a tile.
        fn toggle_flag(&mut self, x: usize, y: usize);
        /// Explore a tile. If it was a mine, BOOM! If not, flood fill mine scouting out from it.
        ///
        /// Returns true if BOOM!
        fn explore_tile(&mut self, x: usize, y: usize) -> bool;
        /// Return whether a tile should be displayed as blank, a flag, a cleared mine, a mine count 1-8, or a kaboom.
        fn get_tile(&self, x: usize, y: usize) -> Option<&MapTile>;

        fn set_mine(&mut self, x: usize, y: usize) -> Option<MapTile>;
    }

    pub struct MapTile {
        pub x: usize,
        pub y: usize,
        pub has_flag: bool,
        pub has_mine: bool,
    }

    pub struct GameState {
        pub map: HashMap<String, MapTile>,
    }

    pub struct Game {
        width: usize,
        height: usize,
        num_of_mines: usize,
        game_state: GameState,
    }

    pub fn new_game(width: usize, height: usize, num_of_mines: usize) -> Game {
        let map = generate_map(width, height);
        Game {
            width,
            height,
            num_of_mines,
            game_state: GameState { map },
        }
    }

    pub fn generate_map(x: usize, y: usize) -> HashMap<String, MapTile> {
        let mut hm = HashMap::new();
        for x_cord in 0..x {
            for y_cord in 0..y {
                let map_key = format!("{}:{}", x_cord, y_cord);
                let map_tile = MapTile {
                    x: x_cord,
                    y: y_cord,
                    has_mine: false,
                    has_flag: false,
                };

                hm.insert(map_key, map_tile);
            }
        }
        hm
    }

    impl ExternalMinesweeper for Game {
        fn get_width(&self) -> usize {
            self.width
        }

        fn get_height(&self) -> usize {
            self.height
        }

        /// Toggle the presence of a flag on a tile.
        fn toggle_flag(&mut self, x: usize, y: usize) {
            let key = format!("{}:{}", x, y);
            let old_tile = self.game_state.map.get(&key).unwrap();

            let new_tile = MapTile {
                x: old_tile.x,
                y: old_tile.y,
                has_flag: !old_tile.has_flag,
                has_mine: old_tile.has_mine,
            };

            self.game_state.map.insert(key, new_tile);
        }
        /// Explore a tile. If it was a mine, BOOM! If not, flood fill mine scouting out from it.
        ///
        /// Returns true if BOOM!
        fn explore_tile(&mut self, x: usize, y: usize) -> bool {
            let key = format!("{}:{}", x, y);
            return self.game_state.map.get(&key).unwrap().has_mine;
        }
        /// Return whether a tile should be displayed as blank, a flag, a cleared mine, a mine count 1-8, or a kaboom.
        fn get_tile(&self, x: usize, y: usize) -> Option<&MapTile> {
            let key = &format!("{}:{}", x, y);
            self.game_state.map.get(key)
        }

        fn set_mine(&mut self, x: usize, y: usize) -> Option<MapTile> {
            let key = format!("{}:{}", x, y);
            let old_tile = self.game_state.map.get(&key).unwrap();
            let new_tile = MapTile {
                x: old_tile.x,
                y: old_tile.y,
                has_flag: old_tile.has_flag,
                has_mine: !old_tile.has_mine,
            };
            self.game_state.map.insert(key, new_tile)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::*;

    #[test]
    fn test_new() {
        let new_game = game::new_game(10, 10, 3);

        assert_eq!(new_game.get_width(), 10);
        assert_eq!(new_game.get_height(), 10);
    }

    #[test]
    fn test_toggle_flag() {
        let mut new_game = game::new_game(10, 10, 3);

        let res = new_game.get_tile(1, 1).unwrap().to_owned();
        assert_eq!(false, res.has_flag);
        new_game.toggle_flag(1, 1);
        let res = new_game.get_tile(1, 1).unwrap().to_owned();
        assert_eq!(true, res.has_flag);
    }

    #[test]
    fn test_has_mine() {
        let mut game = game::new_game(10, 10, 3);
        let res = game.get_tile(1, 1).unwrap().to_owned();
        assert_eq!(false, res.has_mine);
        game.set_mine(1, 1);

        let res = game.get_tile(1, 1).unwrap().to_owned();
        assert_eq!(true, res.has_mine);
    }

    #[test]
    fn test_map() {
        let new_game = game::new_game(10, 10, 3);

        let res = new_game.get_tile(1, 1).unwrap().to_owned();
        // TODO: refactor test or remove or adjust, because mines will be added randomly
        assert_eq!(false, res.has_mine);
        assert_eq!(false, res.has_flag);
    }
}
