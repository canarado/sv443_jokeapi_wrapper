# sv443_jokeapi Rust Crate

cargo: `0.1.1`

## Easy to use crate to interact with the sv443 Joke API
[Located here](https://sv443.net/jokeapi/v2)

As of right now, this crate only retrieves jokes. There is no posting functionality nor is there is any methods to retrieve jokes based on `search strings` or `id`'s.

## Usage
```rs
// import all of the library features
use sv443_jokeapi::{JokeAPI, constants::*};

fn main() {
    // creating a mutable builder struct
    let mut builder: JokeAPI = JokeAPI::builder(); //takes no arguments

    // JokeAPI's methods are chainable, I will seperate
    // the methods from the builder here however to have a clear distinction

    // Here we will chain some methods to start building our query
    builder
    .category(Category::Programming)
    .flag(Flag::Nsfw)
    .format(Format::json) // for right now format is redundant and will break things, will change soon however
    .joke_type(Types::Single);

    builder.build(); // builds the query and validates all of the data

    // up to this point, we've built the query and constructed the builder
    // and now we just need to use the JokeAPI#get method to call the API
    // and get our data which is returned, by default, in JSON format
    // unless specified otherwise with the JokeAPI#format method, which
    // as of right now is broken.

    // JokeAPI#get returns a serde_json Value struct
    let data = builder.get();

    // we expect `The Joke's category is "Programming"` to be printed.
    // for more information on format of the data that should be recieved,
    // visit https://sv443.net/jokeapi/v2
    println!("The joke's category is {}!", data["category"]);
}
```