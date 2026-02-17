printf "jbcom deployment started...  %s\n" "$(date -u -Iseconds)"
ssh me@jbcom "
	sudo mkdir -p /opt/jbcom/photos
	sudo mkdir -p /opt/jbcom/static
	sudo chown -R me:me /opt/jbcom
"
scp -r static/ me@jbcom:/home/me/
scp jbcom.service me@jbcom:/home/me/jbcom.service
scp target/release/jbcom me@jbcom:/home/me/jbcom
ssh me@jbcom "
	sudo mv -f /home/me/jbcom.service /etc/systemd/system/jbcom.service
	sudo systemctl daemon-reload
	sudo systemctl stop jbcom
	mv -f /home/me/jbcom /opt/jbcom/jbcom
	rm -rf /opt/jbcom/static
	mv -f /home/me/static /opt/jbcom/
	sudo systemctl enable jbcom
	sudo systemctl start jbcom
"
printf "jbcom deployment finished.  %s\n" "$(date -u -Iseconds)"
