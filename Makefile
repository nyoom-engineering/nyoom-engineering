# usage:
#   make                   # renders all SVGs with shadows
#   make logo              # renders only logo
#   make icon              # renders only icon
#   make base16           # renders only base16 palette
#   WIDTHS="1024 512" make # custom widths for all renders

SVG_CRATE_DIR ?= svg-renderer
SVG_REPLACE_DIR ?= svg-replace
BASE16_CRATE_DIR ?= base16-renderer
WIDTHS ?=

# SVG files to render
SVG_FILES := logo icon
LOGO_SVG ?= assets/logo.svg
ICON_SVG ?= assets/logo-icon.svg
BASE16_JSON ?= assets/oxocarbon-dark.json

# Output directories
LOGO_OUT ?= out/logo
ICON_OUT ?= out/icon

# Map SVG names to their paths
SVG_PATHS := logo:$(LOGO_SVG) icon:$(ICON_SVG)
OUT_DIRS := logo:$(LOGO_OUT) icon:$(ICON_OUT)

SVG_BIN := $(SVG_CRATE_DIR)/target/release/svg_renderer
REPLACE_BIN := $(SVG_REPLACE_DIR)/target/release/svg_replace
BASE16_BIN := ${BASE16_CRATE_DIR}/target/release/base16_renderer

all: build $(SVG_FILES) base16 custom shadows

build:
	cargo build --release --manifest-path $(SVG_CRATE_DIR)/Cargo.toml
	cargo build --release --manifest-path $(SVG_REPLACE_DIR)/Cargo.toml
	cargo build --release --manifest-path $(BASE16_CRATE_DIR)/Cargo.toml

# Generate the palette SVG and render it
base16: build
	$(BASE16_BIN) $(BASE16_JSON)
	WIDTHS="3840" $(SVG_BIN) palette.svg out/palette
	mv palette.svg out/

custom: build
	$(REPLACE_BIN) "Base16" "Oxocarbon"
	$(SVG_BIN) logo-custom.svg out/logo-custom
	mv logo-custom.svg out/

# Pattern rule for rendering SVGs
$(SVG_FILES): build
	WIDTHS="$(WIDTHS)" $(SVG_BIN) $(word 2,$(subst :, ,$(filter $@:%,$(SVG_PATHS)))) $(word 2,$(subst :, ,$(filter $@:%,$(OUT_DIRS))))

# Generate shadows for all PNGs under out/
shadows: $(SVG_FILES) base16
	cd out && find . -name "*.png" -not -name "*-shadow.png" -exec sh -c 'magick "{}" \( +clone -background black -shadow 40x50+0+36 \) +swap -background transparent -layers merge +repage "$${1%.png}-shadow.png"' sh {} \;

clean:
	cargo clean --manifest-path $(SVG_CRATE_DIR)/Cargo.toml
	cargo clean --manifest-path $(SVG_REPLACE_DIR)/Cargo.toml
	cargo clean --manifest-path $(BASE16_CRATE_DIR)/Cargo.toml
	rm -rf out

.PHONY: all build $(SVG_FILES) base16 custom shadows clean