use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct DeleteArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for DeleteArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for DeleteArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> DeleteArgBuilder<'a> {
    pub fn new() -> DeleteArgBuilder<'a> {
        DeleteArgBuilder(ArgBuilder::new("delete")
            .with_short("d")
            .with_long("delete")
            .with_helptext("deletes the given element")
            .with_takes_value(false)
            .with_required(false)
            .with_value_name("DELETE"))
    } 
}

