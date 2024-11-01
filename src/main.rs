use openssl::rsa::Rsa;
use openssl::sign::Signer;
use openssl::hash::MessageDigest;
use serde::de::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use base64::encode;
use reqwest::Client;

mod network;
use network::BlockData;
mod instance;
use instance::config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    send_block_request().await
}

async fn send_block_request() -> Result<(), Box<dyn std::error::Error>> {
    let command = "START";
    let signature = network::sign_command(command); // 명령어에 대한 서명 생성
    
    let block_data = BlockData {
        ip: "172.20.10.3".to_string(),
        command: command.to_string(),
        sign: signature,
    };

    let client = Client::new();
    let response = client
        .post("http://172.20.10.3:9999/add-block")  // URL도 수정했습니다
        .json(&block_data)
        .send()
        .await?;

    println!("Response: {:?}", response.status());
    Ok(())
}