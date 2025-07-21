fn main() -> anyhow::Result<()> {
    let m = clap::Command::new("antagonist")
        .version("0.1")
        .subcommand(clap::Command::new("info").arg(clap::Arg::new("file").required(true)))
        .subcommand(
            clap::Command::new("tag")
                .arg(clap::Arg::new("tag").required(true))
                .arg(clap::Arg::new("file").required(true)),
        )
        .subcommand(clap::Command::new("remove-id3v1").arg(clap::Arg::new("file").required(true)))
        .subcommand(
            clap::Command::new("remove-pictures").arg(clap::Arg::new("file").required(true)),
        )
        .subcommand(
            clap::Command::new("remove-tag")
                .arg(clap::Arg::new("tag").required(true))
                .arg(clap::Arg::new("file").required(true)),
        )
        .subcommand_required(true)
        .get_matches();

    match m.subcommand() {
        Some(("info", sub_m)) => {
            antagonist::commands::info(sub_m.get_one::<String>("file").unwrap())
        }
        Some(("tag", sub_m)) => {
            antagonist::commands::tag(
                sub_m.get_one::<String>("tag").unwrap(),
                sub_m.get_one::<String>("file").unwrap(),
            );
            Ok(())
        }
        Some(("remove-id3v1", sub_m)) => {
            antagonist::commands::remove_id3v1(sub_m.get_one::<String>("file").unwrap())
        }
        Some(("remove-pictures", sub_m)) => {
            antagonist::commands::remove_pictures(sub_m.get_one::<String>("file").unwrap())
        }
        Some(("remove-tag", sub_m)) => antagonist::commands::remove_tag(
            sub_m.get_one::<String>("tag").unwrap(),
            sub_m.get_one::<String>("file").unwrap(),
        ),
        Some((cmd, _)) => {
            eprintln!("Unknown command: {cmd}");
            std::process::exit(1);
        }
        _ => unreachable!(),
    }
}
