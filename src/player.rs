use crate::items::ItemS;
use rand::{thread_rng, seq::SliceRandom};

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub items: Vec<ItemS>,
    pub health: u8,
    pub cpu: bool,
}
