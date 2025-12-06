# 🔐 jwt-secret-gen

**jwt-secret-gen** adalah command-line tool super ringan dan sangat cepat yang ditulis dalam **Rust** untuk menghasilkan **JWT secret** (HS256 / HS384 / HS512) secara **cryptographically secure**.

Tool ini sangat cocok untuk aplikasi backend, microservices, atau deployment yang membutuhkan secret unik, aman, dan mudah di-generate.

---

## 🚀 Fitur Utama

- 🔒 Menghasilkan secret cryptographically secure menggunakan `OsRng`
- 📦 Output dalam beberapa format:
  - Base64
  - URL-safe Base64 (tanpa padding)
  - Hexadecimal
- 🧪 Mendukung multiple secrets (`--count`)
- 📝 Output ke console atau ke file (`--out`)
- ⚡ Sangat cepat (Rust FTW!)
- 🖥️ CLI modern dengan Clap v4
- 📜 Lisensi MIT

---

## 📥 Instalasi

```bash
git clone https://github.com/yusrilarzaqi/jwt-secret-gen
cd jwt-secret-gen
cargo build --release

```

## 🎯 Cara Menggunakan

```bash
./target/release/jwt-secret-gen
```

### Contoh Output

```
pUnYT9mSqrBwZ3SqCDGt41XHkcYoVdSJITYnlykc870=
```

### Generate secret 64 bytes dalam format base64

```bash
jwt-secret-gen --bytes 64 --format base64
```

### Generate 3 secret dan simpan ke file

```bash
jwt-secret-gen -c 3 -o ./secrets.txt
```

### Format yang tersedia

- Base64
- URL-safe Base64
- Hexadecimal

## Help & Dokumentasi

```bash
jwt-secret-gen --help
```

```
A blazing-fast JWT secret generator written in Rust.
Generates cryptographically secure secrets for HS256/HS384/HS512.

Usage: jwt-secret-gen [OPTIONS]

Options:
  -b, --bytes <BYTES>       Number of random bytes to generate [default: 32]
  -f, --format <FORMAT>     Output format (base64 | urlsafe | hex)
  -c, --count <COUNT>       Generate multiple secrets [default: 1]
  -o, --out <OUT>           Save output to file (append)
      --newline <NEWLINE>   Add newline after each secret [default: true]
  -h, --help                Print help
  -V, --version             Print version
```
