use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct SelectArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for SelectArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for SelectArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> SelectArgBuilder<'a> {
    pub fn new() -> SelectArgBuilder<'a> {
        SelectArgBuilder(ArgBuilder::new("select")
            .with_short("s")
            .with_long("select")
            .with_helptext("Select element")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("SELECT"))
    } 
}

