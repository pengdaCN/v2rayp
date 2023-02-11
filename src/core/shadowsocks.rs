use crate::common::UriQueries;
use anyhow::{Context, Result};

pub struct Shadowsocks {
    name: Option<String>,
    server: String,
    port: i32,
    password: String,
    cipher: String,
    plugin: Option<Sip003>,
}

impl Shadowsocks {
    pub fn from_ss_url(link: &str) -> Result<Self> {
        fn parse(s: &str) -> Option<Shadowsocks> {
            use base64::{engine::general_purpose, Engine as _};
            use http::uri::Uri;

            let link = s.parse::<Uri>().ok()?;

            let server = link.host().map(String::from)?;

            let auth = link.authority()?;
            let username = auth.as_str().split('@').next()?;
            let username =
                String::from_utf8(general_purpose::URL_SAFE.decode(username).ok()?).ok()?;

            let mut username = username.split(':');

            let cipher = username.next().map(String::from)?;
            let password = username.next().map(String::from)?;

            let sip003 = link
                .query()
                .and_then(|q| q.parse::<UriQueries>().ok())
                .and_then(|q| q.get("plugin").and_then(|v| v.first().cloned()))
                .and_then(|plugin_str| Sip003::from_plugin_str(&plugin_str));

            let port = link.port().and_then(|p| p.as_str().parse::<i32>().ok())?;

            Some(Shadowsocks {
                name: s.split('#').skip(1).next().map(String::from),
                server,
                port,
                password,
                cipher,
                plugin: sip003,
            })
        }

        unimplemented!()
    }
}

struct Sip003 {
    name: String,
    opts: Sip003Opts,
}

impl Sip003 {
    fn from_plugin_str(s: &str) -> Option<Self> {
        let fld: Vec<_> = s.splitn(2, ';').collect();
        let name = match *fld.first()? {
            "obfs-local" | "simpleobfs" => String::from("simple-obfs"),
            x => x.to_string(),
        };

        Some(Self {
            name,
            opts: Sip003Opts::from_plugin_str(fld.get(1)?),
        })
    }
}

#[derive(Default)]
struct Sip003Opts {
    tls: String,
    obfs: String,
    host: String,
    path: String,
    impf: String,
}

impl Sip003Opts {
    fn from_plugin_str(s: &str) -> Self {
        let mut opts = Self::default();

        for item in s.split(';') {
            let vals = item.splitn(2, '=').collect::<Vec<_>>();

            let a = vals[0];
            let b = || {
                vals.get(1)
                    .map(|v| String::from(*v))
                    .unwrap_or_else(String::new)
            };

            match a {
                "tls" => opts.tls = String::from("tls"),
                "obfs" | "mode" => opts.obfs = b(),
                "obfs-path" | "obfs-uri" | "path" => {
                    let b = b();
                    if b.starts_with('/') {
                        opts.path = b
                    } else {
                        opts.path = format!("/{b}")
                    }
                }
                "obfs-host" | "host" => opts.host = b(),
                "impl" => opts.impf = b(),
                _ => continue,
            }
        }

        opts
    }
}
