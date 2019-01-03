// TODO: remove
#[allow(dead_code)]
pub struct BabysittingJob {
    start_time: i32,
    end_time: i32,
}

// TODO: remove
#[allow(dead_code)]
impl BabysittingJob {
    fn is_valid(self) -> bool {
        ( self.end_time <= 4  || ( 17 < self.end_time && self.end_time <= 23 ) ) &&
        ( self.start_time >= 17 || ( 0 <= self.start_time  && self.start_time < 4 ) )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_time_after_5pm_is_valid() {
        let job = BabysittingJob { start_time: 17, end_time: 19 };
        assert!(job.is_valid());
    }

    #[test]
    fn start_time_before_5pm_is_invalid() {
        let job = BabysittingJob { start_time: 16, end_time: 20 };
        assert!(!job.is_valid());
    }

    #[test]
    fn end_time_before_4am_is_valid() {
        let job = BabysittingJob { start_time: 0, end_time: 3 };
        assert!(job.is_valid());
    }

    #[test]
    fn end_time_after_4am_is_invalid() {
        let job = BabysittingJob { start_time: 0, end_time: 5 };
        assert!(!job.is_valid());
    }

}
