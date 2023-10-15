
pub struct PlayerModel {
    pub health: i32,
    pub id: i32,
    pub stats: Statistics,
    pub items: Vec<Item>
}


pub struct Statistics {
    pub kills: i32,
    pub deaths: i32,
    pub killstreak: i32
}


pub struct Item {
    pub amount: i32,
    pub name: String,
}
