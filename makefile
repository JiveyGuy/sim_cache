# Fake Name 2023 - Computer Arch. 
# Project - Cache Simulator

##### DATA #####
# generated_data_dir
generated_data_dir = ./data/generated/

all: config_rust_env_logger test_params

# send stderr to logs/err.log
test_params: 
	cargo run -- --bs 32 --us 1024 --is 512 --ds 512 --a 2 --wb true --wa false > $(generated_data_dir)stdouts/test_params.txt 2> $(generated_data_dir)logs/dev.log

config_rust_env_logger:
	export RUST_LOG=debug

