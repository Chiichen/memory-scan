run:
	RUST_LOG=info cargo xtask run

build-ebpf:
	cargo xtask build-ebpf

print:
	python ./tools/print_figure.py ./map.csv