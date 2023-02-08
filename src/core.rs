use serde::Deserialize;

pub mod shadowsocks;

pub trait ServiceObject {
    fn set_name(&mut self, name: &str);
    fn get_name(&self) -> &str;
    fn get_port(&self) -> i32;
    fn get_hostname(&self) -> &str;
    fn get_protocol(&self) -> &str;
    fn proto_to_show(&self) -> &str;
    fn need_plugin_port(&self) -> bool;
    fn configuration(&self);
}

#[derive(Deserialize)]
pub struct SIP008 {
    pub version: i32,
    pub username: String,
    pub user_uuid: String,
    pub bytes_used: u64,
    pub bytes_remained: u64,
    pub servers: Vec<sip008::Service>,
}

pub mod sip008 {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Service {
        pub server: String,
        pub server_port: i32,
        pub password: String,
        pub method: String,
        pub plugin: String,
        pub plugin_opts: String,
        pub remarks: String,
        pub id: String,
    }
}