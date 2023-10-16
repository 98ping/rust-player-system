use r#mod::{PlayerModel};
use crate::r#mod::{Item, Statistics};

#[path= "player/mod.rs"]
mod r#mod;

fn main() {
    let player = &mut make_player();
    let opponent = &mut make_player();

    println!("Health: {}", player.health);
    println!("Kills: {}", player.stats.kills);
    println!("Item Size: {}", player.items.len());

    let result = kill_other(opponent, player);

    println!("New Kills: {}", player.stats.kills);
    println!("New Killstreak: {}", player.stats.killstreak)

}

fn make_player<'a>() -> PlayerModel {
    return PlayerModel {
        health: 0,
        id: 0,
        stats: Statistics::default(),
        items: vec![],
    }
}

fn add_item(target: &mut PlayerModel, item: Item) -> &PlayerModel {
    target.items.push(item);
    return target
}

fn kill_other(model: &mut PlayerModel, ours: &mut PlayerModel) {
    println!("You have killed {}", model.id);
    model.stats.deaths += 1;

    ours.stats.kills += 1;
    ours.stats.killstreak += 1;
}