extern crate cfg_if;
use cfg_if::cfg_if;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod utils;
mod tetris_game;


cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn processString(s: &str);
    fn drawBoard(s: &str );
    fn drawScore(s: &str );
    fn drawNextStone(s: &str );
    fn log(s: &str );
}

#[wasm_bindgen]
pub fn draw() {
    drawBoard("board content");
}

#[wasm_bindgen]
pub struct TheGame {

    game : tetris_game::Game,

}

#[wasm_bindgen]
impl TheGame {

    pub fn new( r1: f64, r2: f64 ) -> TheGame {

        utils::set_panic_hook();

        let mut g = tetris_game::Game::new( r1, r2 );
        TheGame {
            game : g,
        }
    }

    /// turn stone

    pub fn turn(&mut self) {
        log("turn"); 
        self.game.next_move( false, 0, 1, 0.0f64 );
        self.draw();
   }

    pub fn left(&mut self) {
        log("left"); 
        self.game.next_move( false, -1, 0, 0.0f64 );
        self.draw();
    }

    pub fn right(&mut self) {
        log("right"); 
        self.game.next_move( false, 1, 0, 0.0f64 );
        self.draw();
    }

    pub fn next_tick(&mut self, rand: f64 ) -> bool {
        log("next"); 
        let gameover = self.game.next_move( true, 0, 0, rand );
        self.draw();
        gameover
    }

    /// draw the board

    pub fn draw( &self ) {
        log("draw"); 
        let b = self.game.board_state();
        drawBoard( b.as_str() );
        self.draw_score();
    }

    pub fn draw_score( &self ){
        log("draw_score"); 
        drawScore( format!("{}", self.game.score ).as_str() );        
    }

    pub fn draw_next_stone( &self ) {
        log("draw_next_stone"); 
        let next = self.game.next_stone();
        drawNextStone( next.as_str() );
    }

}






