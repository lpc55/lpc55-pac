YAML = lpc55.yaml
SVD = lpc55.svd.patched

build: patch generate work-around-missing-svd2rust-release
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

work-around-missing-svd2rust-release:
	sed -i '/bare-metal/d' Cargo.toml
	sed -i 's/bare_metal::Nr/cortex_m::interrupt::Nr/' src/lib.rs
	sed -i '/bare_metal/d' src/lib.rs
	sed -i '/legacy_directory_ownership/d' src/lib.rs
	sed -i '/plugin_as_library/d' src/lib.rs
	sed -i '/safe_extern_statics/d' src/lib.rs
	sed -i '/unions_with_drop_fields/d' src/lib.rs

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

# PACK_VERSION := 12.4.0
# Get https://mcuxpresso.nxp.com/cmsis_pack/repo/NXP.pidx to see if there's a newer pack
PACK_VERSION := $(shell wget -O - -qq https://mcuxpresso.nxp.com/cmsis_pack/repo/NXP.pidx|grep LPC55S69|python -c'import sys; print(sys.stdin.read().rsplit("version=\"", 1)[1].split("\"", 1)[0])')
PACK := NXP.LPC55S69_DFP.$(PACK_VERSION).pack
fetch-latest-svd:
	wget -qqO- https://mcuxpresso.nxp.com/cmsis_pack/repo/$(PACK) | bsdtar -xf- LPC55S69_cm33_core0.xml

# Maintenance
VERSION := $(shell grep version Cargo.toml|head -1|cut -d' ' -f 3|tr -d '"')
tag:
	git tag -a $(VERSION) -m"v$(VERSION)"

# For CI
rustup:
	rustup target add thumbv8m.main-none-eabi

version:
	echo $(VERSION)
