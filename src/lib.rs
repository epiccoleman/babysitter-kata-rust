#![feature(range_contains)]

use std::collections::HashMap;
use std::ops::Range;

#[allow(dead_code)]
pub struct BabysittingJob {
    start_time: i32,
    end_time: i32,
}

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

    fn calculate_pay(&self, family: &Family) -> i32 {
        let mut pay = 0;
        if self.start_time < self.end_time {
            for hour in self.start_time..self.end_time {
                pay += &family.rate_for_hour(hour)
            }
        } else {
            for hour in self.start_time..24 {
                pay += &family.rate_for_hour(hour)
            }
            for hour in 0..self.end_time {
                pay += &family.rate_for_hour(hour)
            }
        }
        pay
    }
}

#[allow(dead_code)]
struct Family {
    name: String,
    rates: HashMap<Range<i32>, i32>,
}

#[allow(dead_code)]
impl Family {
    fn new(name: String, rates: HashMap<Range<i32>, i32>) -> Family {
        Family { name, rates }
    }

    fn rate_for_hour(&self, hour: i32) -> i32 {
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

    #[test]
    fn family_pays_0_for_undefined_hours() {
        let rates = HashMap::<Range<i32>, i32>::new();

        let family = Family::new("foo".to_string(), rates);
        assert_eq!(family.rate_for_hour(17), 0);
    }

    #[test]
    fn correctly_calculates_pay_when_job_does_not_cross_day_boundary() {
        let mut rates = HashMap::<Range<i32>, i32>::new();
        rates.insert(17..20, 10);

        let family = Family::new("foo".to_string(), rates);

        let job = BabysittingJob {
            start_time: 17,
            end_time: 19,
        };

        assert_eq!(job.calculate_pay(&family), 20);
    }

    #[test]
    fn correctly_calculates_pay_when_job_crosses_day_boundary() {
        let mut rates = HashMap::<Range<i32>, i32>::new();
        rates.insert(20..24, 10);
        rates.insert(0..4, 10);

        let family = Family::new("foo".to_string(), rates);

        let job = BabysittingJob {
            start_time: 22,
            end_time: 2,
        };

        assert_eq!(job.calculate_pay(&family), 40);
    }

    #[test]
    fn correctly_handles_pay_rate_changes(){
        let mut rates = HashMap::<Range<i32>, i32>::new();
        rates.insert(17..19, 15);
        rates.insert(19..24, 20);

        let family = Family::new("foo".to_string(), rates);

        let job = BabysittingJob {
            start_time: 18, end_time: 20,
        };

        assert_eq!(job.calculate_pay(&family), 35);

    }

    #[test]
    fn test_family_a(){
        // Family A pays $15 per hour before 11pm, and $20 per hour the rest of the night
        let mut rates = HashMap::<Range<i32>, i32>::new();
        rates.insert(17..23, 15);
        rates.insert(23..24, 20);
        rates.insert(0..4, 20);

        let family = Family::new("A".to_string(), rates);

        let job = BabysittingJob {
            start_time: 18,
            end_time: 2,
        };

        assert_eq!(job.calculate_pay(&family), 135);
    }

    #[test]
    fn test_family_b(){
        // Family B pays $12 per hour before 10pm, $8 between 10 and 12, and $16 the rest of the night
        let mut rates = HashMap::<Range<i32>, i32>::new();
        rates.insert(17..22, 12);
        rates.insert(22..24, 8);
        rates.insert(0..4, 16);

        let family = Family::new("B".to_string(), rates);

        let job = BabysittingJob {
            start_time: 18,
            end_time: 2,
        };

        assert_eq!(job.calculate_pay(&family), 96);
    }

    #[test]
    fn test_family_c(){
        // Family C pays $21 per hour before 9pm, then $15 the rest of the night
        let mut rates = HashMap::<Range<i32>, i32>::new();
        rates.insert(17..21, 21);
        rates.insert(21..24, 15);
        rates.insert(0..4, 15);

        let family = Family::new("C".to_string(), rates);

        let job = BabysittingJob {
            start_time: 17,
            end_time: 23,
        };

        assert_eq!(job.calculate_pay(&family), 114);
    }
}
