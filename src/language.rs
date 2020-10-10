use std::str::FromStr;
use failure;
use failure::format_err;

const PUNCTUATION: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const SPACE: &str = " ";

macro_rules! language_enum {
    ([$($language:ident),*]) => {
        /// Enumerates all language supported
        #[allow(non_camel_case_types)]
        #[derive(Copy,Clone,Debug)]
        pub enum Language {
            $( $language, )*
        }

        impl Language {
            pub fn all() -> Vec<Language> {
                vec![
                    $( Language::$language, )*
                ]
            }
        }
    }
}

language_enum!([EN]);

impl FromStr for Language {
    type Err = failure::Error;
    fn from_str(it: &str) -> Result<Language, Self::Err> {
        match &*it.to_lowercase() {
            "en" => Ok(Language::EN),
            _ => Err(format_err!("Unknown language {}", it)),
        }
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match *self {
            Language::EN => "en".to_string(),
        }
    }
}

impl Language {
    pub fn punctuation(&self) -> &'static str {
        match self {
            _ => PUNCTUATION
        }
    }

    pub fn default_separator(&self) -> &'static str {
        match self {
            _ => SPACE
        }
    }
}
