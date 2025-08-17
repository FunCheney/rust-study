
### csv 数据转化为 rust 的数据结构
添加依赖
```6502 assembly
cargo add clap --features derive
```

```6502 assembly
cargo run
```

```6502 assembly
cargo run -- csv
```

```6502 assembly
cargo run -- csv -i data/test.csv
```
通过 csv 的 create 来处理csv
```6502 assembly
cargo add csv 
```

```6502 assembly
cargo add serde --features derive 
```

