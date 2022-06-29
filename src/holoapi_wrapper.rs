const HOST: &str = "https://api.holotools.app/v1";

pub fn get_live() {
    let response = reqwest::blocking::get(format!("{}{}", HOST, "/live")).unwrap();

    dbg!(response.text());
}