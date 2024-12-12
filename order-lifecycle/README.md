# Order Lifecycle Application

The **Order Lifecycle Application** is a microservice designed to manage the lifecycle of customer orders. Built using Rust, PostgreSQL, and SQLx, it handles order creation, updates, and tracking, offering a robust and efficient backend for e-commerce and order processing systems.

---

## Features

- **Order Management**:

  - Create, update, and retrieve orders.
  - Track order statuses (e.g., pending, processing, completed).

- **Database Integration**:

  - Uses PostgreSQL for persistent storage.
  - Efficient query handling with SQLx and connection pooling.

- **Dockerized Deployment**:
  - Easily deployable with Docker and Docker Compose.

---

## Prerequisites

- **Rust**: Ensure Rust is installed. ([Rust Installation Guide](https://www.rust-lang.org/tools/install))
- **PostgreSQL**: Version 13+.
- **Docker**: For containerized deployment.

---

## Setup and Installation

**Docker**

This command will run both `postgres` and `order-lifecycle` images.

```
docker compose up
```

**Native**

You can use a local database or run postgres from the docker file:

```
docker compose up postgres
```

and then:

```
cargo sqlx prepare
cargo run
```

to run the order-lifecycle application.

---

## Folder Structure

```
.
├── Cargo.toml
├── Dockerfile
├── README.md
├── docker-compose.yaml
├── migrations
│   └── 20231211100000_create_orders_tables.sql
├── project.json
└── src
    ├── db
    │   ├── mod.rs
    │   ├── orders.rs
    │   └── tasks.rs
    ├── fake
    │   └── mod.rs
    ├── main.rs
    ├── models
    │   ├── country.rs
    │   ├── mod.rs
    │   ├── order.rs
    │   ├── order_error.rs
    │   ├── order_payload.rs
    │   ├── product.rs
    │   ├── task.rs
    │   ├── task_error.rs
    │   └── task_payload.rs
    ├── orders
    │   ├── mod.rs
    │   └── uk
    │       ├── car
    │       │   ├── activate.rs
    │       │   ├── audit.rs
    │       │   ├── cancel.rs
    │       │   ├── issue.rs
    │       │   ├── mod.rs
    │       │   ├── suspend.rs
    │       │   ├── terminate.rs
    │       │   └── update.rs
    │       └── mod.rs
    ├── routes
    │   ├── faker.rs
    │   ├── mod.rs
    │   ├── orders.rs
    │   └── tasks.rs
    └── worker
        ├── mod.rs
        ├── process.rs
        └── worker.rs
```
