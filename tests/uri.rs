#[test]
fn decode() {
    use http::uri::Uri;

    let url = "https://lib.rs/search?q=%E4%B8%AD".parse::<Uri>().unwrap();

    println!("{}", url.query().unwrap());
    println!("{}", urlencoding::decode(url.query().unwrap()).unwrap());
}

#[test]
fn url_query() {
    use v2rayp::common::UriQueries;

    let _ = "q=%E4%B8%AD".parse::<UriQueries>().unwrap();
}
