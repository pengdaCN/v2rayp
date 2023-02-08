use anyhow::Result;

pub struct Shadowsocks {
    name: String,
    server: String,
    port: String,
    password: String,
    cipher: String,
    pulgin: Sip003,
}

impl Shadowsocks {
    pub fn from_ss_url(link: &str) -> Result<Self> {
        fn parse(s: &str) -> Option<Shadowsocks> {
            use http::uri::Uri;
            use base64::{Engine as _, engine::general_purpose};

            let link = s.parse::<Uri>().ok()?;
            let auth = link.authority()?;
            let username = auth.as_str().split('@').next()?;
            let username = String::from_utf8(
                general_purpose::URL_SAFE.decode(username).ok()?
            ).ok()?;

            let mut username = username.split(':');

            let cipher = username.next()?;
            let password = username.next()?;




            unimplemented!()
        }

        unimplemented!()
    }
}

struct Sip003 {
    name: String,
    opts: Sip003Opts,
}

struct Sip003Opts {
    tls: String,
    obfs: String,
    host: String,
    path: String,
    impf: String,
}