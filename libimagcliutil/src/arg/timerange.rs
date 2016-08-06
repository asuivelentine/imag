build_arg_builder!(
    TimeRangeArgBuilder,
    timerange,
    "time-range",
    |builder: ArgBuilder<'a>| builder
        .with_short("tr")
        .with_long("time-range")
        .with_helptext("Select elements within the range")
        .with_takes_value(true)
        .with_required(false)
        .with_value_name("TIMERANGE")
);
