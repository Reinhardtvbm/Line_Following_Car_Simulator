#[derive(Debug)]
pub struct Pid {
    k_p: f64,
    k_i: f64,
    k_d: f64,
    frequency: f64,
    prev_error: f64,
    integral: f64,
    max_val: f64,
    min_val: f64,
    integral_on: bool,
}

impl Pid {
    pub fn new(k_p: f64, k_i: f64, k_d: f64, frequency: f64, max_val: f64, min_val: f64) -> Self {
        Self {
            k_p,
            k_i,
            k_d,
            frequency,
            prev_error: 0.0,
            integral: 0.0,
            max_val,
            min_val,
            integral_on: true,
        }
    }

    pub fn calculate(&mut self, error: f64) -> f64 {
        if self.integral_on {
            self.integral += error * (1.0 / self.frequency);
        }

        let mut output = self.k_p * error
            + self.k_i * self.integral
            + self.k_d * (self.frequency * (error - self.prev_error));

        self.prev_error = error;

        if (output > self.max_val || output < self.min_val)
            && (output / output.abs() == self.integral / self.integral.abs())
        {
            self.integral_on = false;
        } else {
            self.integral_on = true;
        }

        if output > self.max_val {
            output = self.max_val;
        } else if output < self.min_val {
            output = self.min_val;
        }

        output
    }
}
