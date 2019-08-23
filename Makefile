YAML = lpc55s69-core0.yaml
SVD = lpc55s69-core0.svd.patched

prepare: patch generate
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
	curl -s https://www.nxp.com/docs/en/user-guide/UM11126.pdf \
		-o ref/usermanual-lpc55s6x.pdf
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

# Documentation - hosted on <https://lpc55s6x.netlify.com/>
docs-setup: lpc55s6x-pac-docs

lpc55s6x-pac-docs:
	git clone git@github.com:nickray/lpc55s6x-pac-docs.git

docs-build:
	cargo doc
	cp -a target/doc/. lpc55s6x-pac-docs/docs/

docs-open:
	xdg-open lpc55s6x-pac-docs/docs/lpc55s6x_pac/index.html

# docs-publish:
# 	cd lpc55s6x-pac-docs && git add . && git commit -m'Update docs' && git push
