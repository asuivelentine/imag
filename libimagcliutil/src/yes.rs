use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct YesArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for YesArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for YesArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> YesArgBuilder<'a> {
    pub fn new() -> YesArgBuilder<'a> {
        YesArgBuilder(ArgBuilder::new("yes")
            .with_short("y")
            .with_long("yes")
            .with_helptext("Positive acknowledge")
            .with_takes_value(false)
            .with_required(false)
            .with_value_name("YES"))
    } 
}

