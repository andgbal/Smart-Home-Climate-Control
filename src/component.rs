use std::collections::HashMap;

pub struct TemperatureSensor {
    pub current_temp: f32,
}

pub struct TemperatureHistory {
    pub history: Vec<(u64,f32)>
}

pub struct Thermostat {
    pub target_temp: f32,
}

pub struct HeaterMode {
    pub level: u8,
}
