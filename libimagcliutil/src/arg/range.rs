use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct RangeArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for RangeArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for RangeArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for RangeArgBuilder<'a> {

    /// Builds a default RangeArgBuilder with
    ///
    ///  * name        = "range"
    ///  * short       = "r"
    ///  * long        = "range"
    ///  * helptext    = "in range"
    ///  * takes_value = true
    ///  * required    = false
    ///  * value_name  = "RANGE"
    ///
    fn default() -> RangeArgBuilder<'a> {
        RangeArgBuilder(ArgBuilder::new("range")
            .with_short("r")
            .with_long("range")
            .with_helptext("in range")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("RANGE"))
    }
}

impl<'a> RangeArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches<'a>) -> bool {
        arg.is_present("range")
    }

    pub fn fetch_value(&self, arg: &'a ArgMatches) -> Option<Vec<i32>> {
        arg.value_of("range")
            .and_then(|i| Some(i.split("..")
                               .map(|i| i.parse::<i32>().unwrap_or(0))
                               .collect::<Vec<i32>>()))
            .or(None)
    }
}

