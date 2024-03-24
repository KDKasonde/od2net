use lts::Tags;
use regex::Regex;

#[derive(PartialEq)]
enum SpeedMeasure {
    MPH,
    KPH,
    KNOTS,
    Other
}

impl SpeedMeasure {
    // assumption here that osm data is pretty clean 
    // hopefully this stands
    fn parse_speed_measure(speed_measure: String) -> SpeedMeasure {
        let km_checker = Regex::new(
            r"(?:[ ]?(?:km/h|kmh|kph))?"
            )
            .ok()?;

        let mile_checker = Regex::new(
            r"(?:[ ]?(?:mphs))?"
            )
            .ok()?;

        let knots_checker = Regex::new(
            r"(?:[ ]?(?:knots))?"
            )
            .ok()?;
        
        if km_checker.is_match(&speed_measure) {
            return SpeedMeasure::KPH 
        }

        if mile_checker.is_match(&speed_measure) {
            return SpeedMeasure::MPH 
        }
            .
        if knot_checker.is_match(&speed_measure) {
            return SpeedMeasure::KNOTS 
        }

        return SpeedMeasure::Other
    }
}
    



struct MaxSpeed {
    speed: f32, 
    speed_measure: SpeedMeasure
}

impl MaxSpeed {

    // The function's regex is taken from
    // https://wiki.openstreetmap.org/wiki/Key:maxspeed    
    fn parse_tag(osm_speed: String) -> Option<MaxSpeed> {

        let regex_for_numerical_speed = Regex::new(
            r"^([0-9][\.0-9]+?)(?:[ ]?(?:km/h|kmh|kph|mph|knots))?"
            )
            .ok()?;

        let numercal_speed = regex_for_numerical_speed
            .find(&osm_speed)?
            .as_str();

        let regex_for_speed = Regex::new(
            r"([0-9][\.0-9]+?)"
            )
            .ok()?;

        let speed = regex_for_speed
            .find(&numercal_speed)?
            .as_str();

        Some(
            MaxSpeed { 
                speed: speed.parse::<f32>().ok()?, 
                speed_measure: SpeedMeasure::parse_speed_measure(numercal_speed.to_string()) 
            }
        )
    }

    fn convert_km_mi(&mut self) {
        if self.speed_measure == SpeedMeasure::KPH {
            self.speed *= 0.621371;
            self.speed_measure = SpeedMeasure::MPH;
        }
    }

    fn convert_mi_km(&mut self) {
        if self.speed_measure == SpeedMeasure::KPH {
            self.speed *=1.60934;
            self.speed_measure = SpeedMeasure::MPH;
        }
    }
}

pub fn is_highway(tags: Tags) -> bool {
    let is_highway_tag = if tags.is_any(
        "highway",
        vec![
            "primary",
            "motorway"
        ],
    ) {
        true
    } else { 
        false 
    };

    let is_speed_high = if !tags.has("maxspeed") {
        false 
    } else {
        
        let is_high_speed = 

    };

}
