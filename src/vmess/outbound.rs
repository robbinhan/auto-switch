use crate::vmess::vmesssubscribeconfig::VmessSubscribeConfig;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Mux {
    enabled: bool,
    concurrency: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    alterId: i32,
    security: String,
    level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Vnext {
    address: String,
    users: Vec<User>,
    port: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundSettings {
    vnext: Vec<Vnext>
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundSockopt {}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundQuicSettingsHeader {
    r#type: String
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundQuicSettings {
    key: String,
    security: String,
    header: OutboundQuicSettingsHeader,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundTlsSettings {
    allowInsecure: bool,
    alpn: Vec<String>,
    serverName: String,
    allowInsecureCiphers: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundWSSettingsHeaders {
    Host: String
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundWSSettings {
    path: String,
    headers: OutboundWSSettingsHeaders,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundHttpSettings {
    path: String,
    host: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundTcpSettings {
    header: OutboundQuicSettingsHeader
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundKcpSettings {
    header: OutboundQuicSettingsHeader,
    mtu: i32,
    congestion: bool,
    tti: i32,
    uplinkCapacity: i32,
    writeBufferSize: i32,
    readBufferSize: i32,
    downlinkCapacity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutboundStreamSettings {
    sockopt: OutboundSockopt,
    quicSettings: OutboundQuicSettings,
    tlsSettings: OutboundTlsSettings,
    wsSettings: OutboundWSSettings,
    httpSettings: OutboundHttpSettings,
    tcpSettings: OutboundTcpSettings,
    kcpSettings: OutboundKcpSettings,
    security: String,
    network: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Outbound {
    sendThrough: String,
    mux: Mux,
    protocol: String,
    settings: OutboundSettings,
    tag: String,
    streamSettings: OutboundStreamSettings,
}

pub fn gen_outbounds(deserialized: &VmessSubscribeConfig) -> Option<Vec<Outbound>> {
    let mut outbounds: Vec<Outbound> = Vec::new();

    let mux = Mux {
        enabled: false,
        concurrency: 8,
    };

    let mut users = Vec::new();

    let user = User {
        id: deserialized.id.clone(),
        alterId: deserialized.aid.clone().parse::<i32>().unwrap(),
        security: "auto".to_owned(),
        level: 0,
    };

    users.push(user);

    let outbound_settings = OutboundSettings {
        vnext: vec![Vnext {
            address: deserialized.add.clone(),
            users,
            port: deserialized.port.clone(),
        }]
    };

    let stream_settings = OutboundStreamSettings {
        sockopt: OutboundSockopt {},
        quicSettings: OutboundQuicSettings {
            key: "".to_owned(),
            security: "none".to_owned(),
            header: OutboundQuicSettingsHeader {
                r#type: "none".to_owned(),
            },
        },
        tlsSettings: OutboundTlsSettings {
            allowInsecure: false,
            alpn: vec![
                "http/1.1".to_owned(),
            ],
            serverName: "server.cc".to_owned(),
            allowInsecureCiphers: false,
        },
        wsSettings: OutboundWSSettings {
            path: "/".to_owned(),
            headers: OutboundWSSettingsHeaders {
                Host: deserialized.host.clone()
            },
        },
        httpSettings: OutboundHttpSettings {
            path: "".to_owned(),
            host: vec![
                "".to_owned(),
            ],
        },
        tcpSettings: OutboundTcpSettings {
            header: OutboundQuicSettingsHeader {
                r#type: "none".to_owned(),
            }
        },
        kcpSettings: OutboundKcpSettings {
            header: OutboundQuicSettingsHeader {
                r#type: "none".to_owned(),
            },
            mtu: 1350,
            congestion: false,
            tti: 20,
            uplinkCapacity: 5,
            writeBufferSize: 1,
            readBufferSize: 1,
            downlinkCapacity: 20,
        },
        security: "none".to_owned(),
        network: "ws".to_owned(),
    };

    let outbound = Outbound {
        sendThrough: "0.0.0.0".to_owned(),
        mux,
        protocol: "vmess".to_owned(),
        settings: outbound_settings,
        tag: deserialized.ps.clone(),
        streamSettings: stream_settings,
    };

    outbounds.push(outbound);

    Some(outbounds)
}