use std::path::Component::ParentDir;
use log::error;
use r#mod::{PlayerModel};
use crate::r#mod::{Credential, Item, Statistics};

#[path= "mod.rs"]
mod r#mod;

fn main() {
    let player = &mut make_player();
    let opponent = &mut make_player();

    println!("Health: {}", player.health);
    println!("Kills: {}", player.stats.kills);
    println!("Item Size: {}", player.items.len());

    kill_other(opponent, player);

    println!("New Kills: {}", player.stats.kills);
    println!("New Killstreak: {}", player.stats.killstreak);

    self_death(player);
    apply_player_credential(player, "Joe Bengal".to_string(), "MyPassword22".to_string())

}

fn make_player() -> PlayerModel {
    return PlayerModel {
        health: 0,
        id: 0,
        stats: Statistics::default(),
        credentials: Credential::default(),
        position: Position::default(),
        items: vec![]
    }
}

fn apply_player_credential(target: &mut PlayerModel, uname: String, pwd: String) {
    if pwd.contains(" ") || pwd.is_empty() {
        panic!("Your password must not be empty and contain no spaces!")
    }

    if pwd.len() < 8 {
        panic!("Your password is too short!")
    }

    target.credentials = Credential {
        username: uname,
        password: pwd
    };

    println!("Setup a new credential profile for {}", target.id)
}

fn add_item(target: &mut PlayerModel, item: Item) -> &PlayerModel {
    target.items.push(item);
    return target
}

fn self_death(player: &mut PlayerModel) {
    player.stats.deaths += 1;
    println!("You have died! You now have {} deaths", player.stats.deaths)
}

fn kill_other(model: &mut PlayerModel, ours: &mut PlayerModel) {
    println!("You have killed {}", model.id);
    model.stats.deaths += 1;

    ours.stats.kills += 1;
    ours.stats.killstreak += 1;
}