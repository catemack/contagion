use lerp::*;
use rand::*;
use rand_xorshift::XorShiftRng;
use std::collections::HashSet;

use crate::core::geo::polygon::*;
use crate::core::scalar::*;
use crate::core::vector::*;
use std::collections::HashMap;

use super::state::*;

pub fn initial_state(entity_count: u32, cop_entities: f64, infected_entities: f64, random_seed: u32) -> State {
    let portion_of_entities_cop = cop_entities as Scalar;
    let portion_of_entities_infected = infected_entities as Scalar;
    let entity_count_fp = entity_count as Scalar;

    let cop_count: u32 = ((entity_count_fp * portion_of_entities_cop) as u32).max(1);
    let infected_count: u32 = ((entity_count_fp * portion_of_entities_infected) as u32).max(1);
    let civilian_count: u32 = entity_count - (cop_count + infected_count);

    println!("Spawning {} entities: {} cops, {} infected, and {} civilians",
             entity_count, cop_count, infected_count, civilian_count);

    let mut state = State {
        entities: vec!(),
        buildings: vec!(),
        building_outlines: vec!(),
        building_type: HashMap::new(),
        barricades: vec!(),
        selection: HashSet::new(),
        projectiles: vec!(),
        rng: XorShiftRng::seed_from_u64(random_seed as u64),
        money: 20
    };

    let entities = &mut state.entities;
    let buildings = &mut state.buildings;
    let building_outlines = &mut state.building_outlines;
    let building_type = &mut state.building_type;

    // We want the spawn area to be proportional to the number of entities
    // let side_length_of_spawn_area = 3.0 * entity_count_fp.sqrt();
    let side_length_of_spawn_area = 100.0;

    for i in 0..entity_count {
        // TODO: need to optimize this later with housing units and two entities shouldn't be placed on same tile
        let x = state.rng.gen_range(0.0, side_length_of_spawn_area);
        let y = state.rng.gen_range(0.0, side_length_of_spawn_area);
        let facing_angle = state.rng.gen_range(0.0, 1 as Scalar);
        let position = vector2(x, y);
        let velocity = Vector2::zero();

        let infection = if i < infected_count {
            INFECTION_MIN.lerp(INFECTION_MAX, state.rng.gen_range(0.0, 1.0))
        }
        else {
            INFECTION_MIN
        };

        let human = if infected_count <= i && i < infected_count + cop_count {
            let cop_type = if i == infected_count { CopType::Soldier } else { CopType::Normal };
            Human::Cop { cop_type, rounds_in_magazine: cop_type.magazine_capacity(), state_stack: vec!() }
        }
        else {
            Human::Civilian {
                state: HumanState::Running,
                punch_time_cooldown: PUNCH_TIME_COOLDOWN,
                left_hand_status: HandStatus::Normal,
                right_hand_status: HandStatus::Normal
            }
        };

        let dead_or_alive = DeadOrAlive::Alive {
            health: ENTITY_HEALTH_MAX,
            zombie_or_human: ZombieOrHuman::Human {
                infection,
                human
            }
        };
        entities.push(Entity { position, velocity, facing_angle, dead_or_alive });
    }

    // Generate some buildings
    let mut building_x = -20.0;
    let mut building_y = 0.0;
    let mut current_index: usize = 0;

    // Neighbourhood on south side
    while building_x < (side_length_of_spawn_area) + 20.0 {
        building_y = -20.0;

        while building_y < side_length_of_spawn_area / 2.0 - 20.0 {
            buildings.push(Polygon(vec![
                Vector2 { x: building_x, y: building_y },
                Vector2 { x: building_x + 10.0, y: building_y },
                Vector2 { x: building_x + 10.0, y: building_y + 10.0 },
                Vector2 { x: building_x, y: building_y + 10.0 }
            ]));
            building_type.insert(current_index, 0);
            current_index += 1;

            building_y += 20.0;
        }

        building_x += 20.0;
    }

    // Southwest building in the square
    buildings.push(Polygon(vec![
        Vector2 { x: 10.0, y: building_y },
        Vector2 { x: 30.0, y: building_y },
        Vector2 { x: 30.0, y: building_y + 10.0 },
        Vector2 { x: 20.0, y: building_y + 20.0},
        Vector2 { x: 10.0, y: building_y + 20.0 }
    ]));
    building_type.insert(current_index, 1);
    current_index += 1;

    // Southeast building in the square
    buildings.push(Polygon(vec![
        Vector2 { x: 70.0, y: building_y },
        Vector2 { x: 50.0, y: building_y },
        Vector2 { x: 50.0, y: building_y + 10.0 },
        Vector2 { x: 60.0, y: building_y + 20.0 },
        Vector2 { x: 70.0, y: building_y + 20.0 }
    ]));
    building_type.insert(current_index, 2);
    current_index += 1;

    // Octagonal building in the middle of the square
    buildings.push(Polygon(vec![
        Vector2 { x: 32.5, y: building_y + 22.5 },
        Vector2 { x: 32.5, y: building_y + 27.5 },
        Vector2 { x: 37.5, y: building_y + 32.5 },
        Vector2 { x: 42.5, y: building_y + 32.5 },
        Vector2 { x: 47.5, y: building_y + 27.5 },
        Vector2 { x: 47.5, y: building_y + 22.5 },
        Vector2 { x: 42.5, y: building_y + 17.5 },
        Vector2 { x: 37.5, y: building_y + 17.5 }
    ]));
    building_type.insert(current_index, 3);
    current_index += 1;

    buildings.push(Polygon(vec![
        Vector2 { x: -20.0, y: building_y },
        Vector2 { x: -20.0, y: building_y + 30.0 },
        Vector2 { x: -5.0, y: building_y + 30.0 },
        Vector2 { x: -5.0, y: building_y }
    ]));
    building_type.insert(current_index, 4);
    current_index += 1;

    buildings.push(Polygon(vec![
        Vector2 { x: 95.0, y: building_y },
        Vector2 { x: 95.0, y: building_y + 30.0 },
        Vector2 { x: 80.0, y: building_y + 30.0 },
        Vector2 { x: 80.0, y: building_y }
    ]));
    building_type.insert(current_index, 4);
    current_index += 1;

    building_y = building_y + 30.0;

    // Northwest building in the square
    buildings.push(Polygon(vec![
        Vector2 { x: 10.0, y: building_y },
        Vector2 { x: 10.0, y: building_y + 20.0 },
        Vector2 { x: 30.0, y: building_y + 20.0 },
        Vector2 { x: 30.0, y: building_y + 10.0 },
        Vector2 { x: 20.0, y: building_y }
    ]));
    building_type.insert(current_index, 5);
    current_index += 1;

    // Northeast building in the square
    buildings.push(Polygon(vec![
        Vector2 { x: 70.0, y: building_y },
        Vector2 { x: 70.0, y: building_y + 20.0 },
        Vector2 { x: 50.0, y: building_y + 20.0 },
        Vector2 { x: 50.0, y: building_y + 10.0 },
        Vector2 { x: 60.0, y: building_y }
    ]));
    building_type.insert(current_index, 6);

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
