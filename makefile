ifdef OS
	TOOLCHAIN = +stable-i686-pc-windows-msvc
	BINARYNAME = tgconnector.dll
	OUPUTNAME = tgconnector.dll
	CP_RELEASE = cp .\target\release\$(BINARYNAME) .\plugins\$(OUPUTNAME)
	CP_DEBUG = cp .\target\debug\$(BINARYNAME) .\plugins\$(OUPUTNAME)
else
	ifeq ($(shell uname), Linux)
		TOOLCHAIN = +stable-i686-unknown-linux-gnu
		BINARYNAME = libtgconnector.so
		OUPUTNAME = tgconnector.so
		CP_RELEASE = cp target/release/$(BINARYNAME) plugins/$(OUPUTNAME)
		CP_DEBUG = cp target/debug/$(BINARYNAME) plugins/$(OUPUTNAME)
	endif
endif

release:
	cargo $(TOOLCHAIN) build --release
	$(CP_RELEASE)

debug:
	cargo $(TOOLCHAIN) build
	$(CP_DEBUG)

setup:
	sampctl package ensure
	sampctl package build

ensure:
	sampctl package ensure

run:
	sampctl package build
	sampctl package run

clean:
	cargo clean
