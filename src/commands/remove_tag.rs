use id3::TagLike;

pub fn remove_tag(tag: &str, file: &str) -> anyhow::Result<()> {
    if let Ok(mut t) = metaflac::Tag::read_from_path(file) {
        t.remove_vorbis(tag);
        return Ok(t.write_to_path(file)?);
    }
    if let Ok(mut t) = id3::Tag::read_from_path(file) {
        t.remove(tag);
        return Ok(t.write_to_path(file, t.version())?);
    }
    Ok(())
}
