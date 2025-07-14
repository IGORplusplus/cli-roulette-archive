use std::collections::HashMap;
use anyhow::Result;
use rand::distributions::{ Distribution, WeightedIndex };
use rand::thread_rng;
use rand::seq::SliceRandom;

//wrap this in an option for the none
pub enum CurrentTurn {
    Playing,
    Enemy(usize), //cpu will be 0
}


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
}

pub struct Player {
    pub name: String,
    pub id: u8,
    pub items: Vec<ItemS>,
    pub health: u8,
}

pub enum Selection {
    Player(Player),
    Item(ItemS),
}

pub enum Shell {
    Live,
    Blank,
}

pub enum CurrentScreen {
    Main,
    Menu,
    Selection(Selection),
    Visual,
    Exiting,
}

pub struct Data {
    pub players: Vec<Player>,
    pub items: Vec<ItemS>,
    pub turn: u8,
}

pub struct App {
    pub item_selected: Option<ItemS>,
    pub player_turn: Option<Player>, //whose turn it is
    pub player: Player,
    pub enemy: Player,
    pub current_screen: Option<CurrentScreen>, // the current screen the user is looking at, and will later determine what is rendered.
    pub data: Data,
    pub currently_selecting: Option<Selection>,
    pub shotgun: Vec<Shell>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            players: Vec::new(),
            items: Vec::new(),
            turn: 0,
        }
    }

    pub fn default_single(name: String, health: u8) -> Data {
        let player1 = Player {
            name: String::from(name),
            id: 0, 
            items: vec![ItemS::AED],
            health: health,
        }
        let cpu = Player {
            name: String::from("cpu"),
            id: 1,
            items: vec![ItemS::AED],
            health: health,
        }
        Data {
            players: vec![player1, cpu],
            //will not reallt be using this items vector
            //until I come up with shared items
            items: Vec::new(),
            turn: 0,
        }
    }
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
            name: String::new(),
            items: vec![ItemS::AED],
            health: passed_health,
        }
    }

    pub fn reload(player: Player, item_count: u8) -> Player {
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
    pub fn new() -> App {

        App {
            item_selected: None,
            player: Player::new(),
            enemy: Player::new(),
            current_screen: Some(CurrentScreen::Menu),
        }
    }

    // --snip--
    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(
                self.key_input
                    .clone(),

                self.value_input
                    .clone()
            );

        self.key_input 
            = String::new();

        self.value_input 
            = String::new();

        self.currently_editing 
            = None;
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
