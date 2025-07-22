use std::collections::HashMap;
use std::os::unix::thread;
use anyhow::Result;
use rand::distributions::{ Distribution, WeightedIndex };
use rand::thread_rng;
use rand::seq::SliceRandom;

//local and online multiplayer, and singleplayer
//multiplayer items easier to pull off, ten second timer is when a shot could be first fired
pub enum _ItemM {
    Saw, //doubles damage
    Beer, //ejects the next shell
    Cigarette, //restores one health
    Mirror, //deflects bullet only shown after a player makes a decision to 
    Inverter,
    MagnifyingGlass, //shows current shell
    Handcuffs, //skips next player's turn
    Meth, //restores two health, but keeps you from being able to see the next shell
    AED, //secret and default, if shot next round it keeps the health, if not lose a health
    Adrenaline,
    LSD, //when used on you or another player the insepect instead shows them a random shell which
    //is specified
}

//singleplayer items
#[derive(Clone, Copy, Debug)]
pub enum ItemS {
    Saw, //doubles damage
    Beer, //ejects the next shell
    Cigarette, //restores one health
    Mirror, //deflects bullet only shown after a player makes a decision to 
    Inverter,
    MagnifyingGlass, //shows current shell
    Handcuffs, //skips opponent's turn
    Meth, //restores two health, but keeps you from being able to see the next shell
    AED, //secret and default, if shot next round it keeps the health, if not lose a health
    //can be player on other people in which case it isn't secret anymore
    Adrenaline,
    LSD, //when used, you will be shown a shell in the future and it will be specified which number
    //it is 
}

const ALL_ITEMS: [ItemS; 11] = [
    ItemS::Saw,
    ItemS::Beer,
    ItemS::Cigarette,
    ItemS::Mirror,
    ItemS::Inverter,
    ItemS::MagnifyingGlass,
    ItemS::Handcuffs,
    ItemS::Meth,
    ItemS::AED,
    ItemS::Adrenaline,
    ItemS::LSD,
];

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

pub struct Player {
    pub name: String,
    pub items: Vec<ItemS>,
    pub health: u8,
    pub cpu: bool,
}

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

pub enum Shell {
    Live,
    Blank,
    Incendiary,
    BeanBag,
    Electric,
}

pub struct Shotgun {
    shells: Vec<Shell>,
    state: ShotgunState,
}
pub enum ShotgunState {
    Default,
    SawedOff,
    HotPotato,
}

use ShotgunState::*;
impl Shotgun {

    pub fn new() -> Shotgun { 
        Shotgun { 
            shells: Vec::new(),
            state: Default,
        }
    }

    pub fn reload(number_shells: u8, wild_card: bool) -> Shotgun {

        Shotgun {
            shells: new_shells,
            state: Default,
        }
    }
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

pub struct App {
    pub item_selected: Option<ItemS>, //when an item is selected it takes two seconds to use
    pub player_turn: Option<CurrentTurn>, //whose turn it is
    pub players: Players,
    pub current_screen: Option<CurrentScreen>, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_selecting: Option<Selection>,
    pub shotgun: Shotgun,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: String::new(),
            items: vec![ItemS::AED],
            health: 3,
        }
    }

    pub fn new_with_health(passed_health: u8) -> Player {

        Player {
            name: self.name,
            items: vec![ItemS::AED],
            health: passed_health,
        }
    }

    //called after every shot is taken
    pub fn new_items_reload(player: Player, item_count: u8) -> Player {
        let mut rng = thread_rng();
        let mut all_items: Vec<ItemS> = ItemS::all().collect();

        all_items.shuffle(&mut rng);

        let new_items = all_items.iter().take(item_count);

        player.items.extend(new_items);
        player
    }

    pub fn new_round(passed_health: u8, player: Player) -> Player {
        Player {
            name: player.name,
            items: vec![ItemS::AED],
            health: passed_health,
        }
    }

    pub fn use_item(chosen_item: ItemS, mut player: Player) -> (Player, bool) {
        let found = if let Some(pos) = player.items.iter().position(|&item| item == chosen_item){
            player.items.remove(pos);
            true
        } else {
            false
        };
        (player, found)
    }
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
    // --snip--
}
