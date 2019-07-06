# rust-rename-it
Test code:
```
mkdir test
touch test/rust.cha
cargo run test cha txt
```

->build for windows (cross compile on mac)
```
#install rust target for windows-gnu
rustup target add x86_64-pc-windows-gnu
#install toolchain
brew install mingw-w64
#update config file
cat >>~/.cargo/config <<EOF
> [target.x86_64-pc-windows-gnu]
> linker = "x86_64-w64-mingw32-gcc"
> EOF
#use target in cargo build
cargo build --target=x86_64-pc-windows-gnu 
```

->build for linux (cross compile on mac)
```
#install rust target for linux-gnu
rustup target add x86_64-unknown-linux-gnu
#install tollchain
brew tap SergioBenitez/osxct 
brew install x86_64-unknown-linux-gnu
cat >>~/.cargo/config <<EOF
> [target.x86_64-unknown-linux-gnu]
> linker = "x86_64-unknown-linux-gnu-gcc"
> EOF
#use target in cargo build
cargo build --target=x86_64-unknown-linux-gnu
```
