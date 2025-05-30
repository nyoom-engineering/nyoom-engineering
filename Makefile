# usage:
#   make                   # renders both logo and icon with shadows
#   make logo              # renders only logo
#   make icon              # renders only icon
#   WIDTHS="1024 512" make # custom widths for all renders

CRATE_DIR ?= svg-renderer
WIDTHS ?=

# SVG files to render
SVG_FILES := logo icon
LOGO_SVG ?= assets/logo.svg
ICON_SVG ?= assets/logo-icon.svg

# Output directories
LOGO_OUT ?= out/logo
ICON_OUT ?= out/icon

# Map SVG names to their paths
SVG_PATHS := logo:$(LOGO_SVG) icon:$(ICON_SVG)
OUT_DIRS := logo:$(LOGO_OUT) icon:$(ICON_OUT)

BIN := $(CRATE_DIR)/target/release/svg_renderer

all: build $(SVG_FILES) shadows

build:
	cargo build --release --manifest-path $(CRATE_DIR)/Cargo.toml

# Pattern rule for rendering SVGs
$(SVG_FILES): build
	WIDTHS="$(WIDTHS)" $(BIN) $(word 2,$(subst :, ,$(filter $@:%,$(SVG_PATHS)))) $(word 2,$(subst :, ,$(filter $@:%,$(OUT_DIRS))))

# Generate shadows for all PNGs under out/
shadows: $(SVG_FILES)
	cd out && find . -name "*.png" -not -name "*-shadow.png" -exec sh -c 'magick "{}" \( +clone -background black -shadow 40x50+0+36 \) +swap -background transparent -layers merge +repage "$${1%.png}-shadow.png"' sh {} \;

clean:
	cargo clean --manifest-path $(CRATE_DIR)/Cargo.toml
	rm -f out/**/*.png

.PHONY: all build $(SVG_FILES) shadows clean