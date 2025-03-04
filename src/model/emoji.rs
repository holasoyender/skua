use crate::all::{CustomEmoji, EmojiId, ReactionType};


pub enum Emoji {
    Custom(CustomEmoji),
    Unicode(String),
}

impl Emoji {

    pub fn from_unicode(code: &str) -> Self {
        Self::Unicode(code.to_string())
    }

    pub fn from_custom(name: String, id: EmojiId, animated: bool) -> Self {
        Self::Custom(CustomEmoji { name, managed: false, require_colons: false, roles: vec![], id, animated, available: false, user: None })
    }

    pub fn formatted(&self) -> String {
        match self {
            Self::Custom(emoji) => format!("<:{}:{}>", emoji.name, emoji.id),
            Self::Unicode(emoji) => emoji.clone(),
        }
    }

    pub fn reaction(&self) -> ReactionType {
        match self {
            Self::Custom(emoji) => ReactionType::Custom { animated: emoji.animated, id: emoji.id, name: Some(emoji.name.clone()) },
            Self::Unicode(_) => ReactionType::Unicode(self.formatted()),
        }
    }
}