use lts::Tags;
use regex::Regex;

#[derive(PartialEq, Debug)]
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
        println!("{}", &speed_measure);
        let km_checker = vec!["km/h", "kmh", "kph"];

        let mile_checker = "mph";

        let knots_checker = "knots";
       
        for match_str in km_checker {
            if speed_measure.contains(match_str) {
                return SpeedMeasure::KPH
            }

        }

        if speed_measure.contains(mile_checker) {
            return SpeedMeasure::MPH 
        }
            
        if speed_measure.contains(knots_checker) {
            return SpeedMeasure::KNOTS 
        }

        SpeedMeasure::Other
    }
}
    


#[derive(PartialEq)]
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
    
    // these may end up being removed I just thought
    // having them is better than not.
    fn convert_to_mi(&mut self) -> Result<(),()>{
        match self.speed_measure {
            SpeedMeasure::KPH => {
                self.speed *= 0.621371;
                self.speed_measure = SpeedMeasure::MPH;
                Ok(())
            },
            SpeedMeasure::KNOTS => {
                self.speed *= 1.15;
                self.speed_measure = SpeedMeasure::MPH;
                Ok(())
            },
            SpeedMeasure::MPH => {
                Ok(())  
            },
            _ => {
                Err(())
            }
        }
    }

    fn convert_to_km(&mut self) -> Result<(),()>{
        match self.speed_measure {
            SpeedMeasure::MPH => {
                self.speed *= 0.621371;
                self.speed_measure = SpeedMeasure::KPH;
                Ok(())
            },
            SpeedMeasure::KNOTS => {
                self.speed *= 1.852;
                self.speed_measure = SpeedMeasure::KPH;
                Ok(())
            },
            SpeedMeasure::KPH => {
                Ok(())  
            },
            _ => {
                Err(())
            }
        }
    }
}

// so far this only catches maxspeeds which have the type
// numeric speed limit. Converting to miles so far since UK uses miles
// however maybe this should be a user config?
// any errors will resolve as false.
pub fn is_highway(tags: Tags) -> bool {
    if tags.is_any("highway",vec![ "primary","motorway"]) {
        return true
    }

    if !tags.has("maxspeed") {
        return false 
    } 
    
    let tag_value = if let Some(value) = tags.get("maxspeed") {
        value
    } else {
        return false
    };

    let mut max_speed = if let Some(max) = MaxSpeed::parse_tag(tag_value.to_string()) {
        max 
    } else {
        return false
    };
    
    if !max_speed.convert_to_mi().is_ok() {
        return false 
    }
    
    // I chose 50 mph because I am more aware of mph speeds
    //  maybe this should be a user preference if thats possible?
    if max_speed.speed >= 50. {
        return true 
    }        

    false

}

#[cfg(test)]
mod tests{
    use crate::network::highways::{SpeedMeasure, MaxSpeed};

    #[test]
    fn osm_parse_variations() {
        let osm_example_1 = "60 mph + maxspeed:type=GB:nsl_single";
        let osm_example_2 = "80 kph";
        let osm_example_3 = "70 + maxspeed:conditional=60 @ wet";
        let osm_example_4 = "walk";

        let osm_max_1 = MaxSpeed::parse_tag(osm_example_1.to_string()).unwrap();
        let osm_max_2 = MaxSpeed::parse_tag(osm_example_2.to_string()).unwrap();
        let osm_max_3 = MaxSpeed::parse_tag(osm_example_3.to_string()).unwrap();
        let osm_max_4 = MaxSpeed::parse_tag(osm_example_4.to_string());

        assert_eq!(60.0, osm_max_1.speed);
        assert_eq!(80.0, osm_max_2.speed);
        assert_eq!(70.0, osm_max_3.speed);

        assert_eq!(SpeedMeasure::MPH, osm_max_1.speed_measure, "Test that {:?} and {:?} are the same", SpeedMeasure::MPH, osm_max_1.speed_measure);
        assert_eq!(SpeedMeasure::KPH,  osm_max_2.speed_measure, "Test that {:?} and {:?} are the same", SpeedMeasure::KPH, osm_max_2.speed_measure);
        assert_eq!(SpeedMeasure::Other, osm_max_3.speed_measure, "Test that {:?} and {:?} are the same", SpeedMeasure::Other, osm_max_3.speed_measure);

        assert!(osm_max_4.is_none());
    }
}


        


