# A tiny datetime and timestamp tranlate tool.
## from format str to create the time string
- format dis
   - %Y : year
   - %m : month
   - %d : day
   - %H : hour
   - %M : min
   - %S : sec

## timestamp to datetime of CvDate
```rust
use cvdate::CvDate;

let d = CvDate::new(1582939726);
assert_eq!(d.get_date(), "2020-02-29");
assert_eq!(d.get_time(), "09:28:46");
assert_eq!(d.get_date_time(), "2020-02-29 09:28:46");
assert_eq!(d.format("%d/%m/%Y %H:%M"), "29/02/2020 09:28");
assert_eq!(CvDate::is_leap(2020), true);
assert_eq!(CvDate::is_leap(2021), false);
```

## datetime string to datetime of CvDate
```rust
use cvdate::CvDate;

let d = CvDate::new_with_str("%Y/%m-%d %H:%M:%S","2020-07-29 15:23:27");
assert_eq!(d.get_timestamp(), 1596007407);
assert_eq!(d.get_date(), "2020-07-29");
assert_eq!(d.get_time(), "15:23:27");
assert_eq!(d.get_date_time(), "2020-07-29 15:23:27");
assert_eq!(d.format("%d/%m/%Y %H:%M"), "29/07/2020 15:23");
assert_eq!(CvDate::is_leap(2020), true);
assert_eq!(CvDate::is_leap(2021), false);
```
## datetime from one zone to other
```rust
use cvdate::CvDate;

//timezone east 8 str time to other area
let x = CvDate::new_with_str_tz("%Y-%m-%d %H:%M:%S","2020-02-29 05:23:27", 8);
//timezone east +9
assert_eq!(CvDate::new_with_tz(x.get_timestamp(), 9).get_date_time(), "2020-02-29 06:23:27");
//timezone west -10
assert_eq!(CvDate::new_with_tz(x.get_timestamp(), -10).get_date_time(), "2020-02-28 11:23:27");
```
