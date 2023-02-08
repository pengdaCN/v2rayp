use anyhow::{bail, Context, Result};
use http::uri::Uri;
use static_init::dynamic;
use base64::{Engine as _, engine::general_purpose};
use crate::core::ServiceObject;

#[dynamic]
static DEF_HTTP_CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();

pub fn import(url: &str) -> Result<Vec<Box<dyn ServiceObject>>> {
    let url = url.parse::<Uri>()?;
    let link = match url.scheme_str().context("unsupported not scheme")? {
        "http" | "https" => {
            url
        }
        _ => {
            bail!("do not support other agreements for now")
        }
    };

    let raw = DEF_HTTP_CLIENT.get(link.to_string())
        .send()
        .with_context(|| format!("请求订阅地址失败: {}", &link))?
        .text()
        .with_context(|| format!("不能解析订阅响应: {}", &link))?;

    let raw = trim_bom(&raw);

    // 解析base64
    let data = general_purpose::STANDARD.decode(raw).or_else(|_| {
        general_purpose::URL_SAFE.decode(raw)
    }).context("不能解析订阅的base64格式")?;
    let lines = String::from_utf8(data).context("订阅数据不是合法的utf8")?;

    // 解析订阅地址

    unimplemented!()
}

fn resolve_lines(s: &str)

fn trim_bom(s: &str) -> &str {
    s.strip_prefix("\xef\xbb\xbf").and_then(|x| x.strip_suffix("\xef\xbb\xbf")).unwrap()
}