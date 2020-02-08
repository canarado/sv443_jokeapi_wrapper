#![allow(dead_code, unreachable_patterns, non_snake_case, non_camel_case_types, unused_imports)]
// #![allow(unreachable_patterns)]
// #![allow(non_snake_case)]
// #![allow(non_camel_case_types)]

#[macro_use]
extern crate serde;
extern crate reqwest;
use serde::{Serialize, Deserialize};
use reqwest::Error;
use reqwest::Response;
use std::vec::Vec;
pub mod enums;
use enums::*;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug, PartialEq)]
pub struct JokeAPI {
    API_URL: String,
    categories: Vec<String>, 
    flags: Vec<String>,
    formats: String,
    types: String,
    built: bool
}

impl JokeAPI {
    pub fn builder() -> JokeAPI {
        JokeAPI {
            API_URL: String::from("https://sv443.net/jokeapi/v2/joke/"),
            categories: Vec::new(),
            flags: Vec::new(),
            formats: String::new(),
            types: String::new(),
            built: false
        }
    }

    fn category(&mut self, category: Category) -> Result<(), String> {
        match category {
            Category::Any => {
                self.categories.push("Any".to_string());
                Ok(())
            },
            Category::Miscellaneous => {
                self.categories.push("Miscellaneous".to_string());
                Ok(())
            },
            Category::Programming => {
                self.categories.push("Programming".to_string());
                Ok(())
            },
            Category::Dark => {
                self.categories.push("Dark".to_string());
                Ok(())
            },
            _ => {
                Err("No category found".to_string())
            }
        }
    }

    fn flag(&mut self, flag_: Flag) -> Result<(), String> {
        match flag_ {
            Flag::Nsfw => {
                self.flags.push("nsfw".to_string());
                Ok(())
            },
            Flag::Religious => {
                self.flags.push("religious".to_string());
                Ok(())
            },
            Flag::Political => {
                self.flags.push("political".to_string());
                Ok(())
            },
            Flag::Racist => {
                self.flags.push("racist".to_string());
                Ok(())
            },
            Flag::Sexist => {
                self.flags.push("sexist".to_string());
                Ok(())
            },
            _ => {
                Err("No flag found".to_string())
            }
        }
    }

    fn format(&mut self, fmt: Format) -> Result<(), String> {
        match fmt {
            Format::json => {
                self.formats = "json".to_string();
                Ok(())
            },
            Format::xml => {
                self.formats = "xml".to_string();
                Ok(())
            },
            Format::yaml => {
                self.formats = "yaml".to_string();
                Ok(())
            },
            _ => {
                Err("No format found".to_string())
            }
        }
    }

    fn joke_type(&mut self, type_: Type) -> Result<(), String> {
        match type_ {
            Type::Single => {
                self.types = "single".to_string();
                Ok(())
            },
            Type::Twopart => {
                self.types = "single".to_string();
                Ok(())
            },
            _ => {
                Err("No joke type found".to_string())
            }
        }
    }

    fn reset(&mut self) {
        self.API_URL = String::from("https://sv443.net/jokeapi/v2/joke/");
        self.categories = Vec::new();
        self.flags = Vec::new();
        self.formats = String::new();
        self.types = String::new();
        self.built = false;
    }

    fn build(&mut self) -> Result<String, ()> {

        let mut extension_string = String::new();

        // handle categories
        match self.categories.len() {
            0 => {
                extension_string.push_str("Any?");
            },
            1 => {
                extension_string.push_str(&self.categories[0]);
                extension_string.push_str("?");
            },
            _ => {
                let mut catstring = self.categories.join(",");
                catstring.push_str("?");
                extension_string.push_str(&catstring);
            }
        }

        // handle flags
        match self.flags.len() {
            0 => (),
            1 => {
                // extension_string.push_str("blacklistFlags=");
                // extension_string.push_str(&self.flags[0]);
                // extension_string.push_str("?");
                let flagstring = format!("{}{}{}", "blacklistFlags=", self.flags[0], "&");
                extension_string.push_str(&flagstring);
            },
            _ => {
                let flagstring = format!("{}{}{}", "blacklistFlags=", self.flags.join(","), "&");
                extension_string.push_str(&flagstring);
            }
        }

        // handle format
        if self.formats.chars().count() > 0 {
            let formatstring = format!("{}{}{}", "format=", self.formats, "&");
            extension_string.push_str(&formatstring);
        }

        // handle joke types
        if self.types.chars().count() > 0 {
            let typestring = format!("{}{}", "type=", self.types);
            extension_string.push_str(&typestring);
        }

        self.API_URL.push_str(&extension_string);
        self.built = true;
        Ok("Client built".to_string())
    }

    fn get(&mut self) -> Result<Response, Error> {
        let mut response = reqwest::get(&self.API_URL)?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_enum_defs() {
        assert_eq!(Category::Dark, Category::Dark)
    }

    #[test]
    fn create_builder_check() {
        let api_builder: JokeAPI = JokeAPI::builder();

        assert_eq!(api_builder.API_URL, "https://sv443.net/jokeapi/v2/joke/".to_string())
    }

    #[test]
    fn builder_addtofields_check() {
        let mut api_builder: JokeAPI = JokeAPI::builder();
        api_builder.category(Category::Dark).expect("Whoopsies!");

        assert_eq!(api_builder.categories[0], "Dark".to_string())
    }

    #[test]
    fn builder_method_check() {
        let mut api_builder: JokeAPI = JokeAPI::builder();

        api_builder.category(Category::Programming).expect("Yeet!");
        api_builder.flag(Flag::Nsfw).expect("Yeet!");
        api_builder.format(Format::yaml).expect("Yeet!");
        api_builder.joke_type(Type::Single).expect("Yeet!");
        api_builder.build().expect("Fuck");

        assert_eq!(api_builder.categories[0], "Programming");
        assert_eq!(api_builder.flags[0], "nsfw");
        assert_eq!(api_builder.formats, "yaml");
        assert_eq!(api_builder.types, "single");
        assert_eq!(api_builder.API_URL, "https://sv443.net/jokeapi/v2/joke/Programming?blacklistFlags=nsfw&format=yaml&type=single".to_string());
    }

    #[test]
    fn builder_reset_check() {
        let mut api_builder: JokeAPI = JokeAPI::builder();
        api_builder.category(Category::Programming).expect("F");
        api_builder.reset();

        let test_eq_builder: JokeAPI = JokeAPI::builder();

        assert_eq!(api_builder, test_eq_builder)
    }

    #[test]
    fn build_get_test() {
        let mut api_builder: JokeAPI = JokeAPI::builder();
        api_builder.category(Category::Programming).expect("F");

        api_builder.build();
        let typestr = api_builder.get();
        
        match typestr {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{}", e)
        }

        assert_eq!(api_builder.categories[0], "Programming");
    }
}