# rust-rename-it

->build for windows (cross compile on linux or mac)
#update config file
cat >>~/.cargo/config <<EOF
> [target.x86_64-pc-windows-gnu]
> linker = "x86_64-w64-mingw32-gcc"
> EOF
#use target in cargo build
cargo build --target=x86_64-pc-windows-gnu 
