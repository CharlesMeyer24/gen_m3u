use std::cmp;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct Songs {
    m_folder_name: PathBuf,
    m_songs: Vec<String>,
}

impl Songs {
    pub fn new(path: PathBuf) -> Self {
        Songs {
            m_folder_name: path,
            m_songs: Vec::new(),
        }
    }

    pub fn get_songs<'a>(&'a self) -> &'a Vec<String> {
        &self.m_songs
    }

    pub fn create_playlist(&mut self) -> Result<(), Box<dyn Error>> {
        self.create_songs()?;
        self.m_songs.sort_by(|a, b| Songs::order_songs(a, b));
        Ok(())
    }

    fn create_songs(&mut self) -> Result<(), Box<dyn Error>> {
        let allowed_extensions: Vec<&str> = vec!["mp3", "flac", "m4a"];

        self.m_songs = fs::read_dir(&self.m_folder_name)?
            .map(|p_entry| p_entry.unwrap().path())
            .filter(|p_path| {
                let l_extension = match p_path.extension() {
                    Some(p_ext) => p_ext,
                    None => return false,
                };
                allowed_extensions.contains(&l_extension.to_str().unwrap())
            })
            .map(|p_path| String::from(p_path.file_name().unwrap().to_str().unwrap()))
            .collect::<Vec<String>>();

        Ok(())
        /*
        let l_result = l_paths.filter_map(|p_entry| {
            let l_entry = p_entry.unwrap();
            let l_entry_path = l_entry.path();
            let l_extension = match l_entry_path.extension()
            {
                Some(p_ext) => p_ext,
                None => return None,
            };
            let l_extension_str = l_extension.to_str().unwrap();
            if !allowed_extensions.contains(&l_extension_str)
            {
                return None;
            }
            let file_name = l_entry_path.file_name().unwrap();

            let file_name_as_str = file_name.to_str().unwrap();
          mut
            let file_name_as_string = String::from(file_name_as_str);

            Some(file_name_as_string)
          }
        ).collect::<Vec<String>>();
        */
        /*

        for p_entry in fs::read_dir(p_folder_name)? {

            let l_entry = p_entry?;
            let l_path = l_entry.path();

            let l_extension = match l_path.extension()
            {
                Some(p_ext) => p_ext,
                None => continue,
            };

            let l_extension_str = match l_extension.to_str()
            {
                Some(p_ext) => p_ext,
                None => continue,
            };

            if !allowed_extensions.contains(&l_extension_str) {
                continue;
            }

            let l_file_name = match l_path.file_name()
            {
                Some(p_file_name) => p_file_name,
                None => continue,
            };

            let l_file_name_str = match l_file_name.to_str()
            {
                Some(p_file_name) => p_file_name,
                None => continue,
            };

            l_result.push(l_file_name_str.to_string());
        }
        Ok(l_result)*/
    }

    fn order_songs(p1: &str, p2: &str) -> cmp::Ordering {
        let p1_chars: Vec<char> = p1.chars().collect();
        let p2_chars: Vec<char> = p2.chars().collect();

        // find positon of the first difference between the two slices
        let l_id = match std::iter::zip(&p1_chars, &p2_chars).position(|(c1, c2)| c1 != c2) {
            Some(p_id) => p_id,
            // No difference
            None => return std::cmp::Ordering::Equal,
        };

        // When difference is not from number
        if !p1_chars[l_id].is_digit(10) && !p2_chars[l_id].is_digit(10) {
            return p1_chars[l_id].cmp(&p2_chars[l_id]);
        }

        // Try to find positon of first non digits
        let l_new_id = match std::iter::zip(
            &p1_chars[l_id..p1_chars.len()],
            &p2_chars[l_id..p2_chars.len()],
        )
        .position(|(c1, c2)| !c1.is_digit(10) || !c2.is_digit(10))
        {
            Some(p_new_id) => p_new_id,
            // Remaining chars are all digits
            None => return p1_chars[l_id..p1_chars.len()].cmp(&p2_chars[l_id..p2_chars.len()]),
        };

        // first is digit and second not
        if p1_chars[l_new_id + l_id].is_digit(10) {
            return std::cmp::Ordering::Greater;
        }
        // Second is digit and first not
        else if p2_chars[l_new_id + l_id].is_digit(10) {
            return std::cmp::Ordering::Less;
        }

        // None of them are digits
        return p1_chars[l_id..l_new_id + l_id].cmp(&p2_chars[l_id..l_new_id + l_id]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_music_files_with_subdir() {
        // Créer un répertoire temporaire pour les tests
        let temp_dir = tempfile::TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Créer un sous-répertoire dans le répertoire temporaire
        let subdir_path = temp_path.join("subdir");
        fs::create_dir(&subdir_path).unwrap();

        // Créer quelques fichiers de test dans le répertoire et le sous-répertoire
        let test_files = [
            ("song1.mp3", b"song1"),
            ("song2.flac", b"song2"),
            ("song3.m4a", b"song3"),
            ("other.txt", b"other"),
        ];
        for &(file, contents) in &test_files {
            let file_path = temp_path.join(file);
            fs::write(&file_path, contents).unwrap();
        }
        for &(file, contents) in &test_files {
            let file_path = subdir_path.join(file);
            fs::write(&file_path, contents).unwrap();
        }

        let mut l_songs = Songs::new(temp_path.into());
        l_songs.create_songs().unwrap();
        // Appeler la fonction à tester
        let music_files = l_songs.get_songs();
        // Vérifier que la fonction renvoie tous les fichiers musicaux du répertoire et du sous-répertoire
        // Vérifier que la fonction renvoie les fichiers musicaux attendus
        assert_eq!(music_files.len(), 3);
        assert!(music_files.contains(&format!("song1.mp3")));
        assert!(music_files.contains(&format!("song2.flac")));
        assert!(music_files.contains(&format!("song3.m4a")));
    }

    #[test]
    fn test_get_music_files() {
        // Créer un répertoire temporaire pour les tests
        let temp_dir = tempfile::TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Créer quelques fichiers de test dans le répertoire
        let test_files = [
            ("song1.mp3", b"song1"),
            ("song2.flac", b"song2"),
            ("song3.m4a", b"song3"),
            ("other.txt", b"other"),
        ];
        for &(file, contents) in &test_files {
            let file_path = temp_path.join(file);
            fs::write(&file_path, contents).unwrap();
        }

        let mut l_songs = Songs::new(temp_path.into());
        l_songs.create_songs().unwrap();
        // Appeler la fonction à tester
        let music_files = l_songs.get_songs();

        // Vérifier que la fonction renvoie les fichiers musicaux attendus
        assert_eq!(music_files.len(), 3);
        assert!(music_files.contains(&format!("song1.mp3")));
        assert!(music_files.contains(&format!("song2.flac")));
        assert!(music_files.contains(&format!("song3.m4a")));
    }

    #[test]
    fn test_find_order() {
        let mut l_test: Vec<&str> = vec![("03 piste 3"), ("01 piste 1"), ("02 piste 2")];
        l_test.sort_by(|a, b| Songs::order_songs(a, b));
        assert_eq!(l_test, vec!["01 piste 1", "02 piste 2", "03 piste 3"]);

        l_test = vec![
            ("4 - I'm 22"),
            ("9 - Too Much"),
            ("1 - Blicky"),
            ("10 - Get to Work"),
        ];
        l_test.sort_by(|a, b| Songs::order_songs(a, b));
        assert_eq!(
            l_test,
            vec![
                ("1 - Blicky"),
                ("4 - I'm 22"),
                ("9 - Too Much"),
                ("10 - Get to Work")
            ]
        );

        l_test = vec![
            "18 - Outro.mp3",
            "12 - In Too Deep.mp3",
            "6 - Mr.Perfect.mp3",
            "9 - Too Much.mp3",
            "1 - Blicky.mp3",
            "2 - Absurd.mp3",
            "3 - Sixty (Melo).mp3",
            "7 - Exotic Talk.mp3",
            "15 - Free Game.mp3",
            "17 - Can I Vent_.mp3",
            "11 - Bape.mp3",
        ];
        l_test.sort_by(|a, b| Songs::order_songs(a, b));
        assert_eq!(
            l_test,
            vec![
                "1 - Blicky.mp3",
                "2 - Absurd.mp3",
                "3 - Sixty (Melo).mp3",
                "6 - Mr.Perfect.mp3",
                "7 - Exotic Talk.mp3",
                "9 - Too Much.mp3",
                "11 - Bape.mp3",
                "12 - In Too Deep.mp3",
                "15 - Free Game.mp3",
                "17 - Can I Vent_.mp3",
                "18 - Outro.mp3"
            ]
        );

        l_test = vec!["galactic snatchers B", "galactic snatchers A"];
        l_test.sort_by(|a, b| Songs::order_songs(a, b));
        assert_eq!(l_test, vec!["galactic snatchers A", "galactic snatchers B"]);

        l_test = vec![
            "Myka 9 - 02 - Mr.Perfect.mp3",
            "Myka 9 - 04 - I'm 22.mp3",
            "Myka 9 - 01 - Hola.mp3",
        ];
        l_test.sort_by(|a, b| Songs::order_songs(a, b));
        assert_eq!(
            l_test,
            vec![
                "Myka 9 - 01 - Hola.mp3",
                "Myka 9 - 02 - Mr.Perfect.mp3",
                "Myka 9 - 04 - I'm 22.mp3"
            ]
        );

        l_test = vec![
            "galactic snatchers B",
            "galactic snatchers A",
            "galactic snatchers A",
        ];
        l_test.sort_by(|a, b| Songs::order_songs(a, b));
        assert_eq!(
            l_test,
            vec![
                "galactic snatchers A",
                "galactic snatchers A",
                "galactic snatchers B"
            ]
        );

        l_test = vec!["12345678", "1234567899", "123456781"];
        l_test.sort_by(|a, b| Songs::order_songs(a, b));
        assert_eq!(l_test, vec!["12345678", "123456781", "1234567899"]);
    }
}
