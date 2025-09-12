clear
printf "\njbcom build started...  %s\n" "$(date -u -Iseconds)"
cargo fmt
RUSTFLAGS="-C opt-level=z" cargo build --release
./bind.sh
printf "jbcom build finished.  %s\n" "$(date -u -Iseconds)"

