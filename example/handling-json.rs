use sv443_jokeapi::{JokeAPI, JokeResult, constants::*};
mod your_code;

fn main() {
    let mut builder: JokeAPI = JokeAPI::builder();

    // Defaults to json, but we are gonna be explicit here
    // for documentation sake.
    builder.category(Category::Programming).format(Format::json)
    .build();

    let json_data = builder.get();

    match json_data {
        JokeResult::json(J) => {
            // Printing J is going print a serde_json::Value to
            // the console, you can use this to see the Value.
            println("{:?}", J);

            // We can index into J and print without explicitly 
            println!("The joke is nsfw? {}!", J["flags"]["nsfw"]);

            // We can get data from the response like this, by
            // indexing into the Value.
            let category = &J["category"];

            // If the type of joke is a single joke(as opposed to
            // a two part), we could as an example run some function
            // based on this condition
            if &J["type"] == "single".to_string() {
                your_code::some_func();
            }
        },
        JokeResult::Err(E) => {
            println!("{}", E)
        }
    }

}