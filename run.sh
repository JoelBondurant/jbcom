printf "jbcom started...  %s\n" "$(date -u -Iseconds)"
#cargo watch -s "./bind.sh" -x run
./bind.sh
cargo run --release
