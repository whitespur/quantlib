use super::traits::Calendar as Cal;
use super::{BusinessDayConvention, Date, Period, TimeUnit, Weekday};

#[derive(Copy, Clone)]
pub struct Calendar<C: Cal> {
    pub cal_impl: C,
}

impl<C: Cal> Calendar<C> {
    pub fn is_business_day(&self, date: Date) -> bool {
        self.cal_impl.is_business_day(date)
    }
    pub fn is_holiday(&self, date: Date) -> bool {
        !self.cal_impl.is_business_day(date)
    }
    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        self.cal_impl.is_weekend(&weekday)
    }
    pub fn is_end_of_month(&self, _date: Date) -> bool {
        false
    }
    pub fn end_of_month(&self, _date: Date) -> Date {
        Date::default()
    }
    pub fn add_holiday(&self, _date: Date) {}

    pub fn remove_holiday(&self, _date: Date) {}

    pub fn adjust(&self, _date: Date) -> Date {
        Date::default()
    }
    pub fn adjust_with_convention(&self, _date: Date, _convention: BusinessDayConvention) -> Date {
        Date::default()
    }
    pub fn advance_with_convention(
        &self,
        date: Date,
        period: Period,
        convention: BusinessDayConvention,
    ) -> Date {
        self.advance_convention_eom(date, period, convention, false)
    }
    pub fn advance_convention_eom(
        &self,
        date: Date,
        period: Period,
        convention: BusinessDayConvention,
        include_end_of_month: bool,
    ) -> Date {
        self.advance(
            date,
            period.length,
            period.units,
            convention,
            include_end_of_month,
        )
    }
    pub fn advance_by_units(&self, date: Date, n: usize, time_unit: TimeUnit) -> Date {
        self.advance(date, n, time_unit, BusinessDayConvention::Following, false)
    }
    pub fn advance_by_period(&self, date: Date, period: Period) -> Date {
        self.advance(
            date,
            period.length,
            period.units,
            BusinessDayConvention::Following,
            false,
        )
    }

    pub fn advance(
        &self,
        _date: Date,
        _n: usize,
        _time_unit: TimeUnit,
        _convention: BusinessDayConvention,
        _include_end_of_month: bool,
    ) -> Date {
        Date::default()
    }

    pub fn business_days_between(&self, from: Date, to: Date) -> i64 {
        self.business_days_between_include(from, to, true, false)
    }
    pub fn business_days_between_include(
        &self,
        _from: Date,
        _to: Date,
        _include_first: bool,
        _include_last: bool,
    ) -> i64 {
        0
    }
}

pub fn easter_monday(year: usize) -> usize {
    EASTER_MONDAYS[year - 1901]
}
pub fn easter_monday_ortho(year: usize) -> usize {
    ORTHODOX_EASTER_MONDAYS[year - 1901]
}

const EASTER_MONDAYS: [usize; 299] = [
    98, 90, 103, 95, 114, 106, 91, 111, 102, // 1901-1909
    87, 107, 99, 83, 103, 95, 115, 99, 91, 111, // 1910-1919
    96, 87, 107, 92, 112, 103, 95, 108, 100, 91, // 1920-1929
    111, 96, 88, 107, 92, 112, 104, 88, 108, 100, // 1930-1939
    85, 104, 96, 116, 101, 92, 112, 97, 89, 108, // 1940-1949
    100, 85, 105, 96, 109, 101, 93, 112, 97, 89, // 1950-1959
    109, 93, 113, 105, 90, 109, 101, 86, 106, 97, // 1960-1969
    89, 102, 94, 113, 105, 90, 110, 101, 86, 106, // 1970-1979
    98, 110, 102, 94, 114, 98, 90, 110, 95, 86, // 1980-1989
    106, 91, 111, 102, 94, 107, 99, 90, 103, 95, // 1990-1999
    115, 106, 91, 111, 103, 87, 107, 99, 84, 103, // 2000-2009
    95, 115, 100, 91, 111, 96, 88, 107, 92, 112, // 2010-2019
    104, 95, 108, 100, 92, 111, 96, 88, 108, 92, // 2020-2029
    112, 104, 89, 108, 100, 85, 105, 96, 116, 101, // 2030-2039
    93, 112, 97, 89, 109, 100, 85, 105, 97, 109, // 2040-2049
    101, 93, 113, 97, 89, 109, 94, 113, 105, 90, // 2050-2059
    110, 101, 86, 106, 98, 89, 102, 94, 114, 105, // 2060-2069
    90, 110, 102, 86, 106, 98, 111, 102, 94, 114, // 2070-2079
    99, 90, 110, 95, 87, 106, 91, 111, 103, 94, // 2080-2089
    107, 99, 91, 103, 95, 115, 107, 91, 111, 103, // 2090-2099
    88, 108, 100, 85, 105, 96, 109, 101, 93, 112, // 2100-2109
    97, 89, 109, 93, 113, 105, 90, 109, 101, 86, // 2110-2119
    106, 97, 89, 102, 94, 113, 105, 90, 110, 101, // 2120-2129
    86, 106, 98, 110, 102, 94, 114, 98, 90, 110, // 2130-2139
    95, 86, 106, 91, 111, 102, 94, 107, 99, 90, // 2140-2149
    103, 95, 115, 106, 91, 111, 103, 87, 107, 99, // 2150-2159
    84, 103, 95, 115, 100, 91, 111, 96, 88, 107, // 2160-2169
    92, 112, 104, 95, 108, 100, 92, 111, 96, 88, // 2170-2179
    108, 92, 112, 104, 89, 108, 100, 85, 105, 96, // 2180-2189
    116, 101, 93, 112, 97, 89, 109, 100, 85, 105, // 2190-2199
];

const ORTHODOX_EASTER_MONDAYS: [usize; 299] = [
    105, 118, 110, 102, 121, 106, 126, 118, 102, // 1901-1909
    122, 114, 99, 118, 110, 95, 115, 106, 126, 111, // 1910-1919
    103, 122, 107, 99, 119, 110, 123, 115, 107, 126, // 1920-1929
    111, 103, 123, 107, 99, 119, 104, 123, 115, 100, // 1930-1939
    120, 111, 96, 116, 108, 127, 112, 104, 124, 115, // 1940-1949
    100, 120, 112, 96, 116, 108, 128, 112, 104, 124, // 1950-1959
    109, 100, 120, 105, 125, 116, 101, 121, 113, 104, // 1960-1969
    117, 109, 101, 120, 105, 125, 117, 101, 121, 113, // 1970-1979
    98, 117, 109, 129, 114, 105, 125, 110, 102, 121, // 1980-1989
    106, 98, 118, 109, 122, 114, 106, 118, 110, 102, // 1990-1999
    122, 106, 126, 118, 103, 122, 114, 99, 119, 110, // 2000-2009
    95, 115, 107, 126, 111, 103, 123, 107, 99, 119, // 2010-2019
    111, 123, 115, 107, 127, 111, 103, 123, 108, 99, // 2020-2029
    119, 104, 124, 115, 100, 120, 112, 96, 116, 108, // 2030-2039
    128, 112, 104, 124, 116, 100, 120, 112, 97, 116, // 2040-2049
    108, 128, 113, 104, 124, 109, 101, 120, 105, 125, // 2050-2059
    117, 101, 121, 113, 105, 117, 109, 101, 121, 105, // 2060-2069
    125, 110, 102, 121, 113, 98, 118, 109, 129, 114, // 2070-2079
    106, 125, 110, 102, 122, 106, 98, 118, 110, 122, // 2080-2089
    114, 99, 119, 110, 102, 115, 107, 126, 118, 103, // 2090-2099
    123, 115, 100, 120, 112, 96, 116, 108, 128, 112, // 2100-2109
    104, 124, 109, 100, 120, 105, 125, 116, 108, 121, // 2110-2119
    113, 104, 124, 109, 101, 120, 105, 125, 117, 101, // 2120-2129
    121, 113, 98, 117, 109, 129, 114, 105, 125, 110, // 2130-2139
    102, 121, 113, 98, 118, 109, 129, 114, 106, 125, // 2140-2149
    110, 102, 122, 106, 126, 118, 103, 122, 114, 99, // 2150-2159
    119, 110, 102, 115, 107, 126, 111, 103, 123, 114, // 2160-2169
    99, 119, 111, 130, 115, 107, 127, 111, 103, 123, // 2170-2179
    108, 99, 119, 104, 124, 115, 100, 120, 112, 103, // 2180-2189
    116, 108, 128, 119, 104, 124, 116, 100, 120, 112, // 2190-2199
];
