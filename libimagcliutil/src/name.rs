use argbuilder::ArgBuilder;

pub struct NameArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> NameArgBuilder<'a> {
    pub fn new() -> NameArgBuilder<'a> {
        NameArgBuilder(ArgBuilder::new("name")
            .with_short("n")
            .with_long("name")
            .with_helptext("select element by name")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("NAME"))
    } 
}

