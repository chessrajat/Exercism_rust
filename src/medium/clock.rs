use core::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (((hours * (60) + minutes) % (24 * 60)) + (24 * 60)) % (24 * 60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock::new(0, self.minutes + minutes);
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            (self.minutes / 60) % 24,
            self.minutes % 60
        )
    }
}

//Clock
// pub struct Clock {
//     hours: i32,
//     minutes: i32,
// }

// impl Clock {
//     pub fn new(hours: i32, minutes: i32) -> Self {
//         let (m, carry_h) = Self::calculate_minutes(minutes);
//         let h = Self::calculate_hours(hours, carry_h);
//         return Self {
//             hours: h,
//             minutes: m,
//         };
//     }

//     pub fn add_minutes(&self, minutes: i32) -> Self {
//         let (m, carry_h) = Self::calculate_minutes(minutes + &self.minutes);
//         let h = Self::calculate_hours(self.hours, carry_h);
//         return Clock {
//             hours: h,
//             minutes: m,
//         };
//     }

//     fn calculate_minutes(minutes: i32) -> (i32, i32) {
//         let mut m = &minutes % 60;
//         let mut carry_hours = &minutes / 60;
//         if m < 0 {
//             m = 60 + m;
//             carry_hours += -1;
//         }
//         return (m, carry_hours);
//     }

//     fn calculate_hours(hours: i32, carry_hours: i32) -> i32 {
//         let hours = hours + carry_hours;
//         let mut h = hours % 24;

//         if h < 0 {
//             h = 24 + h
//         }
//         return h;
//     }
// }

// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let hours;
//         let minutes;
//         if self.hours < 10 {
//             hours = format!("0{}", self.hours);
//         } else {
//             hours = format!("{}", self.hours);
//         }
//         if self.minutes < 10 {
//             minutes = format!("0{}", self.minutes);
//         } else {
//             minutes = format!("{}", self.minutes);
//         }
//         write!(f, "{}:{}", hours, minutes)
//     }
// }
