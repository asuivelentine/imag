use argbuilder::ArgBuilder;

pub struct EditArgBuilder;

impl<'a> EditArgBuilder {
    pub fn new() -> ArgBuilder<'a> {
        ArgBuilder::new("edit")
            .with_short("e")
            .with_long("edit")
            .with_helptext("edit the given element")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("EDIT")
    } 
}

