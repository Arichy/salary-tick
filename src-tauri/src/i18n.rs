use fluent::{FluentBundle, FluentResource};
use fluent_bundle::FluentArgs;
use serde::{Deserialize, Serialize};
use std::fmt;
use unic_langid::LanguageIdentifier;

#[derive(Clone, Copy)]
pub enum Language {
    EN,
    ZH,
}

// 自定义序列化和反序列化
impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let lang_str = match *self {
            Language::EN => "en",
            Language::ZH => "zh",
        };
        serializer.serialize_str(lang_str)
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "en" => Ok(Language::EN),
            "zh" => Ok(Language::ZH),
            _ => Err(serde::de::Error::custom("unknown language")),
        }
    }
}

impl Into<LanguageIdentifier> for Language {
    fn into(self) -> LanguageIdentifier {
        match self {
            Language::EN => "en".parse().unwrap(),
            Language::ZH => "zh".parse().unwrap(),
        }
    }
}

impl fmt::Debug for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lang_str = match self {
            Language::EN => "en",
            Language::ZH => "zh",
        };
        write!(f, "{}", lang_str)
    }
}

pub struct I18n {
    bundle: FluentBundle<FluentResource>,
}

impl I18n {
    pub fn new(lang: Language) -> Self {
        let lanid: LanguageIdentifier = lang.into();
        let mut bundle = FluentBundle::new(vec![lanid]);

        let ftl_string = match lang {
            Language::ZH => include_str!("i18n/zh.ftl"),
            _ => include_str!("i18n/en.ftl"),
        };

        let resource =
            FluentResource::try_new(ftl_string.to_string()).expect("Failed to parse FTL string");

        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resource");

        Self { bundle }
    }

    pub fn translate(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let msg = self.bundle.get_message(key).expect("Message not found");
        let pattern = msg.value().expect("Message value not found");
        let mut errors = vec![];
        let value = self.bundle.format_pattern(pattern, args, &mut errors);

        if !errors.is_empty() {
            eprintln!("Failed to format message: {:?}", errors);
        }

        value.into_owned()
    }
}
