mod world;
mod component;
mod system;
use crate::component::TemperatureHistory;
use crate::component::TemperatureSensor;
use crate::component::Thermostat;
use crate::component::HeaterMode;
use crate::system::climate_control_system;
use crate::system::nature_sytem;
use crate::system::climate_record_system;
use crate::system::print_dashboard;
use crate::system::update_tick;

fn main() {
    let mut world = world::World::new();
    
    for i in 1..=3 {
        let id = world.spawn();
        let sensor = TemperatureSensor { current_temp: 20.0 + i as f32 };
        
        world.add_sensor(id, sensor);
        
        let thermostat = Thermostat { target_temp: 23.0 };
        world.add_thermostat(id, thermostat);
        
        let heater_mode = HeaterMode { level: 0 };
        world.add_heater_mode(id, heater_mode);

        let temperature_history = TemperatureHistory { history: vec![(0, 20.0 + i as f32)]};

        world.add_temperature_history(id, temperature_history);
    }

    loop {
        nature_sytem(&mut world);
        climate_record_system(&mut world);
        climate_control_system(&mut world);
        print_dashboard(&mut world);
        update_tick(&mut world);
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}