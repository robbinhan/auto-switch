use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mux {
    enabled: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct VmessSubscribeConfig {
    pub v: String,
    // version
    pub ps: String,
    // 名称
    pub add: String,
    // host
    pub port: i32,
    pub id: String,
    //
    pub aid: String,
    pub net: String,
    pub r#type: String,
    pub host: String,
    // 混淆参数
    pub path: String,
    pub mux: Mux,
}