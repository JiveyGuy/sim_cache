all: config_rust_env_logger test_params

# send std error to dev.log
test_params: 
	cargo run -- --bs 32 --us 1024 --is 512 --ds 512 --a 2 --wb true --wa false > test_params.txt 2> dev.log

config_rust_env_logger:
	export RUST_LOG=debug

