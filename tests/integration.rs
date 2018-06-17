extern crate assert_cli;

//kcov doesn't play nice with assert_cli() see
//https://github.com/assert-rs/assert_cli/issues/101
use std::env;
fn get_cwd() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}

#[test]
fn test_help() {
    //test that help works contains a USAGE string
    let cwd = get_cwd();
    let kcov: &str = &format!("{}/target/kcov-master/build/src/kcov", &cwd);
    let include: &str = &format!("--include-path={}", &cwd);
    let bin: &str = &format!("{}/target/debug/example-cli", &cwd);
    let args = ["--verify", include, bin, "-h"];
    assert_cli::Assert::command(&[kcov])
        .with_args(&args)
        .stdout()
        .contains("USAGE")
        .unwrap();
}
