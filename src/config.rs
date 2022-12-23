use super::songs::Songs;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
extern crate tempfile;

pub struct Config {
    pub folder_name: PathBuf,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get folder name"),
        };
        Ok(Config {
            folder_name: PathBuf::from(filename),
        })
    }
}

pub fn run(p_config: Config) -> Result<(), Box<dyn Error>> {
    let mut l_songs = Songs::new(p_config.folder_name.clone());
    l_songs.create_playlist()?;

    let l_folder_name = p_config
        .folder_name
        .strip_prefix(p_config.folder_name.parent().unwrap().to_str().unwrap())?;
    let l_m3u_file = String::from(format!(
        "{}/{}.m3u",
        p_config.folder_name.to_str().unwrap(),
        l_folder_name.to_str().unwrap()
    ));

    let l_file = fs::File::create(l_m3u_file)?;
    let mut l_writer = BufWriter::new(l_file);

    // writes M3U header
    l_writer.write_all(b"#EXTM3U\n")?;

    for l_file_name in l_songs.get_songs() {
        l_writer.write_all(format!("#EXTINF:-1,{}\n", l_file_name).as_bytes())?;
        l_writer.write_all(l_file_name.as_bytes())?;
        l_writer.write_all(b"\n")?;
    }

    Ok(())
}
