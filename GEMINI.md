# Technical Specification: LSM-Based Key-Value Store

This document outlines the architecture and implementation roadmap for a high-performance, in-memory key-value database using a Log-Structured Merge Tree (LSM Tree) design.

## 1. System Architecture

The database consists of four primary components:
1.  **Memtable**: An in-memory sorted data structure (e.g., Skip List or Balanced BST).
2.  **Write-Ahead Log (WAL)**: An append-only file used for durability.
3.  **SSTables (Sorted String Tables)**: Immutable on-disk files containing sorted key-value pairs.
4.  **Index**: A sparse or full index mapping keys to offsets within SSTables.

## 2. Core Operations

### Write Operation (Add / Update / Delete)
1.  Append the operation to the **Write-Ahead Log (WAL)** for crash recovery.
2.  Insert the key-value pair into the **Memtable**.
    * **Updates**: Handled as new insertions; the most recent entry in the Memtable takes precedence.
    * **Deletes**: Handled by inserting a "tombstone" marker (a null value) for the key.

### Read Operation (Search)
1.  Search the **Memtable**. If the key is found, return the value (or null if it is a tombstone).
2.  If not found in memory, consult the **SSTable Index**.
3.  Use the index offset to read the value directly from the **On-Disk Database**.
4.  If multiple SSTables exist, search them from newest to oldest.

## 3. Data Lifecycle

### Flushing
When the Memtable reaches a pre-defined size threshold:
1.  The Memtable is converted into a sorted list of records.
2.  The records are written to a new **SSTable** file on disk.
3.  A corresponding index is generated for this SSTable.
4.  The WAL is cleared, and the Memtable is reset.

### Compaction
Since updates and deletes create duplicate entries across different SSTables, a background compaction process is required:
1.  Read multiple SSTables.
2.  Merge the sorted records.
3.  If duplicate keys exist, keep only the record with the most recent timestamp.
4.  Discard tombstoned records if they are the oldest version.
5.  Write the merged results into a single, new SSTable and delete the old files.

## 4. Implementation Phases

### Phase 1: In-Memory Engine
* Implement a sorted data structure (Red-Black Tree or AVL Tree).
* Define the basic `Get`, `Set`, and `Delete` API.

### Phase 2: Durability
* Implement the Write-Ahead Log (WAL) system.
* Implement a recovery mechanism to rebuild the Memtable from the WAL on startup.

### Phase 3: SSTables and Indexing
* Implement the flushing mechanism (Memtable to Disk).
* Create an index structure to store file offsets for disk-based lookups.
* Ensure SSTables are immutable once written.

### Phase 4: Compaction
* Implement the Merge Sort algorithm for combining multiple SSTables.
* Logic for reclaiming space by removing superseded values and tombstones.

## 5. Performance Characteristics
* **Writes**: O(log N) for Memtable insertion + O(1) for sequential WAL append.
* **Reads**: O(log N) for Memtable + O(log M) for Index lookup + 1 Disk I/O.
* **Space Efficiency**: High for writes, requires compaction to maintain read efficiency and disk space.
