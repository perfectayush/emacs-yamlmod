build:  yamlmod.so

yamlmod.so:
	cargo build --release
	ln -s target/release/libyamlmod.dylib yamlmod.so

