printf "jbcom build started...  %s\n" "$(date -u -Iseconds)"
RUSTFLAGS="-C opt-level=z" cargo build --release
./bind.sh
printf "jbcom build finished.  %s\n" "$(date -u -Iseconds)"

