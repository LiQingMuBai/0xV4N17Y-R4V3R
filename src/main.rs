use std::env;
use std::time::Instant;
use rand::Rng;
use ethereum_types::H160;
use sha3::{Digest, Keccak256};
use hex::encode;
use reqwest;
use serde_json::json;

use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let prefix = env::var("PREFIX").unwrap_or_else(|_| "0x".to_string());
    let suffix = env::var("SUFFIX").unwrap_or_else(|_| "".to_string());

    println!("Searching for address with:");
    println!("Prefix: {}", prefix);
    println!("Suffix: {}", suffix);

    let bot_token = env::var("BOT_TOKEN")
        .expect("请在 .env 文件中设置 BOT_TOKEN");

    let chat_id_str = env::var("CHAT_ID")
        .expect("请在 .env 文件中设置 CHAT_ID");

    let chat_id: i64 = chat_id_str.parse().unwrap();

    let start_time = Instant::now();
    let mut attempts = 0;

    loop {
        attempts += 1;
        let (address, private_key) = generate_address();

        let address_hex = encode(address.as_bytes());
        let full_address = format!("0x{}", address_hex);
        // println!("Address: {}", full_address);
        if full_address.starts_with(&prefix) && full_address.ends_with(&suffix) {
            println!("\nFound matching address after {} attempts!", attempts);
            println!("Address: {}", full_address);
            println!("Private Key: 0x{}", encode(private_key));
            println!("Time taken: {:?}", start_time.elapsed());



            let message = format!(
                "✅ 找到匹配的以太坊地址！\n\
                 地址: {}\n\
                 私钥: 0x{}\n\
                 尝试次数: {}\n\
                 耗时: {:?}",
                full_address,
                encode(private_key),
                attempts,
                start_time.elapsed()
            );

            let rt = Runtime::new()?;
            rt.block_on(send_telegram_message(&bot_token, chat_id, &message))?;

            return Ok(());
        }

        if attempts % 1_000_000 == 0 {
            println!("Attempts: {}M", attempts / 1_000_000);
        }
    }
}

fn generate_address() -> (H160, [u8; 32]) {
    let mut rng = rand::thread_rng();
    let mut private_key = [0u8; 32];
    rng.fill(&mut private_key);

    let public_key = secp256k1::PublicKey::from_secret_key(
        &secp256k1::SecretKey::parse_slice(&private_key).unwrap()
    );

    let public_key_bytes = public_key.serialize();
    let hash = Keccak256::digest(&public_key_bytes[1..]); // 跳过0x04前缀
    let address_bytes = &hash[12..];
    let address = H160::from_slice(address_bytes);

    (address, private_key)
}

mod secp256k1 {
    use k256::{Secp256k1, SecretKey as K256SecretKey, PublicKey as K256PublicKey};
    use k256::elliptic_curve::sec1::ToEncodedPoint;

    pub struct SecretKey(K256SecretKey);
    pub struct PublicKey(K256PublicKey);

    impl SecretKey {
        pub fn parse_slice(data: &[u8]) -> Result<Self, ()> {
            K256SecretKey::from_slice(data)
                .map(SecretKey)
                .map_err(|_| ())
        }
    }

    impl PublicKey {
        pub fn from_secret_key(secret_key: &SecretKey) -> Self {
            PublicKey(secret_key.0.public_key())
        }

        pub fn serialize(&self) -> Vec<u8> {
            self.0.to_encoded_point(false).as_bytes().to_vec()
        }
    }
}

pub async fn send_telegram_message(
    bot_token: &str,
    chat_id: i64,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let client = reqwest::Client::new();
    let payload = json!({
        "chat_id": chat_id,
        "text": message,
        "parse_mode": "HTML"
    });

    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        let status = res.status();
        let body = res.text().await.unwrap_or_else(|_| "无法读取错误响应".to_string());
        Err(format!("Telegram API 请求失败: {} - {}", status, body).into())
    }
}
