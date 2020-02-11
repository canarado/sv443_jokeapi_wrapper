# sv443_jokeapi Rust Crate

cargo: `0.2.0`

## Easy to use crate to interact with the sv443 Joke API
[Located here](https://sv443.net/jokeapi/v2)

As of right now, this crate only retrieves jokes. There is no posting functionality nor is there is any methods to retrieve jokes based on `search strings` or `id`'s.

## Notices
- constants::Format::xml is broken, do not use
- no unwrap or expect on JokeResult
- Preferred format is json, yaml is in a working state, but json is easier to work with as of right now (v.0.2.0)

## Usage
To see how to use, please see the `example` directory in the repository for this crate on [GitHub](https://github.com/canarado/sv443_jokeapi_wrapper/tree/master/example)