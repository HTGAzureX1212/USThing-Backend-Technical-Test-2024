# USThing Backend Technical Test 2024

This is a backend project written for the USThing Backend Technical Test 2024, using the Rust programming language
with the `axum` web framework.

## Running with Docker (Requires Docker Compose)

1. Create a copy of the `.env.example` file and write corresponding values to the environment variables.
2. Run, from the `docker` directory:
```shell
docker compose --env-file ../.env up --build server
```
