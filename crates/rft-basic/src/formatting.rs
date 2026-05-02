#[cfg(test)]
mod tests {
    use std::fmt::Display;

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    #[test]
    fn print_city() {
        for city in [
            City {
                name: "Tokyo",
                lat: 35.6894,
                lon: 139.6917,
            },
            City {
                name: "New York",
                lat: 40.7128,
                lon: -74.0060,
            },
            City {
                name: "London",
                lat: 51.5074,
                lon: -0.1278,
            },
        ] {
            println!("{}", city);
        }
    }
}
