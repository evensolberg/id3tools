#![forbid(unsafe_code)]

use anyhow::Result;

// use clap_mangen;

fn run() -> Result<()> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    println!("Generating Fig completions for id3tag - id3tag.js");
    let mut cli = common::build_cli("latest");
    clap_complete::generate(
        clap_complete_fig::Fig,
        &mut cli,
        "id3tag",
        &mut std::fs::File::create("id3tag.js")?,
    );

    println!("Generating man file for id3tag - id3tag.1 ");
    let out_dir = std::path::PathBuf::from(".");
    let man = clap_mangen::Man::new(cli);
    let mut buffer: Vec<u8> = std::vec::Vec::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("id3tag.1"), buffer)?;

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(()) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{err:#}");
            eprintln!("Error: {err:#}");
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
