use super::download::download;
use std::process::Command;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct SongBase {
    name: String,
    artist: String,
    path: String,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Song {
    pub(crate) song: SongBase,
    url: String,
}

impl Song {
    pub(crate) fn new(name: String, artist: String, url: String) -> Song {
        Song {
            song: SongBase {
                name: name.clone(),
                artist: artist.clone(),
                path: "Songs/".to_owned() + &artist + " - " + &name + ".mp3",
            },
            url,
        }
    }

    pub(crate) fn save(&self, dl_path: &String) {
        let c = Command::new("ffmpeg")
            .args([
                "-i",
                dl_path,
                "-vn",
                "-ab",
                "128k",
                "-ar",
                "44100",
                "-y",
                &self.song.path,
            ])
            .spawn()
            .expect("Error");
        c.wait_with_output().unwrap();
    }

    pub(crate) fn delete(&self, dl_path: &String) {
        let _ = Command::new("rm").arg(dl_path).spawn().expect("Error");
    }

    pub(crate) fn download(&self) -> String {
        let dl_path = download(&self.url);
        self.save(&dl_path);
        dl_path
    }

    pub(crate) fn get_filename(&self) -> String {
        self.song.artist.clone() + " - " + &self.song.name.clone() + ".mp3"
    }
}
