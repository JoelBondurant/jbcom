printf "jbcom deployment started...  %s\n" "$(date -u -Iseconds)"
ssh me@jbcom "
	mkdir -p /home/me/photos
	mkdir -p /home/me/static
"
scp -r static/ me@jbcom:/home/me/
scp target/release/jbcom me@jbcom:/home/me/jbcom.deploy
scp jbcom.service me@jbcom:/home/me/jbcom.service
ssh me@jbcom "
	sudo mv -f /home/me/jbcom.service /etc/systemd/system/jbcom.service
	sudo systemctl daemon-reload
	sudo systemctl stop jbcom
	mv -f /home/me/jbcom.deploy /home/me/jbcom
	sudo systemctl enable jbcom
	sudo systemctl start jbcom
"
printf "jbcom deployment finished.  %s\n" "$(date -u -Iseconds)"
