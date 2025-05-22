#[derive(Debug)]
enum SensorReading {
    Temperature(i32),
    Error(String),
    NoReading,
}
fn analyze_reading(sensor_reading: SensorReading){
   match sensor_reading {
       SensorReading::Error (error) => println!("Sensor error detected: '{}'", error),
       SensorReading::NoReading => println!("No sensor data available."),
       SensorReading::Temperature(temp) if temp < 0 =>println!("Temperature is freezing: {}°C", temp),
       SensorReading::Temperature(temp) if temp > 0 && temp <= 25=> println!("Temperature is normal: {}°C", temp),
       SensorReading::Temperature(temp) if temp > 25=> println!("Temperature is hot: {}°C", temp),
       _ => println!("Unsupported sensor {:?}", sensor_reading)

   }
}
fn main () {
    analyze_reading(SensorReading::Temperature(-5));
    analyze_reading(SensorReading::Temperature(15));
    analyze_reading(SensorReading::Temperature(30));
    analyze_reading(SensorReading::Error(String::from("Failed to connect")));
    analyze_reading(SensorReading::NoReading);
}