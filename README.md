# iron Example Server

## Migration

### setup url

```bash
echo DATABASE_URL=mysql://root@0.0.0.0/iron > .env
```

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
