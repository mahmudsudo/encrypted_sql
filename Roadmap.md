### 1. Setup and Understanding
- **Understand TFHE-rs**: Familiarize yourself with the TFHE-rs library, specifically the CPU integer API features.
- **Project Setup**: Create a new Rust crate for your project, ensuring it's independent and structured properly.
- **Dependency Management**: Ensure your crate depends on tfhe-rs (either version 0.5.x or the main branch). Add other dependencies such as `sqlparser` for parsing SQL queries.

### 2. Database Setup
- **Database Format**: Understand the CSV-based database format as outlined, with the specific data types (integers, booleans, short strings).
- **Loading the Database**: Implement a function to load the database from the directory structure provided (`table_1.csv`, `table_2.csv`, etc.) into a suitable data structure (`Tables`).

### 3. Query Handling
- **Encrypting the Query**:
    - Use `sqlparser` to parse a plaintext SQL query file (`query.txt`).
    - Implement the `encrypt_query` function to convert the parsed SQL query into an `EncryptedQuery` using the TFHE encryption scheme.

### 4. FHE Operations
- **Executing the Query**:
    - Implement the `run_fhe_query` function:
        - Use the `ServerKey` to process the `EncryptedQuery` on the data stored in `Tables`.
        - Return an `EncryptedResult` which holds the results still in encrypted form.

### 5. Decryption and Output
- **Decrypting the Result**:
    - Implement the `decrypt_result` function to decrypt the `EncryptedResult` using the client's private key (`ClientKey`) to get the actual query result in clear text format.
- **Output Format**:
    - Ensure the executable provides outputs as specified, showing runtime, clear DB query result, encrypted DB query result, and whether the results match.

### 6. Testing and Validation
- **Local Testing**:
    - Thoroughly test the system with various SQL queries to ensure it behaves as expected. Pay special attention to the operators and SQL functionalities required by the bounty.
- **Clippy and Checks**:
    - Run `cargo clippy -- --no-deps -D warnings` to ensure your code is lint-free and adheres to Rust's style and quality guidelines.

### 7. Submission
- **Documentation**:
    - Document your code thoroughly, explaining how each component works and the choices you made, particularly around the encryption and query processing.
- **Submission**:
    - Submit your code as specified before the deadline, ensuring all required components are included and functioning as described.

### 8. Benchmark Preparation
- **Performance Optimization**:
    - Optimize your implementation for performance, considering it will be benchmarked on high-performance hardware. This might involve profiling and tuning your code.
