x-z:
  clear && cargo run --bin x --release

x:
  clear && cargo run --bin x

use:
  cargo run --bin use

live:
  cargo run --bin live

proxy:
  mitmweb --mode local:vgc.exe --script mitm.py

devices:
  ./devices.bat

connect:
  echo "Couldn't connect."

update:
  rustup update
  cargo update

test:
  cd "/c/Program Files/mitmproxy/bin/_internal/mitmproxy_windows/"
  ./windows-redirector.exe

build:
  powershell -ExecutionPolicy Bypass -File details.ps1
