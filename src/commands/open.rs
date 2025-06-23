use open;

pub fn open_github(name: &str) {
    let _ = open::that(format!("https://github.com/{}", name));
}
