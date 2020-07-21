use houston as config;
use std::env;

#[test]
fn it_lists_nothing_when_no_profiles() {
    env::set_var("APOLLO_CONFIG_HOME", "./");
    let profiles = config::Profile::list().expect("listing profiles failed");

    assert!(profiles.is_empty());

    config::clear().expect("clearing configuration failed");
}

#[test]
fn it_lists_many_profiles() {
    env::set_var("APOLLO_CONFIG_HOME", "./");
    let cprofile_name = "corporate";
    let cprofile_pass = String::from("corporatepassword");

    config::Profile::set_api_key(cprofile_name, cprofile_pass).expect("setting api key failed");

    let pprofile_name = "personal";
    let pprofile_pass = String::from("personalpassword");

    config::Profile::set_api_key(pprofile_name, pprofile_pass).expect("setting api key failed");

    let profiles = config::Profile::list().expect("listing profiles failed");

    assert!(profiles.len() == 2);
    assert!(profiles.contains(&String::from(pprofile_name)));
    assert!(profiles.contains(&String::from(cprofile_name)));

    config::clear().expect("clearing configuration failed");
}
