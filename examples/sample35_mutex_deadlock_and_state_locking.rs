#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use std::{collections::HashMap, sync::Mutex};
#[allow(unused)]
fn main1() {
    struct Player {
        id: String,
    }
    #[derive(Clone)]
    struct GameId(String);
    struct Game {
        id: GameId,
        players: Vec<Player>,
    }
    impl Game {
        fn add_player(&mut self, player_name: &str) -> bool {
            if self.players.len() < 4 {
                self.players.push(Player {
                    id: player_name.to_owned(),
                });
                true
            } else {
                false
            }
        }
        fn remove_player(&mut self, player_name: &str) {
            self.players.retain(|p| p.id != player_name);
        }
        fn has_player(&self, player_name: &str) -> bool {
            self.players.iter().any(|p| p.id == player_name)
        }
    }

    struct GameServer {
        // Map player name to player info.
        players: Mutex<HashMap<String, Player>>,
        // Current games, indexed by unique game ID.
        games: Mutex<HashMap<GameId, Game>>,
    }

    impl GameServer {
        // Add a new player and join them to a current game.
        fn add_and_join(&self, username: &str, info: Player) -> Option<GameId> {
            // Add the new player
            let mut players = self.players.lock().unwrap();
            players.insert(username.to_owned(), info);
            // Find a game with available space for them to join
            let mut games = self.games.lock().unwrap();
            for (id, game) in games.iter_mut() {
                if game.add_player(username) {
                    return Some(id.clone());
                }
            }
            None
        }

        // Ban the player identified by 'username'm removing them from
        // any current games.
        fn ban_player(&self, username: &str) {
            // Find all games that the user is in and remove them
            let mut games = self.games.lock().unwrap();
            games
                .iter_mut()
                .filter(|(_id, g)| g.has_player(username))
                .for_each(|(_id, g)| g.remove_player(username));

            // Remove the player from the player list
            let mut players = self.players.lock().unwrap();
            players.remove(username);
        }
    }

    // ✖ add_and_join()がplayersのロックを取得し、ban_player()がgamesのロックを取得した場合に
    // デッドロックが発生し、処理が停止する
}

#[allow(unused)]
fn main2() {
    struct Player {
        id: String,
    }
    #[derive(Clone)]
    struct GameId(String);
    struct Game {
        id: GameId,
        players: Vec<Player>,
    }
    impl Game {
        fn add_player(&mut self, player_name: &str) -> bool {
            if self.players.len() < 4 {
                self.players.push(Player {
                    id: player_name.to_owned(),
                });
                true
            } else {
                false
            }
        }
        fn remove_player(&mut self, player_name: &str) {
            self.players.retain(|p| p.id != player_name);
        }
        fn has_player(&self, player_name: &str) -> bool {
            self.players.iter().any(|p| p.id == player_name)
        }
    }

    struct GameServer {
        // Map player name to player info.
        players: Mutex<HashMap<String, Player>>,
        // Current games, indexed by unique game ID.
        games: Mutex<HashMap<GameId, Game>>,
    }

    impl GameServer {
        // Add a new player and join them to a current game.
        fn add_and_join(&self, username: &str, info: Player) -> Option<GameId> {
            // Add the new player
            {
                let mut players = self.players.lock().unwrap();
                players.insert(username.to_owned(), info);
            }
            // Find a game with available space for them to join
            {
                let mut games = self.games.lock().unwrap();
                for (id, game) in games.iter_mut() {
                    if game.add_player(username) {
                        return Some(id.clone());
                    }
                }
            }
            None
        }

        // Ban the player identified by 'username'm removing them from
        // any current games.
        fn ban_player(&self, username: &str) {
            // Find all games that the user is in and remove them
            {
                let mut games = self.games.lock().unwrap();
                games
                    .iter_mut()
                    .filter(|(_id, g)| g.has_player(username))
                    .for_each(|(_id, g)| g.remove_player(username));
            }
            // Remove the player from the player list
            {
                let mut players = self.players.lock().unwrap();
                players.remove(username);
            }
        }
    }

    // ✖ スコープを短くすることでデッドロックを回避しているが、データ整合性に問題が起きる
}

#[allow(unused)]
fn main3() {
    struct Player {
        id: String,
    }
    #[derive(Clone)]
    struct GameId(String);
    struct Game {
        id: GameId,
        players: Vec<Player>,
    }
    impl Game {
        fn add_player(&mut self, player_name: &str) -> bool {
            if self.players.len() < 4 {
                self.players.push(Player {
                    id: player_name.to_owned(),
                });
                true
            } else {
                false
            }
        }
        fn remove_player(&mut self, player_name: &str) {
            self.players.retain(|p| p.id != player_name);
        }
        fn has_player(&self, player_name: &str) -> bool {
            self.players.iter().any(|p| p.id == player_name)
        }
    }
    struct GameState {
        players: HashMap<String, Player>,
        games: HashMap<GameId, Game>,
    }
    // Mutexの範囲をGameState全体に広げることで、データの整合性を保つ
    struct GameServer {
        state: Mutex<GameState>,
    }

    impl GameServer {
        // Add a new player and join them to a current game.
        fn add_and_join(&self, username: &str, info: Player) -> Option<GameId> {
            // Add the new player
            let mut state = self.state.lock().unwrap();
            state.players.insert(username.to_owned(), info);
            // Find a game with available space for them to join
            for (id, game) in state.games.iter_mut() {
                if game.add_player(username) {
                    return Some(id.clone());
                }
            }
            None
        }

        // Ban the player identified by 'username'm removing them from
        // any current games.
        fn ban_player(&self, username: &str) {
            // Find all games that the user is in and remove them
            let mut state = self.state.lock().unwrap();
            state
                .games
                .iter_mut()
                .filter(|(_id, g)| g.has_player(username))
                .for_each(|(_id, g)| g.remove_player(username));
            // Remove the player from the player list
            state.players.remove(username);
        }
    }

    // 〇 データの整合性を保ちながら、デッドロックを回避している
    // ただし、stateのロックを取得している間は他の操作ができないため、
    // パフォーマンスに影響が出る可能性がある
}

fn main() {
    main3();
}
