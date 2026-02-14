KERNEL_TARGET_SPEC=$(ROOT)/kernel/targets/$(ARCH)-unknown-kernel.json
KERNEL_LINKER_SCRIPT=$(ROOT)/kernel/linkers/$(ARCH).ld
KERNEL_CARGO=cargo +$(KERNEL_TOOLCHAIN)
KERNEL_RUSTFLAGS=-C link-arg=-T -C link-arg=$(KERNEL_LINKER_SCRIPT) -C link-arg=-z -C link-arg=max-page-size=0x1000

build/kernel: kernel/Cargo.toml $(KERNEL_TARGET_SPEC) $(KERNEL_LINKER_SCRIPT) build/initfs.tag
	INITFS_FOLDER=$(ROOT)/build/initfs $(KERNEL_CARGO) rustc \
		--manifest-path $(ROOT)/kernel/Cargo.toml \
		--bin kernel \
		--target $(KERNEL_TARGET_SPEC) \
		--release \
		-Z build-std=core,alloc \
		-Z build-std-features=compiler-builtins-mem \
		-- \
		$(KERNEL_RUSTFLAGS) \
		--emit link=$@.all
	objcopy --only-keep-debug $@.all $@.sym
	objcopy --strip-debug $@.all $@

build/kernel_live: kernel/Cargo.toml $(KERNEL_TARGET_SPEC) $(KERNEL_LINKER_SCRIPT) build/initfs_live.tag
	INITFS_FOLDER=$(ROOT)/build/initfs_live $(KERNEL_CARGO) rustc \
		--manifest-path $(ROOT)/kernel/Cargo.toml \
		--bin kernel \
		--target $(KERNEL_TARGET_SPEC) \
		--release \
		-Z build-std=core,alloc \
		-Z build-std-features=compiler-builtins-mem \
		-- \
		$(KERNEL_RUSTFLAGS) \
		--emit link=$@.all
	objcopy --strip-debug $@.all $@
