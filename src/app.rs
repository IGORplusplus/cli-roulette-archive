use std::collections::HashMap;
use std::os::unix::thread;
use anyhow::Result;
use rand::distributions::{ Distribution, WeightedIndex };
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::player::Player;


//local and online multiplayer, and singleplayer
//multiplayer items easier to pull off, ten second timer is when a shot could be first fired
impl ItemS {
    pub fn all() -> impl Iterator<Item = ItemS> {
        ALL_ITEMS.into_iter()
    }
    pub fn new_items(num_items: u8) -> Vec<ItemS> {
        let mut rng = thread_rng();
        let mut items = Vec::new();
        for _ in 0..num_items {
            if let Some(item) = ALL_ITEMS.choose(&mut rng) {
                items.push(*item);
            }
        }
        items
    }
}

//initializes the game to have one player and a cpu
pub fn player_init_single() -> Vec<Player> {
    let user_items = ItemS::new_items(2);
    let cpu_items = ItemS::new_items(2);
    let user1 = Player {
        name: String::from("player1"),
        items: user_items,
        health: 3,
        cpu: false,
    };
    let cpu = Player {
        name: String::from("cpu"),
        items: cpu_items,
        health: 3,
        cpu: true,
    };
    vec![ user1, cpu ]
}

pub enum Selection {
    Player(Player),
    Item(ItemS),
}

pub enum CurrentTurn {
    Playing(usize),
}

pub enum CurrentScreen {
    Main,
    Menu,
    Selection(Selection),
    Visual,
    Exiting,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct App {
    pub item_selected: Option<ItemS>, //when an item is selected it takes two seconds to use
    pub player_turn: Option<CurrentTurn>, //whose turn it is
    pub players: Vec<Player>,
    pub current_screen: Option<CurrentScreen>, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_selecting: Option<Selection>,
    pub shotgun: Shotgun,
}

impl App {
    pub fn initialize() -> App {
        App {
            item_selected: None(),
            player_turn: None(),
            players: Players::new(),
            current_screen: Some(CurrentScreen::Menu),
            shotgun: Shotgun::new(),
        }
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key 
                => {
                    self.currently_editing 
                        = Some(CurrentlyEditing::Value)
                }

                CurrentlyEditing::Value 
                => {
                        self.currently_editing 
                            = Some(CurrentlyEditing::Key)
                }
            };
        } else {
            self.currently_editing 
                = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output 
            = serde_json::to_string(&self.pairs)?;

        println!("{}", output);
        Ok(())
    }
}
