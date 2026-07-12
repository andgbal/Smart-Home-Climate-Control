// use std::collections::HashMap;
use rand::Rng;

// use crate::home::SmartHome;

use crate::world::{World, Entity};

const HEATER_POWER: [f32; 4] = [0.0, 0.5, 1.0, 1.5]; // level 0..3

pub fn print_dashboard(world: &World) {
    print!("\x1B[2J\x1B[1;1H"); // clear screen, move cursor to top-left
    let mut ids: Vec<&Entity> = world.sensors.keys().collect();
    ids.sort();

    for id in ids {
        let sensor = world.sensors.get(id).unwrap();
        let heater_status = world.heater_modes.get(id).unwrap();
        println!("┌─ Room {} ─────────────┐", id);
        println!("│ Heater: {:<12}│", heater_status.level);
        println!("│ Temp:   {:<6.1}°C      │", sensor.current_temp);
        println!("└───────────────────────┘");
    }
}

pub fn climate_control_system(world: &mut World) {
    let ids: Vec<Entity> = world.sensors.keys().copied().collect();

    for id in ids {
        if let (Some(sensor), Some(thermostat), Some(heater_mode)) = (
            world.sensors.get_mut(&id),
            world.thermostats.get_mut(&id),
            world.heater_modes.get_mut(&id),
        ) {
            if sensor.current_temp < thermostat.target_temp {
                
                if heater_mode.level < 3{
                    heater_mode.level += 1;
                }
                //println!("Heating up room {} with temp {}", id, sensor.current_temp);
            }
            else{
                if heater_mode.level > 0{
                    heater_mode.level -= 1;
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
            let random_fluctuation: f32 = rng.gen_range(-0.5..0.5);
            sensor.current_temp += random_fluctuation;
            sensor.current_temp -= 0.8;
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

