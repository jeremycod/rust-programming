use std::collections::HashMap;
use std::rc::Rc;

mod alerting;

#[derive(Debug)]
struct GreenhouseConfig {
    settings: HashMap<String, String>,
}

impl GreenhouseConfig {
    fn get_setting(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }
}

fn main() {
    // Initial configuration loaded when the app starts
    let initial_config = GreenhouseConfig {
        settings: HashMap::from([
            ("temperature_target".to_string(), "22C".to_string()),
            ("humidity_level".to_string(), "75%".to_string()),
            ("light_schedule".to_string(), "16/8".to_string()),
        ]),
    };

    let shared_config = Rc::new(initial_config);

    // Multiple parts of the application get a shared read-only view of the config
    let sensor_module_config = Rc::clone(&shared_config);
    let ui_module_config = Rc::clone(&shared_config);
    let logging_module_config = Rc::clone(&shared_config);

    println!("Initial strong count: {}", Rc::strong_count(&shared_config)); // Output: 4 (main, sensor, ui, logging)

    // Simulate the sensor module reading a setting
    println!(
        "Sensor module temperature target: {:?}",
        sensor_module_config.get_setting("temperature_target")
    );

    // Simulate the UI module displaying the light schedule
    println!(
        "UI module light schedule: {:?}",
        ui_module_config.get_setting("light_schedule")
    );

    // Simulate the logging module recording the humidity level
    println!(
        "Logging module humidity level: {:?}",
        logging_module_config.get_setting("humidity_level")
    );

    // ... later in the application's lifecycle ...

    {
        // A specific component (e.g., a temporary report generator) also needs access
        let report_config = Rc::clone(&shared_config);
        println!("Strong count inside report generator: {}", Rc::strong_count(&shared_config)); // Output: 5
        // The report generator finishes its work, and its reference goes out of scope
    }

    println!("Strong count after report generator finishes: {}", Rc::strong_count(&shared_config)); // Output: 4

    // The application continues to run, with the shared configuration being used
    {
        let alert_config = Rc::clone(&shared_config);

        alerting::alert(&alert_config);
        println!("Strong count in the scope of alerting: {}", Rc::strong_count(&shared_config)); // Output: 4
    }


    println!("Strong count after alerting finishes: {}", Rc::strong_count(&shared_config)); // Output: 4

} // When main ends, shared_config and all its clones go out of scope, and the GreenhouseConfig is dropped.