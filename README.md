# rust-restudy
## OverView

This repository is a sandbox to practice Rust for [keyno63](https://github.com/keyno63).

## Rust development setup (Windows)

This repository is initialized as a Rust binary project.

### First step

- Open a new terminal once after tool installation.
- If `cargo` is not found, run with full path: `C:\Users\<your-user>\.cargo\bin\cargo.exe`.

### Recommended commands in this repo

- Build: `.\dev-cargo.cmd build`
- Test: `.\dev-cargo.cmd test`
- Lint: `.\dev-cargo.cmd clippy -- -D warnings`
- Format check: `.\dev-cargo.cmd fmt -- --check`

## Basic auth HTTP server sample

This repo includes a sample server with:
- `GET /` (public)
- `GET /health` (public)
- `GET /private` (Basic auth required)

### Run

```powershell
$env:BASIC_AUTH_USER="admin"
$env:BASIC_AUTH_PASS="password"
.\dev-cargo.cmd run
```

`BIND_ADDR` can override bind address (default: `127.0.0.1:3000`).

### Access examples

```powershell
curl http://127.0.0.1:3000/
curl http://127.0.0.1:3000/private
curl -u admin:password http://127.0.0.1:3000/private
```

## LICENSE

This repository is MIT License.  
See [the License](./LICENSE) file.
