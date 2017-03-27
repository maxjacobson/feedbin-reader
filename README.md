# feedbin-reader

A terminal reader for [feedbin](http://feedbin.com/)

Stores config in `~/.config/feedbin-reader`

Caches data in a sqlite database in `~/.cache/feedbin-reader`

## Development

Running the thing:

```
cargo run
```

Running tests:

```
make
```

Installing the diesel CLI (useful for generating new migrations, probably other stuff):

```
cargo install diesel_cli --no-default-features --features sqlite
```

(I had some compiliation errors on my laptop without limiting the features like this)
