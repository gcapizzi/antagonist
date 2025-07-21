pub fn remove_id3v1(file: &str) -> anyhow::Result<()> {
    let _ = id3::v1::Tag::remove_from_path(file)?;
    Ok(())
}
