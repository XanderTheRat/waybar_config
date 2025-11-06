#!/usr/bin/env bash
modefile="/tmp/waybar_net_mode"
if [ ! -f "$modefile" ]; then
  echo ssid > "$modefile"
fi
mode=$(cat "$modefile")

if [ "$1" = "next" ]; then
  case "$mode" in
    ssid) newmode="ipv4" ;;
    ipv4) newmode="ipv6" ;;
    ipv6) newmode="ssid" ;;
    *) newmode="ssid" ;;
  esac
  echo "$newmode" > "$modefile"
  exit 0
fi

case "$mode" in
  ssid)
    ssid=$(nmcli -t -f DEVICE,TYPE,STATE,CONNECTION dev | grep ':wifi:connected:' | cut -d: -f4)
    if [ -z "$ssid" ]; then
      echo "<span foreground='#f6ad55'>⚠ Disconnected</span>"
    else
      echo "<span foreground='#63b3ed'> $ssid</span>"
    fi
    ;;
  ipv4)
    ipv4=$(ip -4 addr show | grep 'inet ' | awk '{print $2}' | grep -v '127.0.0.1' | head -n 1)
    if [ -z "$ipv4" ]; then
      echo "<span foreground='#f6ad55'>❌ No IPv4</span>"
    else
      echo "<span foreground='#a78bfa'>ipv4:</span><span foreground='#63b3ed'> $ipv4</span>"
    fi
    ;;
  ipv6)
    ipv6=$(ip -6 a show up | grep 'inet6' | awk '{print $2}' | tail -n 1)
    if [ -z "$ipv6" ]; then
      echo "<span foreground='#f6ad55'>❌ No IPv6</span>"
    else
      echo "<span foreground='#fb923c'>ipv6:</span><span foreground='#63b3ed'> $ipv6</span>"
    fi
    ;;
esac
