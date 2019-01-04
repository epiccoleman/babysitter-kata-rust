#![feature(range_contains)]

use std::collections::HashMap;
use std::ops::Range;

// TODO: remove
#[allow(dead_code)]
pub struct BabysittingJob {
    start_time: i32,
    end_time: i32,
}

// TODO: remove
#[allow(dead_code)]
impl BabysittingJob {
    fn is_valid(&self) -> bool {
        self.times_within_working_hours() && self.start_time_is_before_end_time()
    }

    fn times_within_working_hours(&self) -> bool {
        ((18..24).contains(&self.end_time) || (0..5).contains(&self.end_time))
            && ((17..24).contains(&self.start_time) || (0..4).contains(&self.start_time))
    }

    fn start_time_is_before_end_time(&self) -> bool {
        (self.start_time < self.end_time) || (self.end_time <= 4 && self.start_time <= 23)
    }

    fn calculate_pay(family: &Family) -> i32 {
        //range of values for hours worked
        //map a family function across it
        0
    }
}

#[allow(dead_code)]
struct Family {
    name: String,
    rates: HashMap<Range<i32>, i32>
}

#[allow(dead_code)]
impl Family {
    fn new(name: String, rates: HashMap<Range<i32>, i32>) -> Family {
        Family {
            name,
            rates
        }
    }

    fn rate_for_hour(&self, hour: i32) -> i32{
        let mut rate = 0;
        for (time_range, pay_rate) in &self.rates {
            if time_range.contains(&hour) {
                rate = *pay_rate;
            }
        }
        rate
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_time_after_5pm_is_valid() {
        let job = BabysittingJob {
            start_time: 17,
            end_time: 19,
        };
        assert!(job.is_valid());
    }

    #[test]
    fn start_time_before_5pm_is_invalid() {
        let job = BabysittingJob {
            start_time: 16,
            end_time: 20,
        };
        assert!(!job.is_valid());
    }

    #[test]
    fn end_time_before_4am_is_valid() {
        let job = BabysittingJob {
            start_time: 0,
            end_time: 3,
        };
        assert!(job.is_valid());
    }

    #[test]
    fn end_time_after_4am_is_invalid() {
        let job = BabysittingJob {
            start_time: 0,
            end_time: 5,
        };
        assert!(!job.is_valid());
    }

    #[test]
    fn start_time_before_end_time_is_valid() {
        let job = BabysittingJob {
            start_time: 17,
            end_time: 19,
        };
        assert!(job.is_valid());
    }

    #[test]
    fn start_time_before_end_time_after_midnight_is_valid() {
        let job = BabysittingJob {
            start_time: 17,
            end_time: 3,
        };
        assert!(job.is_valid());
    }

    #[test]
    fn start_time_after_end_time_is_invalid() {
        let job = BabysittingJob {
            start_time: 20,
            end_time: 17,
        };
        assert!(!job.is_valid());
    }

    #[test]
    fn family_pays_correct_rate_for_a_given_hour() {
        let mut rates = HashMap::<Range<i32>, i32>::new();
        rates.insert(17..19, 10);

        let family = Family::new("foo".to_string(), rates);
        assert_eq!(family.rate_for_hour(17), 10);
    }

    fn family_pays_0_for_undefined_hours() {
        let rates = HashMap::<Range<i32>, i32>::new();

        let family = Family::new("foo".to_string(), rates);
        assert_eq!(family.rate_for_hour(17), 0);
    }

}
