echo "jbcom installing."
sudo groupadd -g 7878 jbcom
sudo useradd -m -u 7878 -g 7878 jbcom
PY_SITE=$(python3 -c "import site; print(site.getsitepackages()[0])")
sudo mkdir -p ${PY_SITE}
sudo su -c "pwd > ${PY_SITE}/jbcom.pth"
echo Python3 Site: ${PY_SITE}

sudo cp jbcom.service /etc/systemd/system/

sudo systemctl daemon-reload
sudo systemctl start jbcom
sudo systemctl enable jbcom
echo "jbcom installed."
