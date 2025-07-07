use comfy_table::{Attribute, Cell, ContentArrangement, Table, presets::UTF8_FULL};
use std::fmt::Write;

pub fn info(file: &str) -> anyhow::Result<()> {
    if let Ok(tag) = metaflac::Tag::read_from_path(file) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec![
                Cell::new("FLAC").add_attribute(Attribute::Bold),
                Cell::new(""),
            ]);

        for block in tag.blocks() {
            match block {
                metaflac::Block::StreamInfo(info) => {
                    table.add_row(vec![
                        Cell::new("Stream Info")
                            .add_attributes(vec![Attribute::Italic, Attribute::Bold]),
                        Cell::new(""),
                    ]);
                    table.add_row(vec![
                        "Minimum block size (samples)",
                        &info.min_block_size.to_string(),
                    ]);
                    table.add_row(vec![
                        "Minimum block size (samples)",
                        &info.min_block_size.to_string(),
                    ]);
                    table.add_row(vec![
                        "Maximum block size (samples)",
                        &info.max_block_size.to_string(),
                    ]);
                    table.add_row(vec![
                        "Minimum frame size (bytes)",
                        &info.max_frame_size.to_string(),
                    ]);
                    table.add_row(vec![
                        "Minimum frame size (bytes)",
                        &info.min_frame_size.to_string(),
                    ]);
                    table.add_row(vec!["Sample rate (Hz)", &info.sample_rate.to_string()]);
                    table.add_row(vec!["Number of channels", &info.num_channels.to_string()]);
                    table.add_row(vec!["Bits per sample", &info.bits_per_sample.to_string()]);
                    table.add_row(vec!["Total samples", &info.total_samples.to_string()]);
                    table.add_row(vec![
                        "MD5",
                        &info
                            .md5
                            .iter()
                            .map(|b| format!("{:02x}", b))
                            .collect::<String>(),
                    ]);
                }
                metaflac::Block::VorbisComment(comment) => {
                    table.add_row(vec![
                        Cell::new("Vorbis Comment")
                            .add_attributes(vec![Attribute::Italic, Attribute::Bold]),
                        Cell::new(""),
                    ]);
                    table.add_row(vec!["Vendor", &comment.vendor_string]);
                    table.add_row(vec![
                        Cell::new("Comments").add_attribute(Attribute::Italic),
                        Cell::new(""),
                    ]);
                    for (key, value) in &comment.comments {
                        table.add_row(vec![key, &value.join(", ")]);
                    }
                }
                metaflac::Block::Application(application) => {
                    table.add_row(vec![
                        Cell::new("Application")
                            .add_attributes(vec![Attribute::Italic, Attribute::Bold]),
                        Cell::new(""),
                    ]);
                    table.add_row(vec![
                        "ID",
                        str::from_utf8(&application.id).unwrap_or_default(),
                    ]);
                    table.add_row(vec![
                        "Data",
                        str::from_utf8(&application.data).unwrap_or_default(),
                    ]);
                }
                metaflac::Block::CueSheet(cue_sheet) => {
                    table.add_row(vec![
                        Cell::new("Cue Sheet")
                            .add_attributes(vec![Attribute::Italic, Attribute::Bold]),
                        Cell::new(""),
                    ]);
                    table.add_row(vec![
                        "Media catalog number",
                        &cue_sheet.catalog_num.to_string(),
                    ]);
                    table.add_row(vec![
                        "Number of lead-in samples",
                        &cue_sheet.num_leadin.to_string(),
                    ]);
                    table.add_row(vec!["Is CD", &cue_sheet.is_cd.to_string()]);
                    table.add_row(vec![
                        Cell::new("Tracks").add_attribute(Attribute::Italic),
                        Cell::new(""),
                    ]);
                    for track in &cue_sheet.tracks {
                        let mut track_info = String::new();
                        writeln!(track_info, "Offset (samples): {}", track.offset)?;
                        writeln!(track_info, "Is audio: {}", track.is_audio)?;
                        writeln!(track_info, "ISRC: {}", track.isrc)?;
                        writeln!(track_info, "Pre-emphasis: {}", track.pre_emphasis)?;
                        writeln!(track_info, "Indices:")?;
                        for index in &track.indices {
                            writeln!(track_info, "- Point number: {}", index.point_num)?;
                            writeln!(track_info, "  Offset (samples): {}", index.offset)?;
                        }
                        table.add_row(vec![&track.number.to_string(), &track_info]);
                    }
                }
                metaflac::Block::Padding(padding) => {
                    table.add_row(vec![
                        Cell::new("Padding (bytes)")
                            .add_attributes(vec![Attribute::Italic, Attribute::Bold]),
                        Cell::new(&padding.to_string()),
                    ]);
                }
                metaflac::Block::Picture(picture) => {
                    table.add_row(vec![
                        Cell::new("Picture")
                            .add_attributes(vec![Attribute::Italic, Attribute::Bold]),
                        Cell::new(""),
                    ]);
                    table.add_row(vec!["Type", &format!("{:?}", picture.picture_type)]);
                    table.add_row(vec!["Mime Type", &picture.mime_type]);
                    table.add_row(vec!["Description", &picture.description]);
                    table.add_row(vec!["Width", &picture.width.to_string()]);
                    table.add_row(vec!["Height", &picture.height.to_string()]);
                    table.add_row(vec!["Color Depth", &picture.depth.to_string()]);
                    table.add_row(vec!["Number of Colors", &picture.num_colors.to_string()]);
                    table.add_row(vec!["Data Length", &picture.data.len().to_string()]);
                }
                metaflac::Block::SeekTable(_) => {
                    // skip
                }
                metaflac::Block::Unknown(_) => {
                    // skip
                }
            }
        }
        println!("{table}");
    }

    if let Ok(tag) = id3::v1::Tag::read_from_path(file) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec![
                Cell::new("ID3v1").add_attribute(Attribute::Bold),
                Cell::new(""),
            ]);
        table.add_row(vec!["Title", &tag.title]);
        table.add_row(vec!["Artist", &tag.artist]);
        table.add_row(vec!["Album", &tag.album]);
        table.add_row(vec!["Comment", &tag.comment]);
        if let Some(track) = tag.track {
            table.add_row(vec!["Track", &track.to_string()]);
        }
        if let Some(genre) = tag.genre() {
            table.add_row(vec!["Genre", &genre]);
        }
        if let Some(speed) = tag.speed {
            table.add_row(vec!["Speed", &speed.to_string()]);
        }
        if let Some(start_time) = tag.start_time {
            table.add_row(vec!["Start time", &start_time]);
        }
        if let Some(end_time) = tag.end_time {
            table.add_row(vec!["End time", &end_time]);
        }
        println!("{table}");
    }

    if let Ok(tag) = id3::Tag::read_from_path(file) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec![
                Cell::new("ID3v2").add_attribute(Attribute::Bold),
                Cell::new(""),
                Cell::new(""),
            ]);
        for frame in tag.frames() {
            table.add_row(vec![
                frame.id(),
                frame.name(),
                &crate::id3v2::content_string(frame.content()),
            ]);
        }
        println!("{table}");
    }

    Ok(())
}
