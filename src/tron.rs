use secp256k1::{Secp256k1, SecretKey};
use rand::rngs::OsRng;
use sha3::{Digest, Keccak256};
use dotenv::dotenv;
use std::env;
use bs58;
use tokio;
use reqwest;
use serde_json::json;
use hex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let suffixes_str = env::var("TRON_ADDRESS_SUFFIXES")
        .expect("请在 .env 文件中设置 TRON_ADDRESS_SUFFIXES");
    let suffixes: Vec<String> = suffixes_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if suffixes.is_empty() {
        panic!("TRON_ADDRESS_SUFFIXES 不能为空！");
    }

    println!("🚀 启动多线程地址生成器（区分大小写）...");
    println!("目标后缀: {:?}", suffixes);

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    println!("使用 {} 个线程并发生成...", num_threads);

    let bot_token = env::var("BOT_TOKEN")
        .expect("请在 .env 文件中设置 BOT_TOKEN");

    let chat_id_str = env::var("CHAT_ID")
        .expect("请在 .env 文件中设置 CHAT_ID");

    let chat_id: i64 = chat_id_str.parse().unwrap();


    // 共享的后缀列表和原子计数器
    let suffixes_arc = Arc::new(suffixes);
    let total_attempts = Arc::new(AtomicU64::new(0));

    // 通道：工作线程 → 主线程（用于发送 Telegram）
    let (tx, mut rx) = tokio::sync::mpsc::channel::<MatchResult>(100);

    // 启动多个工作线程
    for i in 0..num_threads {
        let suffixes_clone = Arc::clone(&suffixes_arc);
        let attempts_clone = Arc::clone(&total_attempts);
        let tx_clone = tx.clone();

        std::thread::spawn(move || {
            let secp = Secp256k1::new();
            let mut rng = OsRng;

            loop {
                attempts_clone.fetch_add(1, Ordering::Relaxed);

                // 生成私钥
                let private_key = SecretKey::new(&mut rng);
                let private_key_hex = hex::encode(private_key.as_ref());

                // 公钥（uncompressed，去掉 0x04）
                let public_key = private_key.public_key(&secp);
                let public_key_bytes = &public_key.serialize_uncompressed()[1..];

                // Keccak256
                let mut hasher = Keccak256::new();
                hasher.update(public_key_bytes);
                let hash = hasher.finalize();
                let address_bytes = &hash[hash.len() - 20..];

                // 构造 Tron 地址
                let mut tron_address = vec![0x41];
                tron_address.extend_from_slice(address_bytes);

                // 校验和
                let checksum = double_sha256(&tron_address)[..4].to_vec();
                let mut addr_with_checksum = tron_address;
                addr_with_checksum.extend(checksum);

                // Base58 编码
                let address = bs58::encode(addr_with_checksum).into_string();

                // 检查是否匹配任意后缀（区分大小写）
                for suffix in suffixes_clone.iter() {
                    if address.ends_with(suffix) {
                        let result = MatchResult {
                            address,
                            private_key: private_key_hex,
                            matched_suffix: suffix.clone(),
                        };

                        // 尝试发送到通道（如果通道满则丢弃，避免阻塞）
                        if let Err(_) = tx_clone.try_send(result) {
                            eprintln!("⚠️ 通道已满，丢弃一个匹配结果（建议增加通道容量）");
                        }

                        break; // 找到一个即可，继续下一轮
                    }
                }
            }
        });
    }

    // 主线程：监听匹配结果并发送 Telegram
    println!("✅ 已启动 {} 个工作线程，等待匹配...", num_threads);
    while let Some(result) = rx.recv().await {
        let total = total_attempts.load(Ordering::Relaxed);
        let message = format!(
            "✅ 找到匹配地址！\n\n地址: <code>{}</code>\n私钥: <code>{}</code>\n匹配后缀: {}\n总尝试次数: {}",
            result.address, result.private_key, result.matched_suffix, total
        );

        println!("\n🎯 匹配成功！");
        println!("地址: {}", result.address);
        println!("私钥: {}", result.private_key);
        println!("匹配后缀: {}", result.matched_suffix);
        println!("总尝试次数: {}", total);

        // 发送 Telegram（可失败重试，这里简化）
        let token = bot_token.clone();
        let msg = message.clone();
        let id = chat_id;
        tokio::spawn(async move {
            match send_telegram_message(&token, id, &msg).await {
                Ok(()) => println!(" "),
                Err(e) => println!(" "),
            }
        });
    }
}

#[derive(Debug)]
struct MatchResult {
    address: String,
    private_key: String,
    matched_suffix: String,
}

fn double_sha256(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest as _, Sha256};
    let first = Sha256::digest(data);
    Sha256::digest(&first).to_vec()
}

async fn send_telegram_message(
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
