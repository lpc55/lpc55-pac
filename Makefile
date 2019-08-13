build:
	# TODO: add patches
	./update.sh
	cargo build

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

