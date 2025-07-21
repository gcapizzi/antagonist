use crossterm::style::Stylize;

pub fn info(file: &str) -> anyhow::Result<()> {
    match metaflac::Tag::read_from_path(file) {
        Ok(tag) => print_flac(tag),
        Err(metaflac::Error {
            kind: metaflac::ErrorKind::InvalidInput,
            ..
        }) => {
            // not a FLAC, skip
        }
        Err(e) => return Err(e)?,
    }

    match id3::v1::Tag::read_from_path(file) {
        Ok(tag) => print_id3v1(tag),
        Err(id3::Error {
            kind: id3::ErrorKind::NoTag,
            ..
        }) => {
            // no ID3v1, skip
        }
        Err(e) => return Err(e)?,
    }

    match id3::Tag::read_from_path(file) {
        Ok(tag) => print_id3v2(tag),
        Err(id3::Error {
            kind: id3::ErrorKind::NoTag,
            ..
        }) => {
            // no ID3v2, skip
        }
        Err(e) => return Err(e)?,
    }

    Ok(())
}

fn print_flac(tag: metaflac::Tag) {
    println!("{}", "FLAC".bold());
    for block in tag.blocks() {
        match block {
            metaflac::Block::StreamInfo(info) => {
                println!("* {}", "Stream Info:".italic());
                println!("  Minimum block size (samples): {}", &info.min_block_size);
                println!("  Minimum block size (samples): {}", &info.min_block_size,);
                println!("  Maximum block size (samples): {}", &info.max_block_size,);
                println!("  Minimum frame size (bytes): {}", &info.max_frame_size,);
                println!("  Minimum frame size (bytes): {}", &info.min_frame_size,);
                println!("  Sample rate (Hz): {}", &info.sample_rate);
                println!("  Number of channels: {}", &info.num_channels);
                println!("  Bits per sample: {}", &info.bits_per_sample);
                println!("  Total samples: {}", &info.total_samples);
                println!(
                    "  MD5: {}",
                    &info
                        .md5
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>(),
                );
            }
            metaflac::Block::VorbisComment(comment) => {
                println!("* {}", "Vorbis Comment:".italic());
                println!("  Vendor: {}", &comment.vendor_string);
                println!("  Comments:");
                for (key, value) in &comment.comments {
                    print!("  - {}: ", key);
                    println_indented(&value.join(", "), 6);
                }
            }
            metaflac::Block::Application(application) => {
                println!("* {}", "Application:".italic());
                println!(
                    "  ID: {}",
                    str::from_utf8(&application.id).unwrap_or_default(),
                );
                println!(
                    "  Data: {}",
                    str::from_utf8(&application.data).unwrap_or_default(),
                );
            }
            metaflac::Block::CueSheet(cue_sheet) => {
                println!("* {}", "Cue Sheet:".italic());
                println!("  Media catalog number: {}", &cue_sheet.catalog_num);
                println!("  Number of lead-in samples: {}", &cue_sheet.num_leadin);
                println!("  Is CD: {}", &cue_sheet.is_cd);
                println!("  Tracks:");
                for track in &cue_sheet.tracks {
                    println!("  - Offset (samples): {}", track.offset);
                    println!("  - Is audio: {}", track.is_audio);
                    println!("  - ISRC: {}", track.isrc);
                    println!("  - Pre-emphasis: {}", track.pre_emphasis);
                    println!("  - Indices:");
                    for index in &track.indices {
                        println!("    . Point number: {}", index.point_num);
                        println!("      Offset (samples): {}", index.offset);
                    }
                }
            }
            metaflac::Block::Padding(padding) => {
                println!("* {} {}", "Padding (bytes):".italic(), padding);
            }
            metaflac::Block::Picture(picture) => {
                println!("* {}", "Picture:".italic());
                println!("  Type: {:?}", picture.picture_type);
                println!("  Mime Type: {}", &picture.mime_type);
                println!("  Description: {}", &picture.description);
                println!("  Width: {}", &picture.width);
                println!("  Height: {}", &picture.height);
                println!("  Color Depth: {}", &picture.depth);
                println!("  Number of Colors: {}", &picture.num_colors);
                println!("  Data Length: {}", &picture.data.len());
            }
            metaflac::Block::SeekTable(_) => {
                // skip
            }
            metaflac::Block::Unknown(_) => {
                // skip
            }
        }
    }
}

fn print_id3v1(tag: id3::v1::Tag) {
    println!("{}", "ID3v1".bold());
    println!("Title: {}", &tag.title);
    println!("Artist: {}", &tag.artist);
    println!("Album: {}", &tag.album);
    print!("Comment: ");
    println_indented(&tag.comment, 2);
    if let Some(track) = tag.track {
        println!("Track: {}", &track.to_string());
    }
    if let Some(genre) = tag.genre() {
        println!("Genre: {}", &genre);
    }
    if let Some(speed) = tag.speed {
        println!("Speed: {}", &speed.to_string());
    }
    if let Some(start_time) = tag.start_time {
        println!("Start time: {}", &start_time);
    }
    if let Some(end_time) = tag.end_time {
        println!("End time: {}", &end_time);
    }
}

fn print_id3v2(tag: id3::Tag) {
    println!("{}", "ID3v2".bold());
    for frame in tag.frames() {
        println!(
            "{} [{}]: {}",
            frame.name(),
            frame.id(),
            &crate::id3v2::content_string(frame.content()),
        );
    }
}

fn println_indented(text: &str, indent: usize) {
    if text.contains('\n') {
        let indent_str = " ".repeat(indent);

        println!();
        for line in text.lines() {
            println!("{}{}", indent_str, line);
        }
    } else {
        println!("{}", text);
    }
}
