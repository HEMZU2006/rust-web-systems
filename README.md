# rust-web-systems

A hands-on learning repository for exploring web systems and system architecture through progressive, self-contained builds and experiments.

---

## Course-Wide Requirements & Setup

Apply these baseline standards across all four assignments. Using a single recommended stack keeps debugging and grading consistent.

- **Recommended Rust Stack:**
  - `axum` — Web framework
  - `tokio` — Async runtime
  - `sqlx` — Compile-time checked PostgreSQL driver and migrations
  - `serde` — JSON serialization/deserialization
  - `argon2` — Password hashing
- **Database Persistence:** Every assignment using PostgreSQL must include a `docker-compose.yml` file with a **named Docker volume** (e.g., `pgdata:/var/lib/postgresql/data`) so data survives `docker compose down` and `docker compose up`.
- **GitHub Workflow:** Maintain a single monorepo (e.g., `rust-backend-labs`) with separate folders (`lab-01`, `lab-02`, etc.) or sequential feature branches/PRs. Every repository must include:
  - `.gitignore` (excluding `/target` and `.env`)
  - `.env.example`
  - `README.md` with exact commands to run and test the server

---

## Assignments

### Assignment 1: Hello World API

> **Objective:** Set up a Rust web server, handle HTTP routing and JSON serialization, and establish the GitHub repository workflow.

#### Requirements

1. Initialize a new Cargo project and configure an async HTTP server listening on a configurable port (default `3000`, read from an environment variable).
2. Implement structured logging (e.g., using `tracing` and `tracing-subscriber`) to log incoming requests.
3. Implement the following endpoints:

| Method | Path | Response |
| --- | --- | --- |
| `GET` | `/health` | `200 OK` — `{"status": "ok"}` |
| `GET` | `/hello` | `200 OK` — `{"message": "Hello, World!"}` |
| `GET` | `/hello/{name}` | `200 OK` — `{"message": "Hello, {name}!"}` |

#### Deliverables & Verification

- GitHub repository initialized with a clean commit history and a `README.md` containing `curl` examples.
- Running `cargo run` starts the server cleanly with zero compiler warnings.
- Requesting an undefined route returns `404 Not Found`.

---

### Assignment 2: User Login & Logout API (PostgreSQL + Docker)

> **Objective:** Connect a Rust API to a persistent PostgreSQL database, store passwords securely, and manage user sessions.

#### Requirements

1. **Dockerized Postgres:** Create a `docker-compose.yml` running `postgres:16-alpine` with a named volume for persistence.
2. **Schema & Migrations:** Use SQL migrations (e.g., `sqlx migrate`) to create:
   - `users` table (`id`, `username` UNIQUE, `password_hash`, `created_at`)
   - `sessions` table (`token` PRIMARY KEY, `user_id` REFERENCES `users(id)`, `expires_at`, `created_at`) — or signed JWTs with a server-side revocation table so logout invalidates tokens.
3. **Security:** Hash passwords using `argon2` with a unique salt per user. Never store or log plaintext passwords.
4. **Endpoints:**

| Method | Path | Behavior |
| --- | --- | --- |
| `POST` | `/register` | Accepts `{"username", "password"}`. Returns `201 Created` (or `409 Conflict` if the username exists). |
| `POST` | `/login` | Verifies credentials, creates a session record, and returns a session token (via HTTP-only cookie or `Authorization: Bearer` token). Returns `401 Unauthorized` on failure. |
| `POST` | `/logout` | Deletes or revokes the active session in PostgreSQL. Subsequent requests with that token must fail with `401 Unauthorized`. |
| `GET` | `/me` | Protected route that returns the authenticated user's profile. |

#### Deliverables & Verification

- **Persistence Test:** Register a user, run `docker compose down && docker compose up -d`, and verify the user can still log in.
- **Logout Test:** Confirm that calling `GET /me` with a logged-out token returns `401 Unauthorized`.
- **Hash Verification:** Inspecting the `users` table via `psql` confirms passwords are stored as Argon2 hashes.

---

### Assignment 3: Persistent Counter API

> **Objective:** Perform stateful read/write operations against PostgreSQL, integrate database connection pooling, and handle HTTP error mapping cleanly.

#### Requirements

1. **Database Schema:** Add a `counters` table (`id` or `name` PRIMARY KEY, `value` `BIGINT NOT NULL DEFAULT 0`, `updated_at` `TIMESTAMPTZ`) via migrations. Seed a default counter or allow creating named counters.
2. **Connection Pooling:** Share a `PgPool` across route handlers using framework state (e.g., Axum's `State` extractor).
3. **Endpoints:** *(Optional: protect these routes using the auth middleware built in Assignment 2.)*

| Method | Path | Behavior |
| --- | --- | --- |
| `GET` | `/counter` | Returns the current counter value from PostgreSQL (e.g., `{"value": 42}`). |
| `POST` | `/counter/increment` | Increments the counter by `1` (or an optional JSON body `{"amount": 5}`) and returns the updated value. |
| `POST` | `/counter/decrement` | Decrements the counter by `1` (or an optional `amount`) and returns the updated value. |
| `POST` | `/counter/reset` | Resets the counter to `0`. |

#### Deliverables & Verification

- **Persistence Test:** Increment the counter 5 times, restart both the Rust binary and the Postgres Docker container, and verify `GET /counter` still returns `5`.
- **Error Handling:** Database connection failures or invalid payloads return proper HTTP status codes (`400 Bad Request`, `500 Internal Server Error`) rather than panicking the server.

---

### Assignment 4: Multi-Client Concurrent Counter

> **Objective:** Understand database race conditions (specifically the "lost update" anomaly) under concurrent load and solve them using PostgreSQL concurrency primitives.

#### Requirements

1. **Build a Load/Concurrency Test Client:** Write a separate Rust CLI binary (using `tokio` and `reqwest`) or an integration test inside the repo that spawns at least **50 concurrent tasks**, each sending **20 increment** and **10 decrement** requests simultaneously (1,500 total requests; expected net change = `+500`).
2. **Part A — Reproduce the Bug *(Optional, Recommended)*:** Briefly document what happens when the API uses a naive read-modify-write pattern in Rust (`SELECT value` → add 1 in Rust → `UPDATE SET value = new_value`). Record the expected vs. actual final counter value in `README.md`.
3. **Part B — Fix for Concurrency:** Update the API so concurrent requests never lose updates, using one of these PostgreSQL patterns:
   - **Atomic SQL Update:**
     ```sql
     UPDATE counters SET value = value + $1 WHERE id = $2 RETURNING value;
     ```
   - **Pessimistic Row Locking:** A transaction (`BEGIN ... COMMIT`) using `SELECT value FROM counters WHERE id = $1 FOR UPDATE` before updating.
4. **Connection Pool Tuning:** Configure `sqlx::postgres::PgPoolOptions` (e.g., `max_connections` and `acquire_timeout`) so the server handles burst traffic gracefully without dropping requests.

#### Deliverables & Verification

- The concurrency test client is committed to the repo and runnable via a single command (e.g., `cargo test --test concurrency` or `cargo run --bin load_test`).
- Running the test from a counter value of `0` consistently yields an exact final value of `500` across multiple runs with zero failed HTTP requests.
- A short section in `README.md` explaining *why* the naive approach fails and *how* the PostgreSQL query prevents the race condition.

---

## Suggested Grading Rubric

| Assignment | Core Functionality (50%) | Database & Docker Persistence (25%) | Code Quality, Error Handling & Git (25%) |
| --- | --- | --- | --- |
| **1. Hello World** | All 3 routes return correct JSON & status codes | N/A (substitute: env var config & logging) | Clean Cargo setup, `.gitignore`, clear `README.md` |
| **2. Auth API** | Register, login, logout, and `/me` work end-to-end | Named Docker volume persists users; Argon2 hashing verified | Proper `401`/`409` codes, no hardcoded secrets |
| **3. Counter API** | Get, increment, decrement, and reset work via `PgPool` | Value survives full Docker + server restart; SQL migrations included | No `.unwrap()` on request/DB paths; clean error mapping |
| **4. Multi-Client** | 100% accurate final count under concurrent load | Uses atomic SQL or `FOR UPDATE` transactions; pool tuned | Reproducible Rust test client + race-condition write-up |
