<p align="center">
	<h2 align="center">🚀 Deployooor</h2>
  <h5 align="center">Securely deploy, verify, and log EVM smart contract deployments with a cross-platform desktop GUI. Supports Vyper, Aribtrum Stylus, and Solidity.</h5>
</p>

Deployooor is a cross-platform desktop application made to securely deploy, verify, and log smart contract deployments. Existing tooling for deploying smart contracts into production leaves a lot to be desired. The command line tools that exist today are unfriendly to the end user and typically require a handful of steps and different commands that users need to remember. Such tools do not log deployments nor are they secure by default. Our DevOps tooling simplifies the process of deploying, verifying, and logging smart contracts by providing a wonderful cross-platform GUI to the user. By enforcing the use of keystores we are providing a safer alternative to existing tools so that deployment keys cannot get jacked!

## Build From Source 

### Dependencies 

1. Vyper Compiler (add to PATH),
2. Python,
3. Rust,
4. sqlx-cli: `cargo sqlx-cli install`

#### Rust 

Linux/Mac/WSL: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Windows: Download rustup-init.exe from `https://www.rust-lang.org/tools/install`

#### Vyper

`pip install vyper`

#### Initialize SQLite:

`cargo sqlx database create --database-url sqlite://deployer.db`

`cargo sqlx migrate run --database-url sqlite://deployer.db`

Windows: `move deployer.db ./src-tauri` | Linux / Mac: `mv deployer.db ./src-tauri`

#### Frontend (Svelte): 

`npm install`

Create a .env file in the project root with `DATABASE_URL = sqlite://deployer.db` on the first line. 

To run this software in non-release mode: `cargo tauri dev`. 

### Optional: tauri-cli (for bundling into single installer):
`cargo install tauri-cli` && `cargo tauri build`