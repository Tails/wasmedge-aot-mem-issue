test:
	rm -rf ../prerendering_wasm/dist \
		&& . ${HOME}/.wasmedge/env \
		&& . ${HOME}/.cargo/env \
		&& cargo test \
				-vv \
				--color=always \
				--manifest-path \
					./Cargo.toml \
			-- \
				--nocapture