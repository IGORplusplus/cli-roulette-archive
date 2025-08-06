use rand::thread_rng;

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
