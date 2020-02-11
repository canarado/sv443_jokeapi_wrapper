// This example will document and explain everything, 
// the next few examples will not be so thorough and 
// will focus more on one concept.

// Here we are importing all of our modules, typically
// you want to not import using the '*' (glob) like we
// are here to import all of our constants to make your
// code a bit easier to read, but for ease of creating
// these examples, I am going to do so here.
use sv443_jokeapi::{JokeAPI, JokeResult, constants::*};

fn main() {

    // We create a mutable variable, builder, that is an
    // instance of JokeAPI. by calling the builder method
    let mut builder: JokeAPI = JokeAPI::builder();

    // The builder has a few methods that allow you to create
    // dynamic queries to the Joke API. Using the same method
    // multiple times may or may not cause errors, most often
    // with category and joke_type methods. We also are not
    // going to use the format method, which can be used to
    // query the API to return our joke data in one of three
    // formats: json, yaml, or xml. As of version 0.2.0 of this
    // crate, only json and yaml may be used. This will
    // be explained in another example.
    builder
    .category(Category::Programming)
    .flag(Flag::Nsfw)
    .joke_type(Type::Single)

    // The methods above will be explained and documented a bit
    // better in later examples.

    // The build method attempts to convert the builder's data
    // into a useable form for the API to consume. 
    .build();

    // The get method queries the API with all of our parameters.
    // This returns a `JokeResult` enum.
    let joke = builder.get();

    // Seeing as how `JokeResult` has no 'unwrap-like' methods,
    // similiar to `Result`'s unwrap, to get data from the enum
    // without a match statement, we are going to use a match
    // here to retrieve our data. *just for the time being :)*
    match joke {
        // We know 100% that our data here is going to be a JokeResult::json
        // enum or a JokeResult::Err enum so we can just match against those
        // two without matching against all the JokeResult variants or the
        // wildcard branch.
        JokeResult::json(J) => {
            // J here is a serde_json::Value, so we can print the whole
            // Value or parse it according to the spec located here:
            // https://docs.serde.rs/serde_json/#operating-on-untyped-json-values
            println!("{:?}", J);
        },
        JokeResult::Err(E) => {
            // E is just a String, we know what to do with those.
            println!("ERROR: {}", E);
        }
    }
}