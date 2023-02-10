use crate::core::ServiceObject;
use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose, Engine as _};
use http::uri::Uri;
use static_init::dynamic;

#[dynamic]
static DEF_HTTP_CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();

pub fn import(url: &str) -> Result<Vec<Box<dyn ServiceObject>>> {
    let url = url.parse::<Uri>()?;
    let link = match url.scheme_str().context("unsupported not scheme")? {
        "http" | "https" => url,
        _ => {
            bail!("do not support other agreements for now")
        }
    };

    let raw = DEF_HTTP_CLIENT
        .get(link.to_string())
        .send()
        .with_context(|| format!("请求订阅地址失败: {}", &link))?
        .text()
        .with_context(|| format!("不能解析订阅响应: {}", &link))?;

    // 解析base64
    let data = general_purpose::STANDARD
        .decode(&raw)
        .or_else(|_| general_purpose::URL_SAFE.decode(&raw))
        .context("不能解析订阅的base64格式")?;
    let lines = String::from_utf8(data).context("订阅数据不是合法的utf8")?;

    // 解析订阅地址

    unimplemented!()
}

fn resolve_lines(s: &str) {}
