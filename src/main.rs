mod component;
mod system;
mod home;

fn main() {
    let mut home = SmartHome::new();
    let mut current_tick: u64 = 0;

    println!("Starting Smart Home Simulation...");

    loop {
        println!("\n--- TICK {}---", current_tick);

        current_tick += 1;

        std::thread::sleep(std::time::Duration::from_secs(1));

        nature_system();
    }
}
