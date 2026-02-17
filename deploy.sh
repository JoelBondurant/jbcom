printf "jbcom deployment started...  %s\n" "$(date -u -Iseconds)"
ssh me@jbcom "
	mkdir -p /home/me/.config/systemd/user
	mkdir -p /home/me/photos
	mkdir -p /home/me/static
"
scp -r static/ me@jbcom:/home/me/
scp target/release/jbcom me@jbcom:/home/me/jbcom.deploy
scp jbcom.service me@jbcom:/home/me/.config/systemd/user/jbcom.service
ssh me@jbcom "
	systemctl --user daemon-reload
	systemctl --user stop jbcom
	mv -f /home/me/jbcom.deploy /home/me/jbcom
	sudo setcap 'cap_net_bind_service=+ep' /home/me/jbcom
	systemctl --user enable jbcom
	systemctl --user start jbcom
"
printf "jbcom deployment finished.  %s\n" "$(date -u -Iseconds)"
