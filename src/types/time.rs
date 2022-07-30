#[derive(PartialEq, Debug, Clone)]
pub struct Time {
    pub soc: u32,
    pub fracsec: u24,
    pub leap_second_direction: bool,
    pub leap_second_occured: bool,
    pub leap_second_pending: bool,
    pub time_quality: TimeQuality,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TimeQuality {
    Fault,    //Fault- clock failure, time not reliable
    UTC10s,   //Time within 10s of UTC
    UTC1s,    //Time within 1s of UTC
    UTC100ms, //Time within 100ms of UTC
    UTC10ms,  //Time within 10ms of UTC
    UTC1ms,   //Time within 1ms of UTC
    UTC100us, //Time within 100us of UTC
    UTC10us,  //Time within 10us of UTC
    UTC1us,   //Time within 1us of UTC
    UTC100ns, //Time within 100ns of UTC
    UTC10ns,  //Time within 10ns of UTC
    UTC1ns,   //Time within 1ns of UTC
    Locked,   //Normal operation, clock locked to UTC traceable source
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Clone)]
pub struct u24(u32);

impl u24 {
    pub fn new(i: u32) -> Result<u24, TimeError> {
        let base: u32 = 2;
        if i < (base.pow(24)) {
            Ok(u24(i))
        } else {
            //FRACSEC: Fracsec value much greater than 2^24 -1
            Err(TimeError::U24TypeRangeOverflow)
        }
    }
    pub fn encode(&self) -> u32 {
        self.0
    }
}

#[derive(PartialEq, Debug)]
pub enum TimeError {
    U24TypeRangeOverflow,
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn u24_exceeds_allowed_size() {
        assert_eq!(u24::new(0xFF123456), Err(TimeError::U24TypeRangeOverflow));
    }
}
