use std::collections::HashMap;
use rand::Rng;

use crate::home::SmartHome;

pub fn climate_control_system(home: &mut SmartHome) {
    for (&room_id, sensor) in home.sensors.iter() {
        // Access both the thermostat AND the history for this room
        if let (Some(thermostat), Some(history)) = (
            home.thermostat.get_mut(&room_id), 
            home.history.get(&room_id)
        ) {
            
            // Calculate temperature trend (Current - 5 ticks ago)
            let trend = if history.readings.len() >= 5 {
                let last_temp = history.readings.last().unwrap().1;
                let old_temp = history.readings[history.readings.len() - 5].1;
                last_temp - old_temp
            } else {
                0.0 // Not enough data yet
            };

            // Now use the trend to adjust the heater
            if trend < 0.0 { // Temp is dropping
                if thermostat.heater_mode < 3 {
                    thermostat.heater_mode += 1;
                    println!("Room {}'s heater power increasing to {}!", room_id, thermostat.heater_mode);
                }
            } else if trend > 0.5 { // Temp is rising fast, turn it off
                if thermostat.heater_mode > 0{
                    thermostat.heater_mode -= 1;
                    println!("Room {}'s heater power decreasing to {}!", room_id, thermostat.heater_mode);
                }
                else{
                    thermostat.heater_mode = 0;
                    println!("Room {}'s heater turning OFF!", room_id);
                }
            }
        }
    }
}

const HEATER_TEMP_CHANGE_RATE: [(u8,f32); 4] = [
    (0, 0.0),
    (1, 0.6),
    (2, 0.9),
    (3, 1.2),
];

pub fn heat_system(home: &mut SmartHome){
    for (&room_id, sensor) in home.sensors.iter(){
        if let some(thermostat) = home.thermostat.get_mut(&room_id){
            if thermostat.heater_on{
                sensor.current_temp += HEATER_TEMP_CHANGE_RATE;
            }
        }
    }
}

const OUTDOOW_TEMP: f32 = 10.0;
const TEMP_CHANGE_RATE: f32 = 0.5;

pub fn nature_system(home: &mut SmartHome){
    for (&room_id, sensor) in home.sensors.iter(){
        let random_fluctuation: f32 = rng.gen_range(-0.5..0.5);
        if (sensor.current_temp) < TEMP_CHANGE_RATE {
            sensor.current_temp += TEMP_CHANGE_RATE + random_fluctuation;
        }

        else{
            sensor.current_temp -= TEMP_CHANGE_RATE + random_fluctuation;
        }
    }
}

