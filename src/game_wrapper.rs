use crate::game_state::GameState;
use crate::player::Player;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct GameWrapper {
    game: GameState,
}

#[wasm_bindgen]
impl GameWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameWrapper {
        GameWrapper {
            game: GameState::new(),
        }
    }

    pub fn board(&self) -> JsValue {
        to_value(&self.game.board()).unwrap()
    }

    pub fn current_player(&self) -> Player {
        self.game.current_player()
    }

    pub fn make_move(&mut self, cell: usize) -> Result<(), JsValue> {
        self.game
            .make_move(cell)
            .map_err(JsValue::from_str)
    }

    pub fn check_win(&self) -> Option<crate::player::Player> {
        self.game.check_win()
    }
}
