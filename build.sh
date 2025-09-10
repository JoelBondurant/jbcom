cargo build
scp target/debug/jbcom root@zero:/home/jbcom/jbcom.deploy
ssh root@zero systemctl stop jbcom
ssh root@zero mv /home/jbcom/jbcom.deploy /home/jbcom/jbcom
ssh root@zero setcap 'cap_net_bind_service=+ep' /home/jbcom/jbcom
ssh root@zero systemctl status jbcom

