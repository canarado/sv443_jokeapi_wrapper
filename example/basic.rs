use sv443_jokeapi::{JokeAPI, constants::*};

fn main() {
    let mut builder: JokeAPI = JokeAPI::builder();

    builder
    .category(Category::Programming)
    .flag(Flag::Nsfw)
    .joke_type(Type::Single)
    .build();

    let joke = builder.get();

    println!("{:?}", joke);
}