extern crate rand;

use rand::Rng;
use crate::core::vector::*;
use crate::core::scalar::*;
use crate::constants::presentation::*;

use super::state::*;

pub fn initial_state(count: u32) -> State {

    let mut state = State { entities: vec!()};

    let entities = &mut state.entities;

    for _i in 0..count {
        // TODO: need to optimize this later with housing units and two entities shouldn't be placed on same tile
        let mut rng = rand::thread_rng();
        let x: Scalar = rng.gen_range(0.0f64, 10 as f64); //WINDOW_W as f64);
        let y: Scalar = rng.gen_range(0.0f64, 10 as f64); //WINDOW_H as f64);
        let position = Vector2{x: x, y: y};
        let velocity = Vector2::zero();
        let human_count: u32 = (count as f32 * 0.8) as u32;
        let cop_count: u32 = (count as f32 * 0.18) as u32;
        let zombie_count = count - (human_count + cop_count);
        // spawn 80% humans
        if _i < human_count {
            entities.push(Entity { position, velocity, behaviour: Behaviour::Human });
        }
        // spawn 18% cops
        else if  _i >= human_count && _i < (count - zombie_count) {
            entities.push(Entity { position, velocity, behaviour: Behaviour::Cop });
        }
        // spawn rest zombie
        else {
            entities.push(Entity { position, velocity, behaviour: Behaviour::Zombie });
        }
    }
    state
}
