use crate::component::TemperatureSensor;
use crate::component::Thermostat;
use crate::component::HeaterMode;
use std::collections::HashMap;


pub type Entity = u32;

pub struct World {
    pub sensors: HashMap<Entity, TemperatureSensor>,
    pub thermostats: HashMap<Entity, Thermostat>,
    pub heater_modes: HashMap<Entity, HeaterMode>,
    pub next_id: Entity,
}

impl World{
    pub fn new() -> Self{
        World{
            sensors: HashMap::new(),
            thermostats: HashMap::new(),
            heater_modes: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_sensor(&mut self, id: Entity, sensor: TemperatureSensor){
        self.sensors.insert(id, sensor);
    }
    pub fn add_thermostat(&mut self, id: Entity, thermostat: Thermostat){
        self.thermostats.insert(id, thermostat);
    }
    pub fn add_heater_mode(&mut self, id: Entity, heater_mode: HeaterMode){
        self.heater_modes.insert(id, heater_mode);
    }
    pub fn spawn(&mut self) -> Entity {
        self.next_id +=1;
        return self.next_id - 1;
    }

    pub fn query_sensor_thermostat(&self) -> impl Iterator<Item = (Entity, &TemperatureSensor, &Thermostat)>{
        self.sensors.iter().filter_map(move |(&id, sensor)| {
            self.thermostats.get(&id).map(|thermostat| (id, sensor, thermostat))
        })
    }
}