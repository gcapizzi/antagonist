use base64::{Engine, prelude::BASE64_STANDARD};

pub fn content_string(content: &id3::Content) -> String {
    match content {
        id3::Content::Private(private) => {
            format!(
                "{}: {}",
                private.owner_identifier,
                BASE64_STANDARD.encode(&private.private_data)
            )
        }
        _ => content.to_string(),
    }
}
