// API Categories
#[derive(Debug, PartialEq)]
pub enum Category {
    Any,
    Miscellaneous,
    Programming,
    Dark
}

// API Flags, genres of jokes to ignore
#[derive(Debug, PartialEq)]
pub enum Flag {
    Nsfw,
    Religious,
    Political,
    Racist,
    Sexist
}

// Format to recieve the data in
#[derive(Debug, PartialEq)]
pub enum Format {
    json,
    xml,
    yaml
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Single,
    Twopart
}