use std::error::Error;
use std::thread;
use std::time::Duration;

use gpio_cdev::{Chip, LineHandle, LineRequestFlags};

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u32 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let pin = chip
        .get_line(GPIO_LED)?
        .request(LineRequestFlags::OUTPUT, 0, "led")?;

    for i in 1..=160 {
        pulse(
            Duration::from_millis(20),
            (i as f64 * std::f64::consts::FRAC_PI_8 / 4.0).sin().powi(2),
            &pin,
        )?;
    }
    Ok(())
}

fn pulse(length: Duration, brightness: f64, pin: &LineHandle) -> Result<(), Box<dyn Error>> {
    let nanos = length.as_nanos() as u64;
    let nanos_on = ((nanos as f64) * brightness) as u64;
    let nanos_off = nanos - nanos_on;
    pin.set_value(1)?;
    thread::sleep(Duration::from_nanos(nanos_on));
    pin.set_value(0)?;
    thread::sleep(Duration::from_nanos(nanos_off));
    Ok(())
}
