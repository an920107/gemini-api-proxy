# Research: Project Skeleton & Database Connectivity

## Overview

This document summarizes the technology decisions for the initial project skeleton. All choices are based on the project constitution and the user-provided implementation plan.

## Technology Choices

### Web Framework

-   **Decision**: `actix-web`
-   **Rationale**: A powerful, pragmatic, and extremely fast web framework for Rust. It's a popular choice with good community support.
-   **Alternatives considered**: `axum`, `rocket`. `axum` is also a strong contender, but `actix-web` is more established. `rocket` is less flexible for our needs.

### Database Access

-   **Decision**: `sqlx`
-   **Rationale**: Provides compile-time checked queries against the database, which is a major advantage for catching errors early. It's asynchronous and integrates well with `actix-web`.
-   **Alternatives considered**: `diesel`. `diesel` is a powerful ORM, but `sqlx` offers more direct control over SQL and compile-time safety, which is preferred for this project.

### Asynchronous Runtime

-   **Decision**: `tokio`
-   **Rationale**: The de-facto standard for asynchronous programming in Rust. It's required by `actix-web` and `sqlx`.
-   **Alternatives considered**: `async-std`. `tokio` has a larger ecosystem and is more commonly used.
