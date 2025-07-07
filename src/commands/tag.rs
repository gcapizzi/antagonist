pub fn tag(tag: &str, file: &str) {
    if let Ok(t) = metaflac::Tag::read_from_path(file) {
        if let Some(values) = t.get_vorbis(tag) {
            println!("{}", values.collect::<Vec<_>>().join("\n"));
        }
    }
    if let Ok(t) = id3::Tag::read_from_path(file) {
        for frame in t.frames().filter(|f| f.id() == tag) {
            println!("{}", crate::id3v2::content_string(frame.content()));
        }
    }
}
