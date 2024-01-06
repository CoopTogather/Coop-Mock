# Coop Mock - A Cooperation friendly mock server

## Database

### Migration

This project use Sea-ORM to access Database.

Please make sure you have PostgreSQL ConnectionString before you execute migrate command.

You can put connection string on `.env` file like:
```
DATABASE_URL={your_connection_string}
```

To run migration command. you need to install Sea-ORM Command Line first:
```bash
cargo install sea-orm-cli
```

Then, Execute migrate command:
```bash
sea-orm-cli migrate
```

### Generate database entity

To generate entities of Database, You need to execute this command:
```bash
sea-orm-cli generate entity -o ./coop_service/src/infrastructure/models
```

`-o` means `output-dir`