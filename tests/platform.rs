/*
    Appellation: platform <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate template_rs_bin as pzzld;

use pzzld::config::Settings;

#[test]
fn settings() {
    let settings = {
        let tmp = Settings::build();
        assert!(tmp.is_ok());
        tmp.unwrap()
    };
    assert_eq!(settings.mode().as_ref(), "release");
}
