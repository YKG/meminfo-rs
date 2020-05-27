# Test sysinfo memory used caclulation

A tool to help me check the result against `free` command to verify `sysinfo` memory used caclulation works fine.
The program will keep printing meminfo every second. 

#### Usage


Open two termnials

- Terminal 1

```bash
watch -n 1 free
```

- Terminal 2

```bash
git clone https://github.com/YKG/meminfo-rs.git
cd meminfo-rs
cargo run
```
