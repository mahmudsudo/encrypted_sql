# encrypted_sql

# Roadmap
- [ ] Setup database: sqlite db.
- [ ] Make database accept the query as an input.
- [ ] Implement the CUP Integer API methods, using TFHE-rs for encryption and decryption.
- [ ] Implement loading the database from a directory with the specified structure (tables in CSV file).
- [ ] Implement SQL select, select distinct, where, and, or, not, in, and between operations for encrypted queries
- [ ] Manage operations for integers (<, <=, >, >=, =) and for strings (=).
- [ ] Manage operations for integers (+, -, *, /, %) and for strings (+).
- [ ] Write unit tests for the implementations
- [ ] Test with sample databases and SQL queries
- [ ] Use AWS hardware for benchmarking: (CPU: hpc7a.96xlarge, GPU: p3.2xlarge).
- [ ] Ensure your implementation can run with both multi bit and non-multi bit parameter sets
- [ ] Prepare code for submission.