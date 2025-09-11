printf "jbcom deployment started...  %s\n" "$(date -u -Iseconds)"
scp target/release/jbcom root@zero:/home/jbcom/jbcom.deploy
ssh root@zero systemctl status jbcom
ssh root@zero systemctl stop jbcom
ssh root@zero systemctl status jbcom
ssh root@zero mv -f /home/jbcom/jbcom.deploy /home/jbcom/jbcom
ssh root@zero setcap 'cap_net_bind_service=+ep' /home/jbcom/jbcom
ssh root@zero systemctl start jbcom
ssh root@zero systemctl status jbcom
printf "jbcom deployment finished.  %s\n" "$(date -u -Iseconds)"

