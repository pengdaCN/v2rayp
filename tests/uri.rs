#[test]
fn decode() {
    use http::uri::Uri;

    let url = "https://lib.rs/search?q=%E4%B8%AD".parse::<Uri>().unwrap();

    println!("{}", url.query().expect("1111"));
    println!(
        "{}",
        urlencoding::decode(url.query().expect("222")).expect("333")
    );
}

#[test]
fn query() {
    use v2rayp::common::UriQueries;

    let query = "%E4%B8%AD".parse::<UriQueries>().unwrap();

    println!("{:?}", query.get("q").unwrap())
}
