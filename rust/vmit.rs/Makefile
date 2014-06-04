RUSTC ?= rustc
RUSTC_FLAGS ?=

LIBSRCS = $(shell find src/bin -name '*.rs')
SRCS = $(shell find src/lib -name '*.rs')

all: vmit

vmit: libvmit $(SRCS)
	mkdir -p target
	$(RUSTC) --out-dir target -L./target src/bin/vmit.rs

libvmit: $(LIBSRCS)
	mkdir -p target
	$(RUSTC) --out-dir target src/lib/vmit/lib.rs
 
test: $(SRC)
	mkdir -p target/test
	$(RUSTC) --test --out-dir target/test src/lib/vmit/lib.rs
	./target/test/vmit

clean:
	@rm -rf target


.PHONY: clean
