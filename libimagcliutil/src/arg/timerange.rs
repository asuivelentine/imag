build_arg_builder!(
    TimeRangeArgBuilder,
    timerange,
    "time-range",
    |builder: ArgBuilder<'a>| builder
        .with_long("time")
        .with_helptext("Select via time(-range)")
        .with_takes_value(true)
        .with_required(false)
        .with_value_name("RANGE")
);

pub use self::timerange::*;

