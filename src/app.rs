use std::collections::HashMap;
use anyhow::Result;
use rand::distributions::{ Distribution, WeightedIndex };
use rand::thread_rng;

pub enum CurrentScreen {
    Menu,
    PlayerSelection(usize),
    ItemSelection(ItemsM),
    Visual,
    Exiting,
}

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
    //can be player on other people in which case it isn't secret anymore, one use
    Adrenaline,
    LSD, //when used on you or another player the insepect instead shows them a random shell which
    //is specified
}

//singleplayer items
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

pub struct Player {
    pub name: String,
    pub items: Vec<ItemS>,
    pub health: u8,
}

pub struct VisualData {
    pub players: Vec<Player>,
    pub items: Vec<ItemS>,
}

pub struct App {
    pub key_input: String, // the currently being edited json key.
    pub value_input: String, // the currently being edited json value.
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
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
