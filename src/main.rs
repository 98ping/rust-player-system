use player_model::{PlayerModel};
use crate::player_model::{Item, Statistics};

#[path= "player/player_model.rs"]
mod player_model;

fn main() {
    let player = make_player();
    let opponent = make_player();

    println!("Health: {}", player.health);
    println!("Kills: {}", player.stats.kills);
    println!("Item Size: {}", player.items.len());

    let result = kill_other(opponent, player);

    println!("New Kills: {}", result.1.stats.kills);
    println!("New Killstreak: {}", result.1.stats.killstreak)

}

fn make_player() -> PlayerModel {
    return PlayerModel {
        health: 20,
        id: 1,
        stats: Statistics {
            kills: 0,
            deaths: 0,
            killstreak: 0
        },
        items: [Item {
            amount: 0,
            name: "".to_string(),
        }].into_iter().collect::<Vec<Item>>()
    }
}

fn add_item(
    mut target: PlayerModel, item: Item
) -> PlayerModel {
    target.items.push(item);

    return target
}

fn kill_other(
    mut model: PlayerModel, mut ours: PlayerModel
) -> (PlayerModel, PlayerModel) {
    println!("You have killed {}", model.id);
    model.stats.deaths += 1;

    ours.stats.kills += 1;
    ours.stats.killstreak += 1;

    return (model, ours)
}