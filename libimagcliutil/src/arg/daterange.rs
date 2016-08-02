use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct DateRangeArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for DateRangeArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for DateRangeArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for DateRangeArgBuilder<'a> {

    /// Builds a default DateRangeArgBuilder with name
    ///
    ///  * date-range  = "date-range"
    ///  * short       = "d"
    ///  * long        = "date-range"
    ///  * helptext    = "Select elements within the range"
    ///  * takes_value = true
    ///  * required    = false
    ///  * value_name  = "DATERANGE"
    ///
    fn default() -> DateRangeArgBuilder<'a> {
        DateRangeArgBuilder(ArgBuilder::new("date-range")
            .with_short("d")
            .with_long("date-range")
            .with_helptext("Select elements within the range")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("DATERANGE"))
    }
}

impl<'a> DateRangeArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches)-> bool {
        arg.is_present("date-range")
    }

    pub fn fetch_value(&self, arg: &'a ArgMatches) -> Option<Vec<&'a str>> {
        arg.value_of("date-range")
            .and_then(|i| Some(i.split("..").collect::<Vec<&str>>()))
            .or(None)
    }
}

