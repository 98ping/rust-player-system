pub struct PlayerModel {
    pub health: i32,
    pub id: i32,
    pub stats: Statistics,
    pub credentials: Credential,
    pub items: Vec<Item>,
}

impl Default for PlayerModel {
    fn default() -> Self {
        return Self {
            health: 20,
            id: 1,
            stats: Statistics {
                kills: 0,
                deaths: 0,
                killstreak: 0,
            },
            credentials: Credential {
                username: "John Doe".to_string(),
                password: "ChangeMe123".to_string(),
            },
            items: vec![Item {
                amount: 0,
                name: "".to_string(),
            }],
        };
    }
}

#[derive(Default)]
pub struct Statistics {
    pub kills: i32,
    pub deaths: i32,
    pub killstreak: i32,
}


pub struct Item {
    pub amount: i32,
    pub name: String,
}

#[derive(Default)]
pub struct Credential {
    pub username: String,
    pub password: String
}
