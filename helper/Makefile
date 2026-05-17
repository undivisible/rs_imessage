SHELL := /bin/bash
ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/..)
IMSG_SRC ?= $(ROOT)/helper/imsg-upstream
OUT_DIR := $(ROOT)/lib
DYLIB := $(OUT_DIR)/imsg-bridge-helper.dylib
ARCH ?= arm64e

.PHONY: all clean vendor

all: $(DYLIB)

vendor:
	@if [[ ! -f "$(IMSG_SRC)/Sources/IMsgHelper/IMsgInjected.m" ]]; then \
		rm -rf "$(IMSG_SRC)"; \
		git clone --depth 1 https://github.com/openclaw/imsg.git "$(IMSG_SRC)"; \
	fi

$(DYLIB): vendor
	@mkdir -p "$(OUT_DIR)"
	clang -dynamiclib -arch $(ARCH) -fobjc-arc \
		-Wno-arc-performSelector-leaks \
		-framework Foundation \
		-o "$(DYLIB)" \
		"$(IMSG_SRC)/Sources/IMsgHelper/IMsgInjected.m"
	@echo "Built $(DYLIB)"

clean:
	rm -f "$(DYLIB)"
