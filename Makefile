run:
	cargo run
build:
	sudo snapcraft
edge:
	sudo snapcraft push --release edge *.snap
publish:
	sudo snapcraft push --release stable *.snap
purge:
	sudo multipass delete snapcraft-tarp && sudo multipass purge