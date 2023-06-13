use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    const FORBIDDEN_CHARS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    const MAX_NAME_LENGTH: usize = 256;

    fn is_valid_name(name: &str) -> bool {
        let empty = name.trim().is_empty();
        let too_long = name.graphemes(true).count() > Self::MAX_NAME_LENGTH;
        let contains_invalid = name.chars().any(|ch| Self::FORBIDDEN_CHARS.contains(&ch));

        empty || too_long || contains_invalid
    }

    pub fn parse(name: String) -> Result<SubscriberName, String> {
        match Self::is_valid_name(&name) {
            true => Ok(Self(name)),
            false => Err(format!("{:?}", name)),
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
