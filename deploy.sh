#!/bin/bash
MODE=${1:-release}
if [ "$MODE" = "dev" ]; then
    BUILD_FLAG=""
    TARGET_DIR="debug"
else
    BUILD_FLAG="--release"
    TARGET_DIR="release"
fi
printf "jbcom %s deployment started...  %s\n" "$MODE" "$(date -u -Iseconds)"
cargo build $BUILD_FLAG
ssh me@jbcom "
	sudo mkdir -p /opt/jbcom/photos
	sudo mkdir -p /opt/jbcom/static
	sudo chown -R me:me /opt/jbcom
"
scp -r static/ me@jbcom:/home/me/
scp jbcom.service me@jbcom:/home/me/jbcom.service
scp target/$TARGET_DIR/jbcom me@jbcom:/home/me/jbcom
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
