use argbuilder::ArgBuilder;

pub struct CreateArgBuilder;

impl<'a> CreateArgBuilder {
    pub fn new() -> ArgBuilder<'a> {
        ArgBuilder::new("create")
            .with_short("c")
            .with_long("create")
            .with_helptext("creates a new element with name")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("CREATE")
    } 
}

