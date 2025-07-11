# ydotoo

[ydotool](https://github.com/ReimuNotMoe/ydotool) - automation too, works on wayland

Build from site repo nodes, after make,

[Setup tutorial](https://gabrielstaples.com/ydotool-tutorial/#gsc.tab=0)

  1. Install the service file:
  sudo cp ydotoold.service /etc/systemd/system/

  2. Copy the daemon binary:
  sudo cp ydotoold /usr/local/bin/

  3. Enable and start the service:
  sudo systemctl daemon-reload
  sudo systemctl enable ydotoold
  sudo systemctl start ydotoold

  4. Check service status:
  sudo systemctl status ydotoold


  #environment variable
  export
  YDOTOOL_SOCKET=/tmp/.ydotool_socket
  ydotool key 56:1 62:1 62:0 56:0

  Option 3: Add your user to the input 
  group
  sudo usermod -a -G input slyedoc
  sudo chmod 666 /dev/uinput

  Then restart the daemon to run as
  your user instead of root.
