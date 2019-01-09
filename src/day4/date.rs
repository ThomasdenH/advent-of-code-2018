#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

impl Date {
    pub fn minutes_diff(self, other: Date) -> u8 {
        assert_eq!(self.year, other.year);
        assert_eq!(self.month, other.month);
        assert_eq!(self.day, other.day);
        assert_eq!(self.hour, other.hour);
        self.minute - other.minute
    }
}
