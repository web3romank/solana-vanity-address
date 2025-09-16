# solana-vanity-address
Small, fast Rust tool to brute-force Solana public keys that start with one or more Base58 prefixes. Uses rayon for parallel generation, an atomic counter for low-overhead metrics, and a dedicated reporter thread that prints per-second throughput. Outputs the matching public key and its private key encoded in Base58.

# How to set up
First of all, open cmd, and git clone the repository

Than open cmd in very that folder and paste into it the folowing command: cargo run --release -- pump solana
Where the "pump" and "solana" are the desired prefixes you want to find. You can add as much of them as ypu want. Then press enter and the script will start downloading all the dependecies. After the successful instalation the proces of brutting will start. You will see something like Checked addresses: 2709157 | 420495.93 addr/s. When the script finds one of the prefixes, the brutting stops and you will see both public and private keys in console. Make sure to copy them and save in a safe place. The moment after you close the console you will lose them for an eternity...
