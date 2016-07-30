use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;

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

impl<'a> DateRangeArgBuilder<'a> {
    pub fn new() -> DateRangeArgBuilder<'a> {
        DateRangeArgBuilder(ArgBuilder::new("date-range")
            .with_short("dr")
            .with_long("date-range")
            .with_helptext("Select elements within the range")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("DATERANGE"))
    } 
}

