# iron Example Server

## Migration

### install diesel_cli

```bash
cargo install diesel_cli
```

### create entity

```bash
diesel migration generate create_department
```

### edit ddl

```bash
vim migrations/...
```

### run migration

```bash
diesel migration run
```
