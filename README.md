# feedbin-reader

A terminal reader for feedbin

Stores config in ~/.config/feedbin-reader

Caches data in a sqlite database in ~/.cache/feedbin-reader (planned)

## Development

Running tests:

```
make
```

Installing the diesel CLI:

```
cargo install diesel_cli --no-default-features --features sqlite
```

(I had some compiliation errors on my laptop otherwise)
