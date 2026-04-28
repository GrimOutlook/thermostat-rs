fn main() {
    println!("Hello, world!");
}

enum Mode {
    /// Thermostat attemps to maintain a temperature at or below the given
    /// value.
    Cool(i16),
    /// Thermostat attemps to maintain a temperature at or above the given
    /// value.
    Heat(i16),
    /// Thermostat attempts to maintain the temperature inside a range of
    /// desired values.
    Range { min: i16, max: i16 },
    /// Thermostat takes no action to control temperature.
    Off,
}

struct Thermostat {
    /// Operating mode. Determines if heating or cooling is allowed.
    mode: Mode,
    /// Amount of time to wait between deactivating and reactivating a given
    /// component.
    reactivation_delay: u32,
}
