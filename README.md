# opensase-hr

Self-hosted HR system built with Rust.

## Quick Start

```bash
docker-compose up -d
curl http://localhost:8082/health
```

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL URL | Required |
| `PORT` | Service port | `8082` |
| `NATS_URL` | NATS server | Optional |

## API Endpoints

- `GET /health` - Health check
- `GET /api/v1/*` - API endpoints

## Development

```bash
cp .env.example .env
cargo run
```

## License

MIT OR Apache-2.0
