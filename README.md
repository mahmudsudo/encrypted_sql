# encrypted_sql

# Roadmap
- [x] Setup database: sqlite db.
- [x] Make database accept the query as an input.
- [x] Implement loading the database from a directory with the specified structure (tables in CSV file).
- [x] Clean up Placeholder values and implement the rest of the API methods correctly.
- [x] Implement the CPU Integer API methods, using TFHE-rs for encryption and decryption.
- [x] Implement SQL select, select distinct, where, and, or, not, in, and between operations for encrypted queries
- [x] Manage operations for integers (<, <=, >, >=, =) and for strings (=).
- [x] Manage operations for integers (+, -, *, /, %) and for strings (+).
- [ ] Write unit tests for the implementations
- [ ] Test with sample databases and SQL queries
- [ ] Use AWS hardware for benchmarking: (CPU: hpc7a.96xlarge, GPU: p3.2xlarge).
- [ ] Ensure your implementation can run with both multi bit and non-multi bit parameter sets
- [ ] Prepare code for submission.


```rust
// Decrypted data example.
fn main() {
    let decrypted_value: u8 = encrypted_value.decrypt(&client_key).expect("Decryption failed");
}
```