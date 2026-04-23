# Axii - High-Performance LSM-Based Key-Value Store

Axii is a persistent key-value database implemented in Rust using a [Log-Structured Merge Tree (LSM Tree)](https://en.wikipedia.org/wiki/Log-structured_merge-tree) architecture. It is designed for high write throughput and durability.

## Architecture & Folder Structure

The project is organized as a Cargo workspace to ensure separation of concerns and code reusability.

```text
axii/
├── common/             # Shared library: custom errors, protocol types, and binary serialization.
├── engine/             # Core library: LSM-Tree logic (Memtable, WAL, SSTables).
│   ├── memtable/       # In-memory sorted structure (BTreeMap).
│   ├── wal/            # Write-Ahead Log for crash recovery and durability.
│   └── sstable/        # Sorted String Table formats for persistent storage.
├── server/             # Network library: Handles multi-threaded TCP client connections.
├── cli/                # Terminal binary: Command-line interface for the database.
└── GEMINI.md           # Technical specification and roadmap.
```

## Key Components

- **[Memtable](https://github.com/facebook/rocksdb/wiki/MemTable)**: Fast in-memory writes using a thread-safe `BTreeMap`.
- **[WAL (Write-Ahead Log)](https://en.wikipedia.org/wiki/Write-ahead_logging)**: Ensures data durability by logging operations before applying them to the Memtable.
- **[SSTables](https://www.igvita.com/2012/02/06/sstable-and-log-structured-storage-leveldb/)**: Immutable on-disk files for persistent storage, flushed when the Memtable exceeds size thresholds.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Build the Project

To build all crates in the workspace:

```bash
cargo build
```

### Running the Components

All operations are managed through the unified `cli` tool.

#### 1. Start the Server
The server stores data in the specified directory (defaults to `./data`).

```bash
cargo run --bin cli -- start-server --data ./data
```

#### 2. Perform Database Operations
In a separate terminal, use the CLI to interact with the server:

**Set a key-value pair:**
```bash
cargo run --bin cli -- set mykey "hello world"
```

**Get a value by key:**
```bash
cargo run --bin cli -- get mykey
```

**Delete a key:**
```bash
cargo run --bin cli -- delete mykey
```

## Protocol
Axii uses a custom binary protocol over TCP. Requests and responses are serialized using `bincode` for maximum efficiency.

## Implementation Status

- [x] Workspace & Crate Structure
- [x] Common Type Definitions & Binary Protocol
- [x] In-Memory Engine (Memtable using BTreeMap)
- [x] Write-Ahead Log (WAL) with Recovery
- [x] SSTable Flushing & Persistent Lookups
- [x] Multi-threaded TCP Server
- [x] Unified CLI Client
- [ ] Compaction Worker (Background storage optimization)
