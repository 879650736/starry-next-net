ARCH ?= riscv64
MODE ?= release
LIBC ?= musl

build_dir := $(PWD)/apps/iperf_wrap/build/$(ARCH)-$(LIBC)-iperf

build: build_dir
	@echo "Building iperf for $(ARCH) with $(LIBC)..."
	@echo "build_dir: $(build_dir)"
	@echo "pwd: $(PWD)"
	@mkdir -p $(build_dir) build/riscv64
	@cd ../iperf && ./build.sh $(build_dir)
	@cp ../iperf/src/iperf3 build/riscv64
	@echo "iperf built successfully in $(build_dir)"

build_dir:
	@mkdir -p $(build_dir)

clean:
	@echo "Cleaning iperf..."
	@rm -rf build

.PHONY: build build_dir clean