# solana-vanity-address
Small, fast Rust tool to brute-force Solana public keys that start with one or more Base58 prefixes. Uses rayon for parallel generation, an atomic counter for low-overhead metrics, and a dedicated reporter thread that prints per-second throughput. Outputs the matching public key and its private key encoded in Base58.
