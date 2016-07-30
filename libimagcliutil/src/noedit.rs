use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct NoEditArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for NoEditArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for NoEditArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> NoEditArgBuilder<'a> {
    pub fn new() -> NoEditArgBuilder<'a> {
        NoEditArgBuilder(ArgBuilder::new("no-edit")
            .with_short("ne")
            .with_long("no-edit")
            .with_helptext("do not edit the given element")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("NOEDIT"))
    } 
}

