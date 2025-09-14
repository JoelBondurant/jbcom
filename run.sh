printf "jbcom started...  %s\n" "$(date -u -Iseconds)"
for i in {1..3}; do ./bind.sh && break || sleep 2; done
#cargo watch -s "./bind.sh" -x run
sleep 2
./target/release/jbcom
