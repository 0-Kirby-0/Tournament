#![allow(dead_code)]

#[derive(Clone, Copy, Default)]
pub struct Hsl {
    pub hue: u8,
    pub saturation: u8,
    pub lightness: u8,
}
impl Hsl {
    pub fn new(hue: u8, saturation: u8, lightness: u8) -> Self {
        Hsl {
            hue,
            saturation,
            lightness,
        }
    }
    fn normalised(&self) -> (f64, f64, f64) {
        (
            byte_to_percent(self.hue),
            byte_to_percent(self.saturation),
            byte_to_percent(self.lightness),
        )
    }
    fn from_normalised(hue: f64, saturation: f64, lightness: f64) -> Self {
        Self {
            hue: percent_to_byte(hue),
            saturation: percent_to_byte(saturation),
            lightness: percent_to_byte(lightness),
        }
    }
    fn distance(&self, other: &Self) -> u32 {
        let mut total_distance = 0;

        total_distance += std::cmp::min(
            self.hue.wrapping_sub(other.hue),
            other.hue.wrapping_sub(self.hue),
        ) as u32;

        if (self.lightness <= u8::MIN + 1) | (self.lightness >= u8::MAX - 1) {
            //pointless, because lightness is so low, we're expecting precision drops
        } else {
            total_distance += self.saturation.abs_diff(self.saturation) as u32
        }

        total_distance += self.lightness.abs_diff(other.lightness) as u32;

        total_distance
    }
}

impl std::fmt::Display for Hsl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:02X}][{:02X}][{:02X}]",
            self.hue, self.saturation, self.lightness
        )
    }
}

impl std::fmt::Debug for Hsl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Hue: {:3.0}°, Saturation: {:3.0}%, Lightness: {:3.0}%]",
            byte_to_percent(self.hue) * 3600.,
            byte_to_percent(self.saturation) * 100.,
            byte_to_percent(self.lightness) * 100.
        )
    }
}

#[derive(Clone, Copy, Default)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}
impl Rgb {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }

    fn normalised(&self) -> (f64, f64, f64) {
        (
            byte_to_percent(self.red),
            byte_to_percent(self.green),
            byte_to_percent(self.blue),
        )
    }
    fn from_normalised(red: f64, green: f64, blue: f64) -> Self {
        Self {
            red: percent_to_byte(red),
            green: percent_to_byte(green),
            blue: percent_to_byte(blue),
        }
    }
    fn distance(&self, other: &Self) -> u32 {
        let mut total_distance = 0;

        total_distance += self.red.abs_diff(other.red) as u32;
        total_distance += self.green.abs_diff(other.green) as u32;
        total_distance += self.blue.abs_diff(other.blue) as u32;

        total_distance
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:02X}][{:02X}][{:02X}]",
            self.red, self.green, self.blue
        )
    }
}
impl std::fmt::Debug for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Red: {:3}, Green: {:3}, Blue: {:?}]",
            self.red, self.green, self.blue
        )
    }
}

/* The below is shamelessly stolen and adapted from https://docs.rs/hsl/, go there for great explanations */
impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Self {
        let (red, green, blue) = rgb.normalised();
        let max = red.max(green.max(blue));
        let min = red.min(green.min(blue));

        // Lightness is the average of the max and min rgb color intensities.
        let lightness = (max + min) / 2.;

        // Delta is the difference between the most and least intense rbg value.
        let delta = max - min;

        // Early return: The color is a shade of gray
        if delta < f64::MIN_POSITIVE {
            return Hsl::from_normalised(0., 0., lightness);
        }

        // Saturation depends on Delta, but works differently depending on Lightness
        let saturation = if lightness < 0.5 {
            delta / (max + min) // Below .5 Lightness, Saturation increases as we move away from pure black.
        } else {
            delta / (2. - max - min) // Above .5 Lightness, Saturation decreases as we move towards pure white.
        };

        // Intermediary Hue, stating how far each value is from the maximum, normalised and smoothed.
        let red_i = (((max - red) / 6.) + (delta / 2.)) / delta;
        let green_i = (((max - green) / 6.) + (delta / 2.)) / delta;
        let blue_i = (((max - blue) / 6.) + (delta / 2.)) / delta;

        // Calculate Hue, depending on which value was the max
        let hue = if max == red {
            blue_i - green_i
        } else if max == green {
            (1. / 3.) + red_i - blue_i
        } else if max == blue {
            (2. / 3.) + green_i - red_i
        } else {
            unreachable!("Max is none of the hues? How'd that happen?")
        };

        // Fix wraparounds
        let hue = if hue < 0. {
            hue + 1.
        } else if hue > 1. {
            hue - 1.
        } else {
            hue
        };

        Hsl {
            hue: percent_to_byte(hue),
            saturation: percent_to_byte(saturation),
            lightness: percent_to_byte(lightness),
        }
    }
}
impl From<Hsl> for Rgb {
    fn from(hsl: Hsl) -> Self {
        // Color is grey, so rgb values are just Lightness
        if hsl.saturation == 0 {
            return Rgb {
                red: hsl.lightness,
                green: hsl.lightness,
                blue: hsl.lightness,
            };
        }

        let (hue, saturation, lightness) = hsl.normalised();

        // Upper Bound of the RGB values
        let upper = if lightness < 0.5 {
            lightness * (1. + saturation) // Below .5 lightness, color moves from black, q increases brightness by scaling with saturation
        } else {
            lightness + saturation - (lightness * saturation) //Above .5 lightness, color moves towards white, q adjusts to account for brightness without losing color
        };

        //Lower Bound of the RGB values
        let lower = 2. * lightness - upper;

        fn hue_to_rgb(lower: f64, upper: f64, hue: f64) -> f64 {
            // Fix wraparounds
            let t = if hue < 0. {
                hue + 1.
            } else if hue > 1. {
                hue - 1.
            } else {
                hue
            };

            //this is beyond me, sorry
            if t < 1. / 6. {
                lower + (upper - lower) * 6. * t
            } else if t < 1. / 2. {
                upper
            } else if t < 2. / 3. {
                lower + (upper - lower) * (2. / 3. - t) * 6.
            } else {
                lower
            }
        }

        Rgb {
            red: percent_to_byte(hue_to_rgb(lower, upper, hue + 1.0 / 3.0)),
            green: percent_to_byte(hue_to_rgb(lower, upper, hue)),
            blue: percent_to_byte(hue_to_rgb(lower, upper, hue - 1.0 / 3.0)),
        }
    }
}

fn byte_to_percent(u8: u8) -> f64 {
    u8 as f64 / u8::MAX as f64
}
fn percent_to_byte(f64: f64) -> u8 {
    (f64 * (u8::MAX as f64)).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let mut total_error = 0;
        for chan_1 in u8::MIN..=u8::MAX {
            for chan_2 in u8::MIN..=u8::MAX {
                for chan_3 in u8::MIN..=u8::MAX {
                    //even though I'd like to do the opposite conversion first,
                    //calculating a proper distance between HSL values
                    //that reflects their real difference in color is surprisingly difficult
                    let rgb_1: Rgb = Rgb::new(chan_1, chan_2, chan_3);
                    let hsl_1: Hsl = rgb_1.into();
                    let rgb_2: Rgb = hsl_1.into();
                    let hsl_2: Hsl = rgb_2.into();
                    let rgb_3: Rgb = hsl_2.into();
                    let error = rgb_1.distance(&rgb_3);
                    total_error += error;

                    if error > 5 {
                        //as it happens, the highest error I've seen is 4, which I call good enough :)
                        println!(
                            "Off by {:3}: [{}] ->  [{}] -> [{}]",
                            error, rgb_1, rgb_2, rgb_3
                        );
                    }
                }
            }
        }
        let average_error = total_error as f64 / (255 * 255 * 255) as f64;
        println!("Average error: {}", average_error);
    }
}
