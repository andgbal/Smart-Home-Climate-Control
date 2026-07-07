mod component;
mod world;
use crate::component::TemperatureSensor;
use crate::component::Thermostat;

fn main() {
    let mut world = world::World::new();
    
    for i in 1..=3 {
        let id = world.spawn();
        let sensor = TemperatureSensor { current_temp: 20.0 + i as f32 };
        
        world.add_sensor(id, sensor);
        if i != 2{
            let thermostat = Thermostat { target_temp: 20.0 - i as f32 };
            world.add_thermostat(id,thermostat)
        }
    }

    for (id, sensor, thermostat) in world.query_sensor_thermostat() {
        println!("Sensor {} is of {} degree *with target temperature {}*", id, sensor.current_temp, thermostat.target_temp);
    }
}
