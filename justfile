test day:
    cargo test -p {{day}}

test-all:
    cargo test

new day:
    just create {{day}}
    just download {{day}}

create day:
    cargo generate --path ./template --name {{day}}

download day:
    cargo run -p downloader -- --day {{day}}

bench day:
    cargo bench -p {{day}}