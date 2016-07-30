use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;

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

impl<'a> RangeArgBuilder<'a> {
    pub fn new() -> RangeArgBuilder<'a> {
        RangeArgBuilder(ArgBuilder::new("range")
            .with_short("r")
            .with_long("range")
            .with_helptext("in range")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("RANGE"))
    } 
}

