use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct RemoveArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for RemoveArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for RemoveArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> RemoveArgBuilder<'a> {
    pub fn new() -> AddArgBuilder<'a> {
        RemoveArgBuilder(ArgBuilder::new("remove")
            .with_short("r")
            .with_long("remove")
            .with_helptext("remove the Element")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("REMOVE"))
    } 
}

