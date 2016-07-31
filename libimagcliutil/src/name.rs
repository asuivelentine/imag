use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

pub struct NameArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for NameArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for NameArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for NameArgBuilder<'a> {
    fn default() -> NameArgBuilder<'a> {
        NameArgBuilder(ArgBuilder::new("name")
            .with_short("n")
            .with_long("name")
            .with_helptext("select element by name")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("NAME"))
    }
}

