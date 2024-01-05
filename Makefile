CC=cargo

default: build-release

build-release: FORCE
	$(CC) b --release

clean: FORCE
	-rm -r target

FORCE:
