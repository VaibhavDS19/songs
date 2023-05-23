use rustube::{block, url::Url, Video};

pub fn download(url: &str) -> String {
    let url = Url::parse(url).unwrap();
    let video = block!(Video::from_url(&url)).unwrap();
    let res = video.best_audio().unwrap();
    let path = block!(res.download()).unwrap();
    let filename = path.to_str().unwrap();
    filename.to_string()
}
