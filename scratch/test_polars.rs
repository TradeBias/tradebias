use polars::prelude::*;

fn main() {
    let df = DataFrame::empty();
    let opts = DynamicGroupOptions {
        index_column: "time".into(),
        every: Duration::parse("1d"),
        period: Duration::parse("1d"),
        offset: Duration::parse("0s"),
        ..Default::default()
    };
    // df.group_by_dynamic(opts)
}
