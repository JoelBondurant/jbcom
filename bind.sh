printf "binding to port 443 started...  %s\n" "$(date -u -Iseconds)"
sudo setcap 'cap_net_bind_service=+ep' target/debug/jbcom
sudo setcap 'cap_net_bind_service=+ep' target/release/jbcom
printf "binding to port 443 finished.  %s\n" "$(date -u -Iseconds)"
