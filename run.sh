printf "jbcom started...  %s\n" "$(date -u -Iseconds)"
./bind.sh
sleep 2
#cargo watch -s "./bind.sh" -x run
cargo run --release
