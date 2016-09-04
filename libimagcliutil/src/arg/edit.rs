build_arg_builder!(
    EditArgBuilder,
    edit,
    "edit",
    |builder: ArgBuilder<'a>| builder
        .with_short("e")
        .with_long("edit")
        .with_helptext("Edit in $EDITOR")
        .with_takes_value(false)
        .with_required(false)
);
