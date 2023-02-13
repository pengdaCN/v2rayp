use crate::common::UriQueries;
use crate::core::ServiceObject;
use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};

pub struct Shadowsocks {
    name: Option<String>,
    server: String,
    port: i32,
    password: String,
    cipher: String,
    plugin: Option<Sip003>,
}

impl ServiceObject for Shadowsocks {
    fn set_name(&mut self, name: &str) {
        self.name.replace(String::from(name));
    }

    fn get_name(&self) -> &str {
        if let Some(name) = self.name.as_ref() {
            name
        } else {
            ""
        }
    }

    fn get_port(&self) -> i32 {
        self.port
    }

    fn get_hostname(&self) -> &str {
        &self.server
    }

    fn get_protocol(&self) -> &str {
        Shadowsocks::PROTOCOL
    }

    fn proto_to_show(&self) -> String {
        let ciph = match self.cipher.as_str() {
            "chacha20-ietf-poly1305" | "chacha20-poly1305" => "c20p1305",
            _ => self.cipher.as_str(),
        };

        if let Some(plugin_name) = self.plugin.as_ref().map(|v| v.name.as_str()) {
            format!("SS({ciph}-{plugin_name})")
        } else {
            format!("SS({ciph})")
        }
    }

    fn need_plugin_port(&self) -> bool {
        self.plugin.is_some()
    }

    fn configuration(&self) {
        todo!()
    }
}

impl Shadowsocks {
    const PROTOCOL: &'static str = "shadowsocks";

    pub fn from_ss_url(link: &str) -> Result<Self> {
        fn parse(s: &str) -> Option<Shadowsocks> {
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

        parse(link)
            .or_else(|| {
                let (_, link) = link.split_at(5);

                let parts: Vec<_> = link.splitn(2, '#').collect();
                let first = parts.first().unwrap();
                let l = general_purpose::STANDARD
                    .decode(first)
                    .ok()
                    .or_else(|| general_purpose::URL_SAFE.decode(first).ok())
                    .map(|x| String::from_utf8(x).ok())??;

                let mut l = vec![String::from("s://"), l].concat();
                if let Some(x) = parts.get(1) {
                    l = vec![l, String::from(*x)].join("#");
                }

                parse(&l)
            })
            .context("invalid parameters unrecognized ss address")
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
