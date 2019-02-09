ifdef OS
	TOOLCHAIN = +stable-i686-pc-windows-msvc
	BINARYNAME = tgconnector.dll
	OUPUTNAME = tgconnector.dll
	CP_RELEASE = cp .\target\release\$(BINARYNAME) .\test-server\plugins\$(OUPUTNAME)
	CP_DEBUG = cp .\target\debug\$(BINARYNAME) .\test-server\plugins\$(OUPUTNAME)
else
	ifeq ($(shell uname), Linux)
		TOOLCHAIN = +stable-i686-unknown-linux-gnu
		BINARYNAME = libtgconnector.so
		OUPUTNAME = tgconnector.so
		CP_RELEASE = cp target/release/$(BINARYNAME) test-server/plugins/$(OUPUTNAME)
		CP_DEBUG = cp target/debug/$(BINARYNAME) test-server/plugins/$(OUPUTNAME)
	endif
endif

release:
	cargo $(TOOLCHAIN) build --release
	$(CP_RELEASE)

debug:
	cargo $(TOOLCHAIN) build
	$(CP_DEBUG)

setup:
	cd test-server && mkdir plugins
	cd test-server && mkdir gamemodes
	sampctl package ensure
	sampctl package build
	cd test-server && sampctl server ensure

ensure:
	sampctl package ensure
	
run:
	sampctl package build
	cd test-server && sampctl server run
	
clean:
	cargo clean
