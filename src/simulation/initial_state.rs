use std::collections::HashSet;

use rand::*;
use rand_xorshift::XorShiftRng;

use crate::core::geo::polygon::*;
use crate::core::scalar::*;
use crate::core::vector::*;

use super::state::*;

const PORTION_OF_ENTITIES_COP: Scalar = 0.05;
const PORTION_OF_ENTITIES_ZOMBIE: Scalar = 0.2;

pub fn initial_state(entity_count: u32, random_seed: u32) -> State {
    let entity_count_fp = entity_count as Scalar;
    let cop_count: u32 = ((entity_count_fp * PORTION_OF_ENTITIES_COP) as u32).max(1);
    let zombie_count: u32 = ((entity_count_fp * PORTION_OF_ENTITIES_ZOMBIE) as u32).max(1);
    let human_count: u32 = entity_count - (cop_count + zombie_count);

    println!("Spawning {} entities: {} cops, {} zombies, and {} civilians",
             entity_count, cop_count, zombie_count, human_count);

    let mut state = State {
        entities: vec!(),
        buildings: vec!(),
        building_outlines: vec!(),
        selection: HashSet::new(),
        projectiles: vec!(),
        rng: XorShiftRng::seed_from_u64(random_seed as u64),
    };

    let entities = &mut state.entities;
    let buildings = &mut state.buildings;
    let building_outlines = &mut state.building_outlines;

    // We want the spawn area to be proportional to the number of entities
    // let side_length_of_spawn_area = 3.0 * entity_count_fp.sqrt();
    let side_length_of_spawn_area = 50.0;

    for i in 0..entity_count {
        // TODO: need to optimize this later with housing units and two entities shouldn't be placed on same tile
        let x = state.rng.gen_range(0.0, side_length_of_spawn_area);
        let y = state.rng.gen_range(0.0, side_length_of_spawn_area);
        let facing_angle = state.rng.gen_range(0.0, 1 as Scalar);
        let position = vector2(x, y);
        let velocity = Vector2::zero();

        let zombie_or_human = if i < zombie_count {
            ZombieOrHuman::Zombie {
                state: ZombieState::Roaming
            }
        } else {
            let human = if i < zombie_count + cop_count {
                Human::Cop { rounds_in_magazine: COP_MAGAZINE_CAPACITY, state_stack: vec!() }
            }
            else {
                Human::Civilian
            };
            ZombieOrHuman::Human {
                infection: ENTITY_INFECTION_MIN,
                human
            }
        };

        let dead_or_alive = DeadOrAlive::Alive {
            health: ENTITY_HEALTH_MAX,
            zombie_or_human
        };
        entities.push(Entity { position, velocity, facing_angle, dead_or_alive });
    }

    // Generate some buildings
    let mut building_x = 0.0;
    while building_x < 2.0 * side_length_of_spawn_area {
        let mut building_y = 0.0;

        while building_y < 2.0 * side_length_of_spawn_area {
            buildings.push(Polygon(vec![
                Vector2 { x: building_x, y: building_y },
                Vector2 { x: building_x + 10.0, y: building_y },
                Vector2 { x: building_x + 10.0, y: building_y + 10.0 },
                Vector2 { x: building_x, y: building_y + 10.0 }
            ]));

            building_y += 20.0;
        }

        building_x += 20.0;
    }

    // Generate World Boundary

    let border_top_x = 0.0;
    let border_top_y = 0.0;


    // Lower Boundary
    buildings.push(Polygon(vec![
        Vector2 { x: border_top_x - 25.0, y: border_top_y - 24.5 },
        Vector2 { x: border_top_x + 115.0, y: border_top_y - 24.5 },
        Vector2 { x: border_top_x + 115.0, y: border_top_y - 25.0 },
        Vector2 { x: border_top_x - 25.0, y: border_top_y - 25.0 }
    ]));

    // Right Boundary
    buildings.push(Polygon(vec![
        Vector2 { x: border_top_x + 114.5, y: border_top_y + 115.0 },
        Vector2 { x: border_top_x + 115.0, y: border_top_y + 115.0 },
        Vector2 { x: border_top_x + 114.5, y: border_top_y - 25.0 },
        Vector2 { x: border_top_x + 115.0, y: border_top_y - 25.0 }
    ]));
    // Left Boundary
    buildings.push(Polygon(vec![
        Vector2 { x: border_top_x - 24.5, y: border_top_y + 115.0 },
        Vector2 { x: border_top_x - 25.0, y: border_top_y + 115.0 },
        Vector2 { x: border_top_x - 24.5, y: border_top_y - 25.0 },
        Vector2 { x: border_top_x - 25.0, y: border_top_y - 25.0 }
    ]));

    // Upper Boundary
    buildings.push(Polygon(vec![
        Vector2 { x: border_top_x - 25.0, y: border_top_y + 115.0 },
        Vector2 { x: border_top_x + 115.0, y: border_top_y + 115.0 },
        Vector2 { x: border_top_x + 115.0, y: border_top_y + 114.5 },
        Vector2 { x: border_top_x - 25.0, y: border_top_y + 114.5 }
    ]));

    // Generate outlines around all buildings for building A* pathfinding graphs
    for i in 0..buildings.len() {
        let mut outlines = vec!();
        let norms = buildings[i].normals();

        for j in 0..buildings[i].num_sides() {
            let norm_sum = norms[j] + norms[if j < 1 { buildings[i].num_sides() - 1 } else { j - 1 }];
            let offset = norm_sum * ENTITY_RADIUS * 1.1;
            outlines.push(offset + buildings[i].get(j));
        }

        building_outlines.push(Polygon(outlines));
    }

    state
}
