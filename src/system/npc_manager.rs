use crate::data::game_state::{MAP, GameState};
use crate::data::component::*;
use crate::data::constants::*;
use specs::prelude::*;
use rand::prelude::*;

pub fn spawn_npcs(gs: &mut GameState) {
    let mut rng = thread_rng();
    for _ in 1..100 {
        let (x, y) = (rng.gen_range(0, 10), rng.gen_range(0, 10)); // start at random location
        let loc = MAP[x][y];
        let name_opts = ["Kobold", "Orc", "Dark-Elf", "Marauder"];
        let dir_opts = ["n", "s", "e", "w"];

        let new_enemy = gs.ecs
            .create_entity()
            .with(NPC {})
            .with(Name { val: String::from(name_opts[rng.gen_range(0, 4)]) })
            .with(loc)
            .with(Aggressive {})
            .build();

        if rng.gen_range(0, 2) == 1 {
            gs.add_mover(new_enemy, Direction::from(String::from(dir_opts[rng.gen_range(0, 4)])));
        }
    }
}