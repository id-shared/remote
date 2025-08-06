vgc-t:
  netsh advfirewall firewall set rule name="vgc" new enable=yes

vgc-f:
  netsh advfirewall firewall set rule name="vgc" new enable=no

vgc-n:
  taskkill //IM vgc.exe //F

scp-release:
  clear && RUST_BACKTRACE=1 cargo run --bin scp --release

scp:
  clear && RUST_BACKTRACE=1 cargo run --bin scp

cdp:
  clear && RUST_BACKTRACE=1 cargo run --bin cdp

x-release:
  clear && RUST_BACKTRACE=1 cargo run --bin x --release

x:
  clear && RUST_BACKTRACE=1 cargo run --bin x

test:
  clear && RUST_BACKTRACE=1 cargo run --bin test

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

build:
  powershell -ExecutionPolicy Bypass -File details.ps1
