test day:
    cargo test -p {{day}}

test-all:
    cargo test

create day:
    cargo generate --path ./template --name {{day}}
    just download {{day}}

download day:
    cargo run -p downloader -- --day {{day}}

bench day:
    cargo bench -p {{day}}