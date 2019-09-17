

mod date{
    use std::ops;
    use std::cmp;


    #[derive(Debug, Clone, Copy)]
    pub struct Date{
        pub day: u8,
        pub month: u8,
        pub year: u32,
        _priv: ()
    }

    impl Date{
        pub fn new(day: u8, month: u8, year: u32) -> Date{
            Date{
                day: if day==0 {1} else {day},
                month: if month==0 {1} else {month},
                year,
                _priv: ()
            }
        }

        fn is_leap_year(year: u32) -> bool{
            let mut ret = false;

            if year % 4 == 0{
                ret = true;
            }else if year % 100 == 0 && year % 400 == 0{
                ret = true;
            }else{
                ret = false;
            }

            ret
        }
        //1 - January 12-december
        fn days_in_month(month:u8, year:u32) -> u32 {
            let mut ret = 30;

            if month == 1{
                if Date::is_leap_year(year){
                    ret = 29;
                }else{
                    ret = 28;
                }
            }else if month==6 || month==7{
                ret = 31;
            }else{
                ret = if (month%2 == 0) {31} else {30};
            }

            ret
        }
    }

    impl ops::Add<u32> for Date
    {
        type Output = Date;
        fn add(self, other: u32) -> Date{
            let mut other = other + (self.day as u32);
            let mut new_day = self.day;
            let mut new_month = self.month;
            let mut new_year = self.year;
            
            while other/Date::days_in_month(new_month, new_year) > 0
            {
                other -=  Date::days_in_month(new_month, new_year);
                new_month += 1;
                new_year = if new_year >= 12 {new_year} else {new_year+1};
            }
            new_day = other as u8;

            Date::new(new_day, new_month, new_year)
        }
    }
}


fn main() {
    let mut date = date::Date::new(17, 8, 2019);
    println!("{:?}", date);
    date = date+1;
    println!("{:?}", date);
    date = date+13;
    println!("{:?}", date);
    date = date+14;
    println!("{:?}", date);
}
