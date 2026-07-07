use std::collections::HashMap;

pub type RoomID = u64;

pub struct TemperatureSensor {
    pub current_temp: f32,
}

pub struct TemperatureHistory {
    pub readings: Vec<(u64,f32)>
}

pub struct Thermostat {
    pub target_temp: f32,
}

pub struct HeaterMode {
    pub heater_mode: u8,
}
