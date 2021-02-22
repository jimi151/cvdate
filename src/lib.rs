//! # A tiny datetime and timestamp conversion tool by rust.
//! 
//! notice: Demos local timezone is east 8,other timezone need to change the assert_eq right.
//!
//! ## from format str to create the time string
//! - format dis
//!    - %Y : year
//!    - %m : month
//!    - %d : day
//!    - %H : hour
//!    - %M : min
//!    - %S : sec
//! ## timestamp to datetime of CvDate
//! ```rust
//! use cvdate::CvDate;
//!
//! let d = CvDate::new(1582939726);
//! assert_eq!(d.get_date(), "2020-02-29");
//! assert_eq!(d.get_time(), "09:28:46");
//! assert_eq!(d.get_date_time(), "2020-02-29 09:28:46");
//! assert_eq!(d.format("%d/%m/%Y %H:%M"), "29/02/2020 09:28");
//! assert_eq!(CvDate::is_leap(2020), true);
//! assert_eq!(CvDate::is_leap(2021), false);
//!
//! ```
//! ## datetime string to datetime of CvDate
//! ```rust
//! use cvdate::CvDate;
//!
//! let d = CvDate::new_with_str("%Y/%m-%d %H:%M:%S","2020-07-29 15:23:27");
//! assert_eq!(d.get_timestamp(), 1596007407);
//! assert_eq!(d.get_date(), "2020-07-29");
//! assert_eq!(d.get_time(), "15:23:27");
//! assert_eq!(d.get_date_time(), "2020-07-29 15:23:27");
//! assert_eq!(d.format("%d/%m/%Y %H:%M"), "29/07/2020 15:23");
//! assert_eq!(CvDate::is_leap(2020), true);
//! assert_eq!(CvDate::is_leap(2021), false);
//! ```
//! ## datetime from one zone to other
//! ```rust
//! use cvdate::CvDate;
//!
//! //timezone east 8 str time to other area
//! let x = CvDate::new_with_str_tz("%Y-%m-%d %H:%M:%S","2020-02-29 05:23:27", 8);
//! //timezone east +9
//! assert_eq!(CvDate::new_with_tz(x.get_timestamp(), 9).get_date_time(), "2020-02-29 06:23:27");
//! //timezone west -10
//! assert_eq!(CvDate::new_with_tz(x.get_timestamp(), -10).get_date_time(), "2020-02-28 11:23:27");
//! ```
#[derive(Debug,Default)]
pub struct CvDate{
    tmstp: i64,
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    min: i64,
    sec: i64,
    week: i64,
    tz: i64,
}

use std::sync::Once;
const FOUR_YEAR_DAY: i64 = 1461;
const DAY_SEC: i64 =  86400;
const MONTH_ARR: [i64; 12] =      [31,28,31,30,31,30,31,31,30,31,30,31];
const MONTH_LEAP_ARR: [i64; 12] = [31,29,31,30,31,30,31,31,30,31,30,31];
static mut VAL: i64 = 0;
static CACHE: Once = Once::new();

impl CvDate{
    /// new CvDate with timestmp
    /// ```rust
    /// use cvdate::CvDate;
    /// let _x = CvDate::new(1582939726);
    /// ```
    pub fn new(tmstp: i64) -> Self {
        let mut t =Self::default();
        t.set_timestamp(tmstp);
        t.set_zone(13);
        t.build();
        t
    }

    /// new CvDate with timestmp and timezone
    /// ```rust
    /// use cvdate::CvDate;
    /// let _x = CvDate::new_with_tz(1582939726, -7);
    /// ```
    pub fn new_with_tz(tmstp: i64, tz: i64) -> Self {
        let mut t =Self::default();
        t.set_timestamp(tmstp);
        t.set_zone(tz);
        t.build();
        t
    }


    fn set_timestamp(&mut self, stp: i64) {
        self.tmstp = stp;
    }

    /// new CvDate with datetime string
    /// ```rust
    /// use cvdate::CvDate;
    ///
    /// let _x = CvDate::new_with_str("%Y-%m-%d %H:%M:%S","2050-12-18 15:23:27");
    /// ```
    pub fn new_with_str(fm: &str, dt: &str) -> Self {
        let mut t =Self::default();
        t.set_zone(13);
        t.build_str(fm, dt);
        t
    }

    /// new CvDate with datetime string and timezone
    /// ```rust
    /// use cvdate::CvDate;
    ///
    /// let _x = CvDate::new_with_str_tz("%Y-%m-%d %H:%M:%S","2050-12-18 15:23:27", -7);
    /// ```
    pub fn new_with_str_tz(fm: &str, dt: &str, tz: i64) -> Self {
        let mut t =Self::default();
        t.set_zone(tz);
        t.build_str(fm, dt);
        t
    }
       
    fn build_str(&mut self, fm: &str, dt: &str) {
        let s1: Vec<&str> = fm.split(&['-','/',' ',':'][..]).collect();
        let s2: Vec<&str> = dt.split(&['-','/',' ',':'][..]).collect();
        if s1.len() != s2.len() {
            eprint!("format err");
        }
        self.tmstp -= 3600_i64.checked_mul(self.tz).unwrap_or(0);
        s1.iter().zip(s2.iter()).for_each(|(k,v)|{
            match *k {
                "%Y" => {
                    self.year = v.parse::<i64>().unwrap_or_default();
                    let ally = self.year.checked_sub(1970).unwrap_or(0);
                    let loopy= ally.checked_div(4).unwrap_or(0);
                    self.tmstp += loopy.checked_mul(FOUR_YEAR_DAY).unwrap_or(0).
                                        checked_mul(DAY_SEC).unwrap_or(0);
                    (loopy.checked_mul(4).unwrap_or(0)..ally).for_each(|y|{
                        let days: i64 = if Self::is_leap(y + 1970) { 366 } else { 365 };
                        self.tmstp += days.checked_mul(DAY_SEC).unwrap_or(0);
                    });
                }
                "%m" => {
                    self.month = v.parse::<i64>().unwrap_or_default();
                    let month = if Self::is_leap(self.year) { MONTH_LEAP_ARR } else { MONTH_ARR };
                    for (i,md) in month.iter().enumerate() {
                        let cm: i64 = (i as i64).checked_add(1).unwrap_or(0);
                        if cm < self.month {
                            self.tmstp += md.checked_mul(DAY_SEC).unwrap_or(0);
                        }
                    }
                }
                "%d" => {
                    self.day = v.parse::<i64>().unwrap_or_default();
                    let d: i64 = self.day.checked_sub(1).unwrap_or(0);
                    self.tmstp += d.checked_mul(DAY_SEC).unwrap_or(0);
                }
                "%H" => {
                    self.hour = v.parse::<i64>().unwrap_or_default();
                    self.tmstp += self.hour.checked_mul(3600).unwrap_or(0);
                }
                "%M" => {
                    self.min = v.parse::<i64>().unwrap_or_default();
                    self.tmstp += self.min.checked_mul(60).unwrap_or(0);
                }
                "%S" => {
                    self.sec = v.parse::<i64>().unwrap_or_default();
                    self.tmstp += self.sec;
                }
                _ =>(),
            }
        });
        self.week = self.week(self.tmstp);
    }

    fn build(&mut self) {
        let loop_num = self.get_loop_year_last();
        let mut start = loop_num.checked_mul(
                FOUR_YEAR_DAY.checked_mul(DAY_SEC).unwrap_or(0)
            ).unwrap_or(0).checked_sub(
                3600i64.checked_mul(self.tz).unwrap_or(0)
            ).unwrap_or(0);
        let begin_year = 1970_i64.checked_add(loop_num.checked_mul(4).unwrap_or(0)).unwrap_or(0);
        for y in 0..4 {
            let cy = begin_year.checked_add(y).unwrap_or(0);
            let cy_end =  start + (DAY_SEC * (if Self::is_leap(cy) { 366 } else {365}));
            if self.tmstp > cy_end {
                start = cy_end;
            } else {
                self.year = cy;
                let arr = if Self::is_leap(self.year) { MONTH_LEAP_ARR } else { MONTH_ARR };
                for (i,m) in arr.iter().enumerate() {
                    let m_end = start.checked_add(m.checked_mul(DAY_SEC).unwrap_or(0)).unwrap_or(0);
                    if self.tmstp < m_end {
                        self.month = (i as i64).checked_add(1).unwrap_or(0);
                        let day = self.tmstp.checked_sub(start).unwrap_or(0)
                                            .checked_div(DAY_SEC).unwrap_or(0);
                        self.day = day.checked_add(1).unwrap_or(0);
                        start += day.checked_mul(DAY_SEC).unwrap_or(0);
                        self.hour = self.tmstp.checked_sub(start).unwrap_or(0)
                                        .checked_div(3600).unwrap_or(0);
                        start += self.hour.checked_mul(3600).unwrap_or(0);
                        self.min = self.tmstp.checked_sub(start).unwrap_or(0).checked_div(60).unwrap_or(0);
                        start += self.min.checked_mul(60).unwrap_or(0);
                        self.sec = self.tmstp.checked_sub(start).unwrap_or(0);
                        break;
                    } else{
                        start = m_end;
                    }
                }
                break;
            }
        }
        self.week = self.week(self.tmstp);
    }

    /// get week from timestamp
    /// ```rust
    /// use cvdate::CvDate;
    ///
    /// let x = CvDate::new_with_tz(1613958868, 8);
    /// assert_eq!(x.get_week(), 1);
    /// ```
    fn week(&self, stp: i64) -> i64 {
        let w = stp.checked_add(3600_i64.checked_mul(self.tz).unwrap_or(0)).unwrap_or(0)
                .checked_div(DAY_SEC).unwrap_or(0)
                .checked_rem(7).unwrap_or(0)
                .checked_add(4).unwrap_or(0);
        if w > 7 { 
            w.checked_sub(7).unwrap_or(0)
        }else{ 
            w 
        }
    }

    /// get cur week from CvDate
    /// ```rust
    /// use cvdate::CvDate;
    ///
    /// let x = CvDate::new_with_tz(1613836800,8);
    /// assert_eq!(x.get_week(), 7);
    /// ```
    pub fn get_week(&self) -> i64 {
        self.week
    }

    /// from datetime str to timestamp
    /// ```rust
    /// use cvdate::CvDate;
    ///
    /// let d = CvDate::new_with_str("%Y/%m-%d %H:%M:%S","2020-07-29 15:23:27");
    /// assert_eq!(d.get_timestamp(), 1596007407);
    /// let d = CvDate::new_with_str("%Y-%m-%d","2021-01-01");
    /// assert_eq!(d.get_timestamp(), 1609430400);
    /// ```
    pub fn get_timestamp(&self) -> i64 {
        self.tmstp
    }

    /// set cur timezone 
    /// if not set with this fn will use local timezhone
    ///
    fn set_zone(&mut self, z: i64) {
        let cz = if z == 13 {
            unsafe {
                CACHE.call_once(||{
                    VAL = if cfg!(target_os = "windows") {
                        let cbc = String::from_utf8(
                            std::process::Command::new("wmic")
                            .args(&["TIMEZONE","get","*","/value","|","find","/I","\"Description\""])
                            .output().expect("failed to execute process").stdout).unwrap();
                        let tm_arr = cbc.trim().split(&['C',':'][..]).collect::<Vec<_>>();
                        tm_arr.get(1).unwrap().parse::<i64>().unwrap_or(0)
                    } else {
                        let cbc = String::from_utf8(std::process::Command::new("date")
                            .arg("-R").output().expect("failed to execute process").stdout).unwrap();
                        let tm_arr = cbc.trim().trim_end_matches('0').rsplitn(2,' ').collect::<Vec<_>>();
                        tm_arr.get(0).unwrap().parse::<i64>().unwrap_or(0)
                    }
                });
                VAL
            }
        }else{
            z
        };
        self.tz = cz;
    }

    /// get datetime string from format
    /// - format dis
    ///    - %Y : year
    ///    - %m : month
    ///    - %d : day
    ///    - %H : hour
    ///    - %M : min
    ///    - %S : sec
    /// ```rust
    /// use cvdate::CvDate;
    ///
    /// let d = CvDate::new(1582939726);
    /// assert_eq!(d.format("%d/%m/%Y %H:%M"), "29/02/2020 09:28");
    /// assert_eq!(d.format("%Y-%m-%d"), "2020-02-29");
    /// ```
    pub fn format(&self, astr: &str) -> String {
        astr.replace("%d", &self.get_two(self.day))
            .replace("%Y", &self.year.to_string())
            .replace("%m", &self.get_two(self.month))
            .replace("%H", &self.get_two(self.hour))
            .replace("%M", &self.get_two(self.min))
            .replace("%S", &self.get_two(self.sec))
    }

    fn get_two(&self, i: i64) -> String {
        format!("{:02}",i)
    }

    /// get date Y-m-d string
    /// ```rust
    /// use cvdate::CvDate;
    ///
    /// let d = CvDate::new(1582939726);
    /// assert_eq!(d.get_date(), "2020-02-29");
    /// ```
    pub fn get_date(&self) -> String {
        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// get time hh:mm:ss string
    /// ```rust
    /// use cvdate::CvDate;
    /// let d = CvDate::new(1582939726);
    /// assert_eq!(d.get_time(), "09:28:46");
    /// ```
    pub fn get_time(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.min, self.sec)
    }

    /// get datetime Y-m-d hh:mm:ss string
    /// ```rust
    /// use cvdate::CvDate;
    /// let d = CvDate::new(1582939726);
    /// assert_eq!(d.get_date_time(), "2020-02-29 09:28:46");
    /// ```
    pub fn get_date_time(&self) -> String {
        format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.min, self.sec)
    }

    /// check a year if is leap
    /// ```rust
    /// use cvdate::CvDate;
    ///
    /// assert_eq!(CvDate::is_leap(2020), true);
    /// assert_eq!(CvDate::is_leap(2021), false);
    /// ```
    pub fn is_leap(year: i64) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    fn get_loop_year_last(&self) -> i64 {
        self.tmstp.checked_div(FOUR_YEAR_DAY.checked_mul(DAY_SEC).unwrap_or(0)).unwrap_or(0)
    }
}

