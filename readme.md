# 🌐 Cloudflare DDNS Updater

A Rust-based tool that automatically updates a Cloudflare DNS A record to match your public IP. Perfect for home servers, Raspberry Pi setups, or any environment without a static IP.

---

## ✨ Features

- ✅ Automatically fetches your public IP
- ✅ Updates DNS A record using Cloudflare API
- ✅ Configurable via a `config.toml` file
- ✅ Systemd-compatible with `.service` and `.timer` support
- ✅ `.deb` packaging with `cargo-deb` for easy deployment

---

## 📦 Installation

### 1. **Install From `.deb`**

After building (or downloading) the `.deb` file:

```bash
sudo dpkg -i cloudflare-ddns_0.1.0_armhf.deb
```

### 2. **Configuration**

Edit the config file at:

```bash
/etc/cloudflare-ddns/config.toml
```

Example:

```toml
zone_id = "your-cloudflare-zone-id"
record_name = "your.domain.com"
auth_email = "you@example.com"
auth_key = "your-global-api-key"
```

---

## ⚙️ Usage

Run manully:

```bash
cloudflare-ddns
```

Or as a service using systemd:

```bash
sudo systemctl enable --now cloudflare-ddns.timer
```

Check status:

```bash
systemctl status cloudflare-ddns.timer
```

---

## 🛠 Building From Source

```bash
cargo build --release
```

To build a .deb package:

```bash
cargo install cargo-deb
cargo deb
```

---

## 📁 Directory Layout

```
cloudflare-ddns/
├── Cargo.toml
├── src/main.rs
├── config/config.toml
├── systemd/cloudflare-ddns.service
├── systemd/cloudflare-ddns.timer
```

---

## 🧪 Testing

Make sure to test:
-	Public IP fetch works
-	DNS record updates correctly
-	Systemd service runs on schedule