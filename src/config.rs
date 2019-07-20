extern crate serde;
extern crate serde_derive;

use std::collections::HashMap;
use serde::Deserialize;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    title: String,

    #[serde(default)]
    import: Vec<String>,

    forge: Forge,

    #[serde(default)]
    mods: HashMap<String, Mod>,

    // Default filter settings.
    accept: Accept,

    // A list of mods (from imports, presumably) to exclude.
    #[serde(default)]
    exclude: Vec<String>,
}

#[derive(Deserialize)]
struct Forge {
    major: String,
    minor: Option<String>,
}

#[derive(Deserialize)]
struct Mod {
    // The .jar can be specified directly. This overrides everything else.
    jar: Option<String>,
    filter: Filter,

}

#[derive(Deserialize)]
struct Filter {
    accept: Accept,
    // List of versions, or version ranges, to exclude.
    exclude: Vec<String>,
    // Regexp of versions to exclude.
    regexp: Option<String>,
}

#[derive(Deserialize)]
struct Accept {
    stable: bool,
    beta: bool,
    alpha: bool,
}


#[cfg(test)]
mod tests {
    use toml::de::Error;
    use crate::config::Config;

    #[test]
    fn parses_minimal_config() {
        let toml: Result<Config, Error> = toml::from_str(r#"
          title = "Test pack"

          accept.stable = true
          accept.beta = true
          accept.alpha = false

          [forge]
          major = "1.12.2"
        "#);
        toml.unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_empty_config() {
        let toml: Result<Config, Error> = toml::from_str("");
        toml.unwrap();
    }
}