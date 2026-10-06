# Rust Kubernetes Operator

A production-grade, asynchronous Kubernetes operator built from scratch in **Rust** using the `kube-rs` ecosystem and the `tokio` runtime. This controller manages custom resources efficiently, demonstrating robust systems-level design, deterministic state reconciliation, and clean resource lifecycle management.

## Architecture & Design Patterns

* **Level-Triggered Reconciliation:** Implements an asynchronous control loop that reacts to cluster events and continuously converges the actual state of the system toward the desired custom specification.
* **Asynchronous Runtime:** Powered by `tokio` for high-throughput, non-blocking concurrent event handling and network I/O.
* **Robust Lifecycle Management:** Utilizes Kubernetes **finalizers** to handle clean teardown operations, ensuring zero resource leaking upon deletion of custom resources.
* **Status Subresources:** Continuously updates custom resource status conditions to provide clear observability into the health and reconciliation progress of managed components.

## Tech Stack

* **Language:** Rust (Edition 2024 / C++26 toolchain synergy)
* **Kubernetes Client & Controller Framework:** `kube-rs` (`kube`, `kube-runtime`, `kube-core`)
* **Async Runtime:** `tokio`
* **Serialization/Deserialization:** `serde` & `serde_json`
* **Local Testing Environment:** `kind` (Kubernetes in Docker)

## Project Structure

```text
.
├── Cargo.toml          # Dependencies and workspace config
├── src/
│   ├── main.rs         # Entry point, telemetry setup, and controller initialization
│   ├── crd.rs          # Custom Resource Definition (CRD) struct definitions
│   └── controller.rs   # Core reconciliation logic, error handling, and finalizer management
└── manifests/          # Sample CRD YAML definitions and RBAC configurations
