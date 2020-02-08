# sv443_jokeapi Rust Crate
```rs
// example of what should eventually should be the crate's usage, not quite here yet but getting there
use sv443_jokeapi::{JokeAPI, JokeAPIEnums::*};

fn main() {
    let mut builder: JokeAPI = JokeAPI::builder();
    builder
    .category(Category::Dark)
    .flag(Flag::Nsfw)
    .format(Format::yaml)
    .joke_type(Types::Single);

    builder.build();

    let jokeResponse = builder.get(); // should return a struct
}
```

As of right now, usage of this crate is looking something more like this
```rs
use sv443_jokeapi::{JokeAPI, JokeAPIEnums::*};

fn main {
    let mut builder: JokeAPI = JokeAPI::builder();

    builder.category(Category::Dark).expect("Some String value") // i am not expecting to return a Result<> forever, just temporarily for testing

    builder.build();

    let jokeResponse = builder.get(); // as of right now, returns a string. Going to add serde support soon to return struct
}
```