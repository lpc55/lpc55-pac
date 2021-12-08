YAML = lpc55.yaml
SVD = lpc55.svd.patched
PACK_VERSION = 13.0.0

# Path to `svd`/`svdtools`
SVDTOOLS ?= svd

# this make target runs under svdtools, ensure it works at all times.
check: patch generate
	cargo check

build: setup patch generate
	cargo build

doc: setup patch generate
	cargo doc
doc-open: setup patch generate
	cargo doc --open

setup: update-venv fetch-svd
	cargo install svd2rust --version 0.20.0

patch:
	$(SVDTOOLS) patch $(YAML)
	sed -i '/alternateGroup/d' $(SVD)

# Generates PAC source code from (patched) SVD
generate:
	rm -rf src
	mkdir src
	svd2rust -i ./$(SVD)
	form -i lib.rs -o src/ && rm lib.rs
	cargo fmt

show-latest-pack:
	@echo $(shell wget -O - -qq https://mcuxpresso.nxp.com/cmsis_pack/repo/NXP.pidx|grep LPC55S69|python -c'import sys; print(sys.stdin.read().rsplit("version=\"", 1)[1].split("\"", 1)[0])')

PACK := NXP.LPC55S69_DFP.$(PACK_VERSION).pack
fetch-svd:
	wget -qqO- https://mcuxpresso.nxp.com/cmsis_pack/repo/$(PACK) | bsdtar -xf- LPC55S69_cm33_core0.xml
	mv LPC55S69_cm33_core0.xml svd/pack-$(PACK_VERSION)-LPC55S69_cm33_core0.xml
	ln -sf svd/pack-$(PACK_VERSION)-LPC55S69_cm33_core0.xml lpc55.svd

# External documentation
fetch-docs:
	mkdir -p ref
	curl -s https://www.nxp.com/docs/en/data-sheet/LPC55S6x.pdf \
		-o ref/datasheet-lpc55s6x.pdf
	curl -s https://www.nxp.com/docs/en/errata/ES_LPC55S6x.pdf \
		-o ref/errata-lpc55s6x.pdf
	curl -sk https://static.docs.arm.com/100235/0004/arm_cortex_m33_dgug_100235_0004_00_en.pdf \
		-o ref/genericuserguide-cortexm33.pdf
	echo "For the LPC55S6x User manual (UM11126), login is needed"
	# curl -s https://www.nxp.com/docs/en/user-guide/UM11126.pdf \
	# 	-o ref/usermanual-lpc55s6x.pdf
	# `pip install nxp-dlagent` for the following step
	nxp-dl UM11126 && mv UM11126.pdf ref/

# Maintenance
VERSION := $(shell grep version Cargo.toml|head -1|cut -d' ' -f 3|tr -d '"')
tag:
	git tag -a $(VERSION) -m"v$(VERSION)"

# For CI
rustup:
	rustup target add thumbv8m.main-none-eabi

version:
	echo $(VERSION)

update-venv: venv
	venv/bin/pip install -U pip
	venv/bin/pip install -r requirements.txt

venv:
	python3 -m venv venv
