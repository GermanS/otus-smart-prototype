struct UnpluggedSocket<'a> {
    location: &'a str,
}

impl<'a> UnpluggedSocket<'a> {
    fn new(location: &'a str) -> Self {
        Self { location }
    }

    fn on(&self) -> PluggedSocket {
        PluggedSocket::new(self.location)
    }

    fn info(&self) {
        println!("{}", self._status());
    }

    fn _status(&self) -> String {
        format!("Location: {}, State: Off", self.location)
    }
}

struct PluggedSocket<'a> {
    location: &'a str,
}

impl<'a> PluggedSocket<'a> {
    fn new(location: &'a str) -> Self {
        Self { location }
    }

    fn wattage(&self) -> f32 {
        42.0
    }

    fn off(&self) -> UnpluggedSocket {
        UnpluggedSocket::new(self.location)
    }

    fn info(&self) {
        println!("{}", self._status());
    }

    fn _status(&self) -> String {
        format!(
            "Location: {}, State: On, Wattage: {}",
            self.location,
            self.wattage()
        )
    }
}

struct Thermometer(f32);

impl Thermometer {
    fn new(degrees: f32) -> Self {
        Self(degrees)
    }

    fn temperature(&self) -> f32 {
        self.0
    }

    fn info(&self) {
        println!("Current temp: {}", self.temperature());
    }
}

fn main() {
    let socket = UnpluggedSocket::new("kitchen");
    socket.info();

    let socket = socket.on();
    socket.info();
    socket.wattage();

    let socket = socket.off();
    socket.info();

    let thermometer = Thermometer::new(36.7);
    thermometer.info()
}
