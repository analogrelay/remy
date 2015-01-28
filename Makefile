CARGOFILES=$(wildcard */Cargo.toml)
CARGODIRS=$(foreach cargofile,$(CARGOFILES),$(notdir $(patsubst %/,%,$(dir $(cargofile)))))

define RUST_template
.PHONY: $(1)
build_$(1):
	@cd $(1) && cargo build --verbose
test_$(1):
	@cd $(1) && cargo test --verbose
doc_$(1):
	@cd $(1) && cargo doc --verbose
$(1): build_$(1) test_$(1)
endef

build: $(foreach cargodir,$(CARGODIRS),build_$(cargodir))

test: $(foreach cargodir,$(CARGODIRS),test_$(cargodir))

doc: $(foreach cargodir,$(CARGODIRS),doc_$(cargodir))

all: build test

travis: all

$(foreach cargodir,$(CARGODIRS), $(eval $(call RUST_template,$(cargodir))))