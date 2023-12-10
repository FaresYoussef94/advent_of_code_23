// use std::collections::HashSet;

#[derive(Clone, Copy, Default)]
pub struct GameDefinition{
    pub game_id: usize,
    pub winning_cards: [u32; 10],
    pub round_cards: [u32; 25],
}

