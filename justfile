make x:
  clear && RUST_BACKTRACE=1 cargo run --bin {{x}} --release

run x:
  clear && RUST_BACKTRACE=1 cargo run --bin {{x}}

update:
  rustup update
  cargo update

proxy:
  mitmweb --mode local:vgc.exe --script mitm.py

devices:
  ./devices.bat

clean:
  rm -rf ./target

build:
  powershell -ExecutionPolicy Bypass -File details.ps1
