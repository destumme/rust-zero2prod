# Zero2Prod Book
Just working through the [Zero2Prod](https://www.zero2prod.com/index.html?country_code=US) rust book.

Code comments are mostly my notes. Won't be a 1-1 comparison, as I play around with things.

## Setup

Requires rust, direnv, docker, sqlx

Zero 2 Prod book has 1 big setup script. Made it in compose instead, db user is created in entrypoint script.

```
$ mkdir data
$ docker compose up -d
$ ./scripts/migrate.sh
```

Remove data dir and delete compose stack and vols to re run .init.sh on container.

