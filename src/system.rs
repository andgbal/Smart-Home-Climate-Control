use std::collections::HashMap;
use rand::Rng;

pub fn climate_control_system(home: &mut SmartHome){
    for (&room_id, sensor) in home.sensors.iter() {
        if let some(thermostat) = home.thermostat.get_mut(&room_id){
            if (sensor.current_temp - home.TemperatureHistory.get(-5)) < target_temp{
                if thermostat.heater < 3{
                    thermostat.heater += 1;
                    println!("Room {}'s heater is turning ON to ${}", room_id, thermostat.heater);
                } else {
                    println!("Room {}'s heater is already operating at strongest mode!", room_id);
                }
            }

            else{
                if thermostat.heater > 0{
                    thermostat.heater_on = 0;
                    println!("Room {}'s heater is turning OFF!", room_id);
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

