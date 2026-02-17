printf "\njbcom build started...  %s\n" "$(date -u -Iseconds)"
cargo build --release
printf "jbcom build finished.  %s\n" "$(date -u -Iseconds)"

