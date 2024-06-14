# Payment Service with API

## Description

This is a simple payment service with API. It is a RESTful API that allows you to create, read, update and delete payments. The service is written in Rust

## Installation 

To install the service, you need to have Rust installed on your machine. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

After installing Rust, you can clone the repository and run the service with the following commands:

```bash
git clone
cd payment-service
cargo run
```

Use environment variables to configure the service. The following environment variables are available:

- `DATABASE_URL`: The URL of the database.
- `HOST`: The host of the service.
- `PORT`: The port of the service.

### Docker

You can also run the service using Docker. To do this, you need to have Docker installed on your machine. You can install Docker by following the instructions on the [official website](https://docs.docker.com/get-docker/).

After installing Docker, you can run the service with the following commands:

```bash
git clone
cd payment-service
docker build -t payment-service .
docker run -p 8080:8080 payment-service
```

