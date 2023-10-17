use r#mod::{PlayerModel};
use crate::r#mod::{Item, Statistics};

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
    println!("New Killstreak: {}", player.stats.killstreak)

    self_death(player)

}

fn make_player() -> PlayerModel {
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