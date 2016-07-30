use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct AddArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for AddArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for AddArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> AddArgBuilder<'a> {
    pub fn new() -> AddArgBuilder<'a> {
        AddArgBuilder(ArgBuilder::new("add")
            .with_short("a")
            .with_long("add")
            .with_helptext("add to Element")
            .with_takes_value(false)
            .with_required(false)
            .with_value_name("ADD"))
    } 
}

