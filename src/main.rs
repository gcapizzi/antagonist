fn main() -> anyhow::Result<()> {
    let m = clap::Command::new("antagonist")
        .version("0.1")
        .subcommand(clap::Command::new("info").arg(clap::Arg::new("file").required(true)))
        .subcommand(
            clap::Command::new("tag")
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
        Some((cmd, _)) => {
            eprintln!("Unknown command: {cmd}");
            std::process::exit(1);
        }
        _ => unreachable!(),
    }
}
