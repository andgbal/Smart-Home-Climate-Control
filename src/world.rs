use crate::component::TemperatureSensor;
use crate::component::Thermostat;
use std::collections::HashMap;

pub type Entity = u32;

pub struct World {
    pub sensors: HashMap<Entity, TemperatureSensor>,
    pub thermostats: HashMap<Entity, Thermostat>,
    pub next_id: Entity,
}

impl World{
    pub fn new() -> Self{
        World{
            sensors: HashMap::new(),
            thermostats: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_sensor(&mut self, id: Entity, sensor: TemperatureSensor){
        self.sensors.insert(id, sensor);
    }
    pub fn add_thermostat(&mut self, id: Entity, thermostat: Thermostat){
        self.thermostats.insert(id, thermostat);
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