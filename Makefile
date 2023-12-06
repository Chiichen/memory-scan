run:
	RUST_LOG=info cargo xtask run

build-ebpf:
	cargo xtask build-ebpf

raw-print:
	python ./tools/print_figure.py ./map.csv

sort:
	python ./tools/sort.py ./map.csv

print:
	python ./tools/sort.py ./map.csv
	python ./tools/print_figure.py ./sorted_csv.csv