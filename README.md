# Genesis - Embedded OTA Library

Genesis is a Rust-powered library for **secure Over-The-Air (OTA) updates** and configuration management on ESP32-C3 devices. Think of it as your firmware's fairy godmother – with cryptographic paranoia, atomic update magic, and async wizardry courtesy of Embassy runtime.

## Why Bother?

Because manually flashing devices at 3 a.m. while swearing under your breath is so last decade. Genesis handles:

* 🔐 **Secure Updates**: Every firmware signed and verified via GPG. No trust issues here.
* 🔄 **Atomic Swaps**: Rollback-safe partition switching – you won’t brick it (probably).
* 📡 **Async Everything**: Powered by Embassy async runtime. Because blocking is for suckers.
* 📦 **Drop-In Design**: Integrates neatly into existing ESP32-C3 projects. Minimal hair-pulling.
* 🔧 **Dynamic Config**: Push config updates like a boss.
* 📊 **Update Progress**: Real-time OTA status so you can look busy while sipping coffee.

## Quick Start (Because Patience Is Overrated)

### Prereqs

Make sure you’ve got Rust and the ESP toolchain installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install espup
espup install
cargo install espflash cargo-espflash
```

Then fix your shell:

```bash
source ~/export-esp.sh  # Or add it to .bashrc, like an adult
```

### Add Genesis To Your Project

`Cargo.toml`:

```toml
[dependencies]
genesis = { path = "../genesis" } # or git if you’re feeling fancy
```

### Usage Sample

```rust
use genesis::{OtaClient, OtaConfig, Version};
use genesis::storage::Esp32C3Storage;
use genesis::verification::default_public_key;

let config = OtaConfig::new("https://your-server.local/ota")?
    .with_device_id("device-001")?
    .with_version(Version::new(1, 0, 0, 1));

let partition = Esp32C3Storage::get_update_partition()?;
let storage = Esp32C3Storage::new(partition);

let public_key = default_public_key()?;
let mut client = OtaClient::new(config, storage, public_key);

match client.check_update(socket, rx_buf, tx_buf).await {
    UpdateStatus::Available(manifest) => {
        println!("Update available: v{}", manifest.version);
        client.download_and_apply(manifest, socket, rx_buf, tx_buf).await?;
    }
    UpdateStatus::UpToDate => println!("Already up to date"),
    UpdateStatus::CheckFailed(e) => eprintln!("Check failed: {:?}", e),
}
```

## Nerdy Bits (Architecture)

* **OtaClient**: Does the heavy lifting
* **SignatureVerifier**: Keeps your firmware legit
* **UpdateStorage**: Talks to flash partitions (and doesn’t yell at them)
* **ConfigManager**: Manages device config like a digital butler
* **Manifest**: Metadata magic scroll

### Security Flow (Because We Like Sleeping At Night)

1. Firmware signed with a GPG key you trust (hopefully).
2. Public key baked into firmware (yes, at compile time).
3. SHA256 digest checks because we’re paranoid.
4. Atomic partition switching = no bricks, no tears.

### Update Dance

1. Poll for manifest
2. Verify signature
3. Download firmware
4. Verify integrity
5. Flash inactive partition
6. Update boot flags
7. Reboot into the new hotness
8. Auto-rollback if things explode

## Dev Life

* **Build:** `cargo build --release`
* **Test:** `cargo test --target x86_64-unknown-linux-gnu`
* **Docs:** `cargo doc --open`

## Contributing (aka Please Help)

1. Fork it
2. Branch it (`git checkout -b feature/magic`)
3. Commit it (`git commit -m 'Make it better'`)
4. Push it (`git push origin feature/magic`)
5. PR it

## License

This is free and unencumbered software released into the public domain. For more details, see [UNLICENSE](UNLICENSE).

## Thanks To

* [Embassy](https://embassy.dev/) for async goodness
* [esp-hal](https://github.com/esp-rs/esp-hal) for ESP32-C3 drivers
* [ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek) because crypto is scary
