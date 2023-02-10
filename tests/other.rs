#[test]
fn blocking_http_cli() {
    let cli = reqwest::blocking::Client::new();

    cli.get("http://baidu.com").send().unwrap();
}
