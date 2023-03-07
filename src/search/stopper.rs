use super::report::Report;

pub trait Stopper {
    fn should_stop(&self, report: &Report) -> bool;
    fn max_depth(&self) -> Option<u8>;
}
