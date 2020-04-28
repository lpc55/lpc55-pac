YAML = lpc55.yaml
SVD = lpc55.svd.patched

build: patch generate
	cargo build

patch:
	svd patch $(YAML)

# Generates PAC source code from (patched) SVD
generate:
	rm -rf src
	mkdir src
	svd2rust -i ./$(SVD)
	form -i lib.rs -o src/ && rm lib.rs
	cargo fmt

# External documentation
fetch-docs:
	mkdir -p ref
	curl -s https://www.nxp.com/docs/en/data-sheet/LPC55S6x.pdf \
		-o ref/datasheet-lpc55s6x.pdf
	echo "For the LPC55S6x User manual (UM11126), login is needed"
	# curl -s https://www.nxp.com/docs/en/user-guide/UM11126.pdf \
	# 	-o ref/usermanual-lpc55s6x.pdf
	curl -s https://www.nxp.com/docs/en/errata/ES_LPC55S6x.pdf \
		-o ref/errata-lpc55s6x.pdf
	curl -sk https://static.docs.arm.com/100235/0004/arm_cortex_m33_dgug_100235_0004_00_en.pdf \
		-o ref/genericuserguide-cortexm33.pdf

# Maintenance
VERSION := $(shell grep version Cargo.toml|head -1|cut -d' ' -f 3|tr -d '"')
tag:
	git tag -a $(VERSION) -m"v$(VERSION)"

# For CI
rustup:
	rustup target add thumbv8m.main-none-eabi

version:
	echo $(VERSION)
