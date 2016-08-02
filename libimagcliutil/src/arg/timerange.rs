use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct TimeRangeArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for TimeRangeArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for TimeRangeArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for TimeRangeArgBuilder<'a> {

    /// Builds a default TimeRangeArgBuilder with
    ///
    ///  * name        = "time-range"
    ///  * short       = "t"
    ///  * long        = "time-range"
    ///  * helptext    = "Select elements within the range"
    ///  * takes_value = true
    ///  * required    = false
    ///  * value_name  = "TIMERANGE"
    ///
    fn default() -> TimeRangeArgBuilder<'a> {
        TimeRangeArgBuilder(ArgBuilder::new("time-range")
            .with_short("tr")
            .with_long("time-range")
            .with_helptext("Select elements within the range")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("TIMERANGE"))
    }
}

impl<'a> TimeRangeArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches) -> bool {
        arg.is_present("time-range")
    }

    pub fn fetch_value(&self, arg: &'a ArgMatches) -> Option<Vec<&'a str>> {
        arg.value_of("time-range")
            .and_then(|i| Some(i.split("..").collect::<Vec<&str>>()))
            .or(None)
    }
}

