use std::collections::HashMap;
use crate::component::*;

pub type RoomID = usize;

pub struct SmartHome {
    pub sensors: HashMap<RoomID, TemperatureSensor>,
    pub thermostat: HashMap<RoomID, Thermostat>,
    pub heater_modes: HashMap<RoomID, HeaterMode>,
    pub history: HashMap<RoomID, TemperatureHistory>,
}

impl SmartHome {
    pub fn new() -> Self {
        Self {
            sensors: HashMap::new(),
            thermostat: HashMap::new(),
            heater_modes: HashMap::new(),
            history: HashMap::map(),
        }
    }
}