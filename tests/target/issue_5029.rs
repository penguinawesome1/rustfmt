// rustfmt-style_edition: 2027
// rustfmt-unstable_features: true

pub const FOO: &str = foo!("
    foo
    bar
");

pub const BAR: &str = bar!(
    "foo
    bar"
);

fn main() {
    let foo = foo!("
        foo bar
        foo bar
    ");
}
