// use std::collections::HashMap;
use rand::Rng;

// use crate::home::SmartHome;

use crate::world::{World, Entity};

const HEATER_POWER: [f32; 6] = [0.0, 0.5, 1.0, 2.0, 3.5, 5.0]; // level 0..3
const OUTER_TEMP: f32 = 13.0;

pub fn update_tick(world: &mut World){
    world.tick += 1;
}

pub fn print_dashboard(world: &World) {
    print!("\x1B[2J\x1B[1;1H"); // clear screen, move cursor to top-left
    let mut ids: Vec<&Entity> = world.sensors.keys().collect();
    ids.sort();

    for id in ids {
        let sensor = world.sensors.get(id).unwrap();
        let heater_status = world.heater_modes.get(id).unwrap();
        let temp_diff = if let Some(history_component) = world.temperature_histories.get(id) {
            // If it exists, try to get the tick - 1 value
            if let Some((_, prev_temp)) = history_component.history.iter().copied().rev().nth(1) {
                sensor.current_temp - prev_temp
            } else {
                0.0 // Fallback if it's Tick 0
            }
        } else {
            0.0 // Fallback if the room has NO history component at all
        };
        println!("┌─ Room {} ────────────┐", id);
        println!("│ Heater: {:<12}│", heater_status.level);
        println!("│ Temp: {:<6.1}°C      │", sensor.current_temp);
        println!("│ Diff: {:<6.1}°C      │", temp_diff);
        println!("└─────────────────────┘");
    }
}

pub fn climate_record_system(world: &mut World){
    let ids: Vec<Entity> = world.sensors.keys().copied().collect();
    for id in ids {
        if let (Some(sensor), Some(temperature_history)) = (
            world.sensors.get_mut(&id),
            world.temperature_histories.get_mut(&id),
        ) {
            temperature_history.history.push((world.tick, sensor.current_temp));

            if temperature_history.history.len() > 60 {
                temperature_history.history.remove(0); // For small vectors under ~100 items, this is perfectly fine
            }
        }
        
    }
}

pub fn climate_control_system(world: &mut World) {
    let ids: Vec<Entity> = world.sensors.keys().copied().collect();

    for id in ids {
        if let (Some(sensor), Some(thermostat), Some(heater_mode), Some(temperature_history)) = (
            world.sensors.get_mut(&id),
            world.thermostats.get_mut(&id),
            world.heater_modes.get_mut(&id),
            world.temperature_histories.get_mut(&id),
        ) {

            if let Some((prev_tick, prev_temp)) = temperature_history.history.iter().rev().nth(1) {
                if sensor.current_temp < *prev_temp {
                    if heater_mode.level < 5{
                        heater_mode.level += 1;
                    }
                }
                else{
                    if heater_mode.level > 0{
                        heater_mode.level -= 1;
                    }
                }
            }
            sensor.current_temp += HEATER_POWER[heater_mode.level as usize];
        }
    }
}

pub fn nature_sytem(world: &mut World) {
    let ids: Vec<Entity> = world.sensors.keys().copied().collect();
    let mut rng = rand::thread_rng();
    for id in ids{
        if let Some(sensor) = world.sensors.get_mut(&id) {
            let random_fluctuation: f32 = rng.gen_range(0.2..0.5);
            if OUTER_TEMP < sensor.current_temp {
                sensor.current_temp -= random_fluctuation * (sensor.current_temp - OUTER_TEMP);
            }
        }
    }
}

//    pub fn climate_control_system(world: &mut World) {
//        for (id, sensor, thermostat) in world.query_sensor_thermostat_mut() {
//            if sensor.current_temp < thermostat.target_temp {
//                println!("Heating up room {}", id);
//            }
//        }
//    }


// pub fn heat_system(home: &mut SmartHome){
//     for (&room_id, sensor) in home.sensors.iter(){
//         if let some(thermostat) = home.thermostat.get_mut(&room_id){
//             if thermostat.heater_on{
//                 sensor.current_temp += HEATER_TEMP_CHANGE_RATE;
//             }
//         }
//     }
// }

// const OUTDOOW_TEMP: f32 = 10.0;
// const TEMP_CHANGE_RATE: f32 = 0.5;

// pub fn nature_system(home: &mut SmartHome){
//     for (&room_id, sensor) in home.sensors.iter(){
//         let random_fluctuation: f32 = rng.gen_range(-0.5..0.5);
//         if (sensor.current_temp) < TEMP_CHANGE_RATE {
//             sensor.current_temp += TEMP_CHANGE_RATE + random_fluctuation;
//         }

//         else{
//             sensor.current_temp -= TEMP_CHANGE_RATE + random_fluctuation;
//         }
//     }
// }

