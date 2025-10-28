markdown
0xV4N17Y-R4V3R 💥
_Brute-force the blockchain. One hex at a time._
Forge your digital identity with precision and rage.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust) ![License](https://img.shields.io/badge/license-MIT-blue) ![Status](https://img.shields.io/badge/status-alpha-red)

ascii
▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
█ 0x64D3........................23f4 █
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
▲ That's the only address I need. The rest can burn.

0xV4N17Y-R4V3R is a minimal, high-performance, configurable Ethereum vanity address generator built in Rust. Define your desired prefix and suffix via .env, then brute-force until destiny matches your hash.
You're not generating an address.
You're forging an identity.

🔥 Features
✅ Configurable via .env — set PREFIX and SUFFIX in seconds
✅ Pure Rust — no Node.js, no Python, just metal and math
✅ Lightweight & Fast — powered by k256 and sha3, zero bloat
✅ Cyberpunk Ethos — CLI as cathedral, brute force as art
✅ Offline Key Generation — your private keys never touch the network

🛠 Installation & Build

bash
git clone https://github.com/yourname/0xV4N17Y-R4V3R.git
cd 0xV4N17Y-R4V3R
cargo build --release

Binary: target/release/0xV4N17Y-R4V3R

🧪 Usage
1. Configure Your Target

Create a .env file:

env
PREFIX=0x64D3
SUFFIX=23f4

🎯 Goal: Generate an Ethereum address like 0x64D3...23f4
⚠️ Warning: Longer patterns = exponentially longer search times.
Matching both 0x64D3 and 23f4 ≈ ~4.3 billion attempts on average.

2. Start the Brute-Force Rave

bash
cargo run --release

Sample Output:

Starting search for address beginning with '0x64D3' and ending with '23f4'...
Attempted 100,000 keys...
Attempted 200,000 keys...
✅ Match found!
Attempts: 283,712
Time: 2.834s
🎉 Address: 0x64d3a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f823f4
🔑 Private Key: 0x8c4b3f2a1d0e4f8a7b2c9d1e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a

📁 Project Structure

0xV4N17Y-R4V3R/
├── src/main.rs # Core logic: keygen + matching
├── .env.example # Config template
├── .gitignore # Keep your .env private
├── Cargo.toml # Rust manifest
└── README.md # You're reading it

⚠️ Security Notice
🔐 Keys are generated locally — never transmitted, never stored.
🚫 Do not use generated addresses for mainnet funds unless you fully understand the risks.
💣 This tool is for educational and artistic purposes. No liability assumed for lost keys or funds.
🧠 Brute-forcing is CPU-intensive. May cause overheating on weak hardware. Proceed with caution.

🤖 Roadmap
[ ] Multi-threading support (rayon)
[ ] GPU acceleration (OpenCL / CUDA)
[ ] Resume from checkpoint (save progress)
[ ] Regex pattern matching (e.g. 0x[dead]{4}.*[beef]{4})
[ ] ASCII animation intro (hex rain on startup)

🎸 License

MIT License.
Code is freedom. Freedom is brute force.

_"At the edge of entropy, I found my address."_
— An anonymous terminal rave addict

🔧 Build your identity. Burn the CPU. Own the chain.

#cypherpunk #web3 #rust #ethereum #vanityaddress #0xV4N17Y-R4V3R
