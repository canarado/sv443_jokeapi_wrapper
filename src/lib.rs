#![allow(dead_code, unreachable_patterns, non_snake_case, non_camel_case_types, unused_imports)]

use reqwest::Error;
use reqwest::Response;
use std::io::Read;
use std::vec::Vec;
pub mod constants;
use constants::*;
use std::any::type_name;
use serde_json;
// use yaml_rust::{YamlLoader, Yaml, YamlEmitter};
use serde_yaml;

#[derive(Debug, PartialEq)]
pub enum JokeResult {
    json(serde_json::Value),
    yaml(serde_yaml::Value),
    Err(String)
}

// impl JokeResult<T> {
//     pub fn get_json(&self, _: T) -> T {
//         match self {
//             crate::JokeResult::json(R) => R,
//             crate::JokeResult::yaml(R) => R,
//             crate::JokeResult::Err(E) => E
//         }
//     }
// }

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

    pub fn category(&mut self, category: Category) -> &mut Self {
        match category {
            Category::Any => {
                self.categories.push("Any".to_string());
                self
            },
            Category::Miscellaneous => {
                self.categories.push("Miscellaneous".to_string());
                self
            },
            Category::Programming => {
                self.categories.push("Programming".to_string());
                self
            },
            Category::Dark => {
                self.categories.push("Dark".to_string());
                self
            }
        }
    }

    pub fn flag(&mut self, flag_: Flag) -> &mut Self {
        match flag_ {
            Flag::Nsfw => {
                self.flags.push("nsfw".to_string());
                self
            },
            Flag::Religious => {
                self.flags.push("religious".to_string());
                self
            },
            Flag::Political => {
                self.flags.push("political".to_string());
                self
            },
            Flag::Racist => {
                self.flags.push("racist".to_string());
                self
            },
            Flag::Sexist => {
                self.flags.push("sexist".to_string());
                self
            }
        }
    }

    pub fn format(&mut self, fmt: Format) -> &mut Self {
        match fmt {
            Format::json => {
                self.formats = "json".to_string();
                self
            },
            Format::xml => {
                self.formats = "xml".to_string();
                self
            },
            Format::yaml => {
                self.formats = "yaml".to_string();
                self
            }
        }
    }

    pub fn joke_type(&mut self, type_: Type) -> &mut Self {
        match type_ {
            Type::Single => {
                self.types = "single".to_string();
                self
            },
            Type::Twopart => {
                self.types = "single".to_string();
                self
            }
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.API_URL = String::from("https://sv443.net/jokeapi/v2/joke/");
        self.categories = Vec::new();
        self.flags = Vec::new();
        self.formats = String::new();
        self.types = String::new();
        self.built = false;

        self
    }

    pub fn build(&mut self) -> &mut Self {

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
                let flagstring = format!("{}{}{}", "blacklistFlags=", self.flags[0], "&");
                extension_string.push_str(&flagstring);
            },
            _ => {
                let flagstring = format!("{}{}{}", "blacklistFlags=", self.flags.join(","), "&");
                extension_string.push_str(&flagstring);
            }
        }

        // handle format
        if self.formats.chars().count() > 0 && self.formats != "json".to_string() {
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
        self
    }

    pub fn get(&mut self) -> JokeResult {
        let mut response = reqwest::get(&self.API_URL).unwrap();

        assert!(response.status().is_success());

        let mut content = String::new();
        response.read_to_string(&mut content).expect("Couldn't read response data to string");

        match self.formats.as_str() {
            "json" => {
                let json_data: serde_json::Value = serde_json::from_str(&content).unwrap();

                JokeResult::json(json_data)
            },
            "yaml" => {
                // let mut yaml_data = YamlLoader::load_from_str(&content).unwrap();
                // let yaml_data_ = &yaml_data[0];

                // // let mut writer = String::new();
                // // {
                // //     let mut emitter = YamlEmitter::new(&mut writer);
                // //     emitter.dump(&yaml_data).unwrap();
                // // }
                let yaml_data: serde_yaml::Value = serde_yaml::from_str(&content).unwrap();

                JokeResult::yaml(yaml_data)
            },
            "xml" => {
                JokeResult::Err("xml is currently not able to be used, as said in the documentation, please use yaml or json(default) formats".to_string())
            },
            _ => {
                JokeResult::Err("No format found".to_string())
            }
        }

        // let mut content = String::new();
        // response.read_to_string(&mut content).expect("Couldn't read response data to string");

        // let json_data: Value = serde_json::from_str(&content).unwrap();

        // json_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_builder_check() {
        let api_builder: JokeAPI = JokeAPI::builder();

        assert_eq!(api_builder.API_URL, "https://sv443.net/jokeapi/v2/joke/".to_string())
    }

    #[test]
    fn builder_addtofields_check() {
        let mut api_builder: JokeAPI = JokeAPI::builder();
        api_builder.category(Category::Dark);

        assert_eq!(api_builder.categories[0], "Dark".to_string())
    }

    #[test]
    fn builder_method_check() {
        let mut api_builder: JokeAPI = JokeAPI::builder();

        api_builder.category(Category::Programming);
        api_builder.flag(Flag::Nsfw);
        api_builder.format(Format::yaml);
        api_builder.joke_type(Type::Single);
        api_builder.build();

        assert_eq!(api_builder.categories[0], "Programming");
        assert_eq!(api_builder.flags[0], "nsfw");
        assert_eq!(api_builder.formats, "yaml");
        assert_eq!(api_builder.types, "single");
        assert_eq!(api_builder.API_URL, "https://sv443.net/jokeapi/v2/joke/Programming?blacklistFlags=nsfw&format=yaml&type=single".to_string());
    }

    #[test]
    fn builder_reset_check() {
        let mut api_builder: JokeAPI = JokeAPI::builder();
        api_builder.category(Category::Programming);
        api_builder.reset();

        let test_eq_builder: JokeAPI = JokeAPI::builder();

        assert_eq!(api_builder, test_eq_builder)
    }

    #[test]
    fn build_get_test() {
        let mut api_builder: JokeAPI = JokeAPI::builder();
        api_builder.category(Category::Programming);

        api_builder.build();
        let content = api_builder.get();
        
        println!("{:?}", content);

        assert_eq!(api_builder.categories[0], "Programming");
    }

    #[test]
    fn chainability() {
        let mut api_builder: JokeAPI = JokeAPI::builder();

        api_builder
        .category(Category::Dark)
        .flag(Flag::Nsfw)
        .format(Format::yaml)
        .joke_type(Type::Single);

        api_builder.build();
        
        assert_eq!(api_builder.API_URL, "https://sv443.net/jokeapi/v2/joke/Dark?blacklistFlags=nsfw&format=yaml&type=single".to_string())
    }

    #[test]
    fn test_formatting() {
        let mut builder: JokeAPI = JokeAPI::builder();

        builder.category(Category::Dark).format(Format::yaml);

        builder.build();

        let data = builder.get();

        match data {
            JokeResult::json(D) => {
                println!("{:?}", D)
            },
            JokeResult::yaml(T) => {
                let f = &T["category"];

                println!("The category is {}!", f.as_str().unwrap())
            },
            JokeResult::Err(E) => {
                println!("{:?}", E)
            },
            _ => {
                println!("Whoops!")
            }
        }
    }

    #[test]
    fn get_json() {
        let mut builder: JokeAPI = JokeAPI::builder();

        builder
        .category(Category::Dark)
        .format(Format::json)
        .build();

        let data = builder.get();

        match data {
            JokeResult::json(D) => {
                println!("The category is {}!", D["category"])
            },
            JokeResult::yaml(T) => {
                let f = &T["category"];

                println!("The category is {}!", f.as_str().unwrap())
            },
            JokeResult::Err(E) => {
                println!("{:?}", E)
            },
            _ => {
                println!("Whoops!")
            }
        }
    }
}