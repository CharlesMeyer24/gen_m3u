use super::songs::Songs;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
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
    let mut l_file = fs::File::create(l_m3u_file)?;
    for l_file_name in l_songs.get_songs() {
        write!(l_file, "{}\n", l_file_name)?;
    }

    Ok(())
}
