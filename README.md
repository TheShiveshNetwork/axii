# Axii - High-Performance LSM-Based Key-Value Store

Axii is a high-performance, persistent key-value database implemented in Rust using a Log-Structured Merge Tree (LSM Tree) architecture. It is designed for high write throughput and scalability.

## Architecture & Folder Structure

The project is organized as a Cargo workspace to ensure separation of concerns, scalability, and code reusability.

```text
axii/
├── common/             # Shared library: custom errors, primitive types (Key, Value), and configs.
├── engine/             # Core library: LSM-Tree logic (Memtable, WAL, SSTables, Compaction).
│   ├── memtable/       # In-memory sorted structures (e.g., SkipList).
│   ├── wal/            # Write-Ahead Log for crash recovery and durability.
│   ├── sstable/        # Sorted String Table formats, indexing, and block management.
│   └── compaction/     # Background merge-sort logic for storage optimization.
├── server/             # Network binary: Handles TCP/gRPC client connections and queries.
├── cli/                # Terminal binary: Interactive REPL for database interaction.
└── tests/              # Integration tests covering cross-crate functionality.
```

## Key Components

- **Memtable**: Fast in-memory writes using sorted structures.
- **WAL (Write-Ahead Log)**: Ensures data durability before flushing to disk.
- **SSTables**: Immutable on-disk files for persistent storage with sparse indexing.
- **Compaction**: Background process that merges SSTables to reclaim space and optimize reads.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.75+ recommended)

### Build the Project

To build all crates in the workspace:

```bash
cargo build
```

### Running the Components

**Start the Server:**
```bash
cargo run -p server
```

**Launch the CLI:**
```bash
cargo run -p cli
```

### Running Tests

Run all unit and integration tests:

```bash
cargo test
```

## Implementation Status

- [x] Workspace & Crate Structure
- [x] Common Type Definitions
- [ ] In-Memory Engine (Memtable)
- [ ] Write-Ahead Log (WAL)
- [ ] SSTables & Indexing
- [ ] Compaction Worker
- [ ] Network Server & CLI REPL
