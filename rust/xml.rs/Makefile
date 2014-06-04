RUSTC ?= rustc
RUSTC_FLAGS ?=

LIBSRCS = $(shell find src/xml -name '*.rs')

all: xml

xml: $(LIBSRCS)
	mkdir -p target
	$(RUSTC) --out-dir target src/xml/lib.rs
 
test: $(SRC)
	mkdir -p target/test
	$(RUSTC) --test --out-dir target/test src/xml/lib.rs
	./target/test/xml

clean:
	@rm -rf target

.PHONY: clean
