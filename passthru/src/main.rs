use passthru::PassThru;

struct BaseThing {
    one: bool,
    two: Option<String>,
}

#[derive(PassThru)]
struct DerThing {
    #[passthru]
    base: BaseThing,
}

fn main() {
    println!("Hello, world!");
}
