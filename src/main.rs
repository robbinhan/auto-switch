extern crate reqwest;
extern crate base64;
//extern crate async_std;
extern crate serde;
extern crate serde_json;
extern crate rand;

mod vmess;

use std::str;
use serde::{Serialize, Deserialize};
use tokio::timer::Interval;
use std::time::{Instant, Duration};
use rand::Rng;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;


/**
{
  "routing": {
    "name": "all_to_main",
    "domainStrategy": "AsIs",
    "rules": [
      {
        "type": "field",
        "outboundTag": "台湾 BGP 04",
        "port": "0-65535"
      }
    ]
  },
  "inbounds": [
    {
      "listen": "127.0.0.1",
      "protocol": "socks",
      "settings": {
        "ip": "127.0.0.1",
        "auth": "noauth",
        "udp": false
      },
      "tag": "socksinbound",
      "port": 1080
    },
    {
      "listen": "0.0.0.0",
      "protocol": "http",
      "settings": {
        "timeout": 0
      },
      "tag": "httpinbound",
      "port": 1081
    }
  ],
  "dns": {
    "servers": [
      "localhost"
    ]
  },
  "log": {
    "loglevel": "none"
  },
  "outbounds": [
    {
      "sendThrough": "0.0.0.0",
      "mux": {
        "enabled": false,
        "concurrency": 8
      },
      "protocol": "vmess",
      "settings": {
        "vnext": [
          {
            "address": "pccw5.0tk8a3a1q4t94dler.com",
            "users": [
              {
                "id": "b275aabc-4ffe-36a1-92f5-6fa13d5f35d2",
                "alterId": 64,
                "security": "auto",
                "level": 0
              }
            ],
            "port": 153
          }
        ]
      },
      "tag": "台湾 BGP 04",
      "streamSettings": {
        "sockopt": {},
        "quicSettings": {
          "key": "",
          "security": "none",
          "header": {
            "type": "none"
          }
        },
        "tlsSettings": {
          "allowInsecure": false,
          "alpn": [
            "http/1.1"
          ],
          "serverName": "server.cc",
          "allowInsecureCiphers": false
        },
        "wsSettings": {
          "path": "/",
          "headers": {
            "Host": "9f5b336107.xwdsh.xyz"
          }
        },
        "httpSettings": {
          "path": "",
          "host": [
            ""
          ]
        },
        "tcpSettings": {
          "header": {
            "type": "none"
          }
        },
        "kcpSettings": {
          "header": {
            "type": "none"
          },
          "mtu": 1350,
          "congestion": false,
          "tti": 20,
          "uplinkCapacity": 5,
          "writeBufferSize": 1,
          "readBufferSize": 1,
          "downlinkCapacity": 20
        },
        "security": "none",
        "network": "ws"
      }
    }
  ]
}
**/
#[derive(Serialize, Deserialize, Debug)]
struct RoutingRule {
    r#type: String,
    outboundTag: String,
    port: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Routing {
    name: String,
    domainStrategy: String,
    rules: Vec<RoutingRule>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InboundSetting {
    ip: String,
    auth: String,
    udp: bool,
    timeout: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Inbound {
    listen: String,
    protocol: String,
    settings: InboundSetting,
    tag: String,
    port: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct DNS {
    servers: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    loglevel: String
}

#[derive(Serialize, Deserialize, Debug)]
struct VmessJsonConfig {
    routing: Routing,
    inbounds: Vec<Inbound>,
    dns: DNS,
    log: Log,
    outbounds: Vec<vmess::outbound::Outbound>,
}

const SUB_URL: &str = "https://dler.cloud/subscribe/tWt8ThVD9uJlPEcx?mu=av2";


#[tokio::main]
async fn main() {

    // test gfw timer
    let mut loop_interval = Interval::new(Instant::now() + Duration::from_secs(60), Duration::from_secs(60));

    loop {
        let instant = loop_interval.next().await.unwrap();

        println!("loop_interval: {:?}", instant);

        match test_gfw().await {
            Ok(_res) => {
                println!("test gfw ok");
            }
            Err(err) => {
                println!("test gfw fail,{:?}", err);
                let _ = switch_config().await;
            }
        };
    }
}

// 请求vmess订阅地址
async fn get_vemsses(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()?
        .get(url)
        .send().await?;
//
//    println!("Status: {}", res.status());

    let body = res.text().await?;


    Ok(body)
}

// 测试gfw
async fn test_gfw() -> Result<bool, reqwest::Error> {
    let res = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .proxy(reqwest::Proxy::https("http://127.0.0.1:1081")?)
        .proxy(reqwest::Proxy::http("http://127.0.0.1:1081")?)
        .build()?
        .get("https://medium.com/")
        .send().await?;

    println!("Status: {}", res.status());


    Ok(res.status() == 200)
}

// 切换配置
async fn switch_config() -> Result<String, reqwest::Error> {
    // get sub body
    let body = get_vemsses(SUB_URL).await?;
//    println!("Body:\n\n{}", body);

    // base64decode
    let all_v2mess_vec = base64::decode(&body).unwrap();
    let all_v2mess = str::from_utf8(&all_v2mess_vec).unwrap();

//    println!("\nall_v2mess :{}", all_v2mess);

    let all_v2mess_array: Vec<&str> = all_v2mess.split("\n").collect();

    let mut vmess_subscribe_config_array: Vec<vmess::vmesssubscribeconfig::VmessSubscribeConfig> = Vec::new();
    for v2mess in all_v2mess_array {
        if !v2mess.is_empty() {
//            println!("\nv2mess :{}", v2mess);

            let vmess_base64 = v2mess.replace("vmess://", "");
            let decode_vmess_base64 = base64::decode(&vmess_base64).unwrap();
            let vmess_json_config = str::from_utf8(&decode_vmess_base64).unwrap();

//            println!("\nvmess_json_config :{}", vmess_json_config);

            // Convert the JSON string back to a Point.
            let deserialized: vmess::vmesssubscribeconfig::VmessSubscribeConfig = serde_json::from_str(&vmess_json_config).unwrap();

            if deserialized.ps.contains("香港") || deserialized.ps.contains("台湾") {
                // Prints deserialized = Point { x: 1, y: 2 }
//                println!("deserialized = {:?}", deserialized);
                vmess_subscribe_config_array.push(deserialized);
            }
        }
    }

    let config_len = vmess_subscribe_config_array.len() as i32;
    if config_len > 0 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, config_len) as usize;
        println!("Integer: {:?}", x);
        let deserialized: &vmess::vmesssubscribeconfig::VmessSubscribeConfig = vmess_subscribe_config_array.get(x).unwrap();
        println!("deserialized = {:?}", deserialized);

        let mut route_rules: Vec<RoutingRule> = Vec::new();

        let route_rule = RoutingRule {
            r#type: "field".to_owned(),
            outboundTag: deserialized.ps.clone(),
            port: "0-65535".to_owned(),
        };

        route_rules.push(route_rule);

        let vmess_json_config = VmessJsonConfig {
            routing: Routing {
                name: "all_to_main".to_owned(),
                domainStrategy: "AsIs".to_owned(),
                rules: route_rules,
            },
            inbounds: gen_inbounds().unwrap(),
            dns: DNS {
                servers: vec![
                    "localhost".to_owned()
                ]
            },
            log: Log {
                loglevel: "none".to_owned()
            },
            outbounds: vmess::outbound::gen_outbounds(deserialized).unwrap(),
        };

        println!("final vmess_json_config = {:?}", vmess_json_config);

        // to json
        let vmess_json_config_string = serde_json::to_string(&vmess_json_config).unwrap();
        println!("final vmess_json_config_string = {:?}", vmess_json_config_string);
        // store to file

        let path = Path::new("v2ray.json");
        let display = path.display();

        // 以只写模式打开文件，返回 `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {:?}", display, why),
            Ok(file) => file,
        };

        // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
        match file.write_all(vmess_json_config_string.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {:?}", display, why)
            }
            Ok(_) => {
                println!("successfully wrote to {}", display);

                let output = Command::new("sudo")
                    .arg("systemctl")
                    .arg("restart")
                    .arg("v2ray")
                    .output()
                    .expect("failed to execute process");

                println!("systemctl output :{:?}", output.stdout)
            }
        }
    }


    Ok("ok".to_owned())
}


fn gen_inbounds() -> Option<Vec<Inbound>> {
    let mut inbounds: Vec<Inbound> = Vec::new();

    let inbound_socks = Inbound {
        listen: "0.0.0.0".to_owned(),
        protocol: "socks".to_owned(),
        settings: InboundSetting {
            ip: "0.0.0.0".to_owned(),
            auth: "noauth".to_owned(),
            udp: false,
            timeout: 0,
        },
        tag: "socksinbound".to_owned(),
        port: 1080,
    };

    inbounds.push(inbound_socks);

    let inbound_http = Inbound {
        listen: "0.0.0.0".to_owned(),
        protocol: "http".to_owned(),
        settings: InboundSetting {
            ip: "0.0.0.0".to_owned(),
            auth: "noauth".to_owned(),
            udp: false,
            timeout: 0,
        },
        tag: "httpinbound".to_owned(),
        port: 1081,
    };

    inbounds.push(inbound_http);

    Some(inbounds)
}

