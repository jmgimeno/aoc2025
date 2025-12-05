test day:
    cargo test -p {{day}}

test-all:
    cargo test

create day:
    cargo generate --path ./template --name {{day}}
