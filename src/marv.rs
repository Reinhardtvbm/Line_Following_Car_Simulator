use std::{
    f64::consts::PI,
    io::Write,
    time::{Duration, SystemTime},
};

use crate::pid::Pid;

#[derive(Debug)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Marv {
    axle_length: f64,
    sensor_radius: f64,
    wheel_diameter: f64,
    axle_center: Coord,
    sensor_positions: Vec<Coord>,
    line_pos: i8,
    pid: Pid,
}

impl Marv {
    pub fn new(
        axle_length: f64,
        sensor_radius: f64,
        wheel_diameter: f64,
        axle_center: Coord,
    ) -> Self {
        let positions = [
            PI / 12.0,
            PI / 18.0,
            PI / 36.0,
            0.0,
            -PI / 36.0,
            -PI / 18.0,
            -PI / 12.0,
        ]
        .into_iter()
        .map(|angle| -> Coord {
            Coord {
                x: sensor_radius * angle.cos(),
                y: sensor_radius * angle.sin(),
            }
        })
        .collect();

        Self {
            axle_length,
            sensor_radius,
            wheel_diameter,
            axle_center,
            sensor_positions: positions,
            pid: Pid::new(200.0, 80.0, 0.0, 500.0, 120.0, -120.0),
            line_pos: 0,
        }
    }

    pub fn run(&mut self, duration: Duration, frequency: f64) {
        // strings for output to files
        let mut string_output = String::new();
        let mut sensor_strings = vec![String::from(""); 7];
        let mut pid_vals = String::new();

        let period = ((1.0 / frequency) * 1_000_000_000.0) as u128;
        let perios_secs = 1.0 / frequency;
        let cycles = duration.as_nanos() / period;

        let mut angle: f64 = 0.0;
        let mut pid_result = 0.0;

        let pid_freq = frequency / 1000.0;

        for i in 0..cycles {
            let angular_velocity = (pid_result * self.wheel_diameter) / self.axle_length;

            let velocity = 120.0 - (pid_result.abs() / 2.0);

            angle = angle + angular_velocity * perios_secs;

            if i % pid_freq as u128 == 0 {
                pid_result = self.pid.calculate(self.line_pos as f64 * PI / 36.0);
            }

            pid_vals.push_str(format!("{},{}\n", self.axle_center.x, pid_result).as_str());

            let delta_x = velocity * angle.cos();
            let delta_y = velocity * angle.sin();

            let add_x = perios_secs * delta_x;
            let add_y = perios_secs * delta_y;

            string_output
                .push_str(format!("{},{}\n", self.axle_center.x, self.axle_center.y).as_str());

            sensor_strings
                .iter_mut()
                .zip(self.sensor_positions.iter())
                .for_each(|(sensor_string, sensor)| {
                    sensor_string.push_str(format!("{},{}\n", sensor.x, sensor.y).as_str());
                });

            self.axle_center.x += add_x;
            self.axle_center.y += add_y;

            let indices = vec![3, 4, 2, 5, 1, 6, 0];

            self.sensor_positions
                .iter_mut()
                .zip(indices.iter())
                .for_each(|(sensor, &index)| {
                    sensor.x = (self.sensor_radius
                        * (((index as f64 - 3.0) * (PI / 36.0)) + angle).cos())
                        + self.axle_center.x;

                    sensor.y = (self.sensor_radius
                        * (((index as f64 - 3.0) * (PI / 36.0)) + angle).sin())
                        + self.axle_center.y;

                    let line_y = 50.0 * (0.05 * sensor.x - 1.0).cos() - 50.0;

                    if (sensor.y - line_y).abs() < 0.3 {
                        self.line_pos = index as i8 - 3;
                    }
                });
        }

        let mut out_file = std::fs::File::create("output.txt").unwrap();

        out_file.write(string_output.as_bytes());

        let mut pid_file = std::fs::File::create("pid_out.txt").unwrap();

        pid_file.write(pid_vals.as_bytes());

        for (index, string) in sensor_strings.iter().enumerate() {
            let mut file = std::fs::File::create(format!("sensor{}.txt", index)).unwrap();

            file.write(string.as_bytes());
        }
    }
}
