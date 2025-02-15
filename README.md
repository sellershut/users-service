<div align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/sellershut/users-service/check.yaml?label=build">
  
 <a href="https://codecov.io/github/sellershut/users-service" > 
 <img src="https://codecov.io/github/sellershut/users-service/graph/badge.svg?token=5a5zLjjz45"/> 
 </a>
</div>
<h1 align="center">users-service</h1>
<p align="center">
A gRPC server for interacting with platform users
<br />

## Build Dependencies
- `protoc`
- [`sqlx-cli`](https://github.com/launchbadge/sqlx)

## Services
- `postgres` - The main database, included in the development [docker stack](contrib/compose.yaml)

## Usage

- Clone the repository:
```sh
git clone https://github.com/sellershut/users-service.git
cd users-service
```

- Start your docker stack:
```sh
docker compose -f contrib/compose.yaml up -d
```

- Run database migrations:
```sh
cp .env.example .env
cargo sqlx migrate run
```

> [!IMPORTANT]  
> The [config file](users.toml) use the defaults in the [docker stack](contrib/compose.yaml). If you update either, ensure they are both aligned

- Run the application
```sh
cargo run -- --help
```
