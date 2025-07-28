# PRODUCTION MAKEFILE

colex:
	@cargo build --release

install: colex
	@if [ "$(shell whoami)" != "root" ]; then \
		echo "You must be root to install this package!"; \
		exit 1; \
	fi
	@install -Dm755 target/release/colex /usr/local/bin/colex
	@echo "Installed to /usr/local/bin/colex"

clean:
	@cargo clean

