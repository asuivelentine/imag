use argbuilder::ArgBuilder;

pub struct NameArgBuilder;

impl<'a> NameArgBuilder {
    pub fn new() -> ArgBuilder<'a> {
        ArgBuilder::new("name")
            .with_short("n")
            .with_long("name")
            .with_helptext("select element by name")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("NAME")
    } 
}

