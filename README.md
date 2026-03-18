# Weavefront

Self-hosted Web3 hosting panel. Deploy static sites to IPFS, Arweave, and Filecoin from a single dashboard.

## Features

- **Multi-protocol deploys** — IPFS (Pinata), Arweave (ArDrive Turbo), Filecoin (Lighthouse)
- **Archive upload** — upload `.zip` or `.tar.gz` archives, single files, or directories
- **Configurable gateways** — choose Cloudflare, Arweave.net, Lighthouse, or custom gateway URLs
- **Project management** — create, list, deploy, and track deployment history
- **SQLite storage** — zero-config database, no external services needed
- **Built with Rust** — Axum + Tokio async runtime

## Quick Start

```bash
git clone https://github.com/AleziuzBuildings/weavefront.com.git
cd weavefront.com/backend

# Set required env vars
export ADMIN_PASSWORD="your-secret-password"

# Set at least one deploy target
export PINATA_JWT="your-pinata-jwt"
# and/or
export ARWEAVE_API_KEY="your-ardrive-turbo-key"
# and/or
export LIGHTHOUSE_API_KEY="your-lighthouse-key"

cargo run
```

The panel is available at `http://localhost:3100`.

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `ADMIN_PASSWORD` | Yes | — | Password for dashboard login |
| `PINATA_JWT` | No | — | Pinata JWT for IPFS pinning |
| `ARWEAVE_API_KEY` | No | — | ArDrive Turbo API key for Arweave uploads |
| `LIGHTHOUSE_API_KEY` | No | — | Lighthouse API key for Filecoin storage |
| `IPFS_GATEWAY` | No | `https://cloudflare-ipfs.com/ipfs/` | IPFS gateway URL |
| `ARWEAVE_GATEWAY_URL` | No | `https://arweave.net` | Arweave gateway URL |
| `FILECOIN_GATEWAY` | No | `https://gateway.lighthouse.storage/ipfs/` | Filecoin/Lighthouse gateway URL |
| `MAX_UPLOAD_MB` | No | `50` | Maximum upload size in MB |
| `WEAVEFRONT_HOST` | No | `0.0.0.0` | Bind address |
| `WEAVEFRONT_PORT` | No | `3100` | Listen port |
| `WEAVEFRONT_DB_PATH` | No | `weavefront.db` | SQLite database path |

## API Reference

All protected endpoints require a `Authorization: Bearer <token>` header (obtained from `/api/auth/login`).

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| `GET` | `/api/health` | No | Health check |
| `GET` | `/api/docs` | No | API documentation (JSON) |
| `POST` | `/api/auth/login` | No | Login, returns bearer token |
| `GET` | `/api/targets` | Yes | List available deploy targets |
| `GET` | `/api/projects` | Yes | List all projects |
| `POST` | `/api/projects` | Yes | Create a new project |
| `GET` | `/api/projects/{id}` | Yes | Get project details |
| `DELETE` | `/api/projects/{id}` | Yes | Delete a project |
| `POST` | `/api/projects/{id}/deploy` | Yes | Deploy archive to project's target |
| `GET` | `/api/projects/{id}/deployments` | Yes | List deployment history |
| `GET` | `/api/settings` | Yes | Get current settings |
| `PUT` | `/api/settings` | Yes | Update settings |

## Deploy Targets

- **IPFS** — Pins content via [Pinata](https://pinata.cloud). Content is addressed by CID and served through configurable IPFS gateways.
- **Arweave** — Permanent storage via [ArDrive Turbo](https://ardrive.io). Data is stored on-chain and accessible through Arweave gateways.
- **Filecoin** — Decentralized storage via [Lighthouse](https://lighthouse.storage). Content is pinned to Filecoin and served through Lighthouse gateways.

## Tech Stack

- [Axum](https://github.com/tokio-rs/axum) — async web framework
- [Tokio](https://tokio.rs) — async runtime
- [SQLite](https://www.sqlite.org) via rusqlite — embedded database
- [reqwest](https://github.com/seanmonstar/reqwest) — HTTP client for upstream APIs

## License

MIT

## Disclaimer

Weavefront is provided as-is. Decentralized storage is permanent by design — content deployed to Arweave or Filecoin cannot be deleted. You are responsible for the content you deploy. The authors are not liable for any damages arising from the use of this software.
