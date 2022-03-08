RSOURCES := $(wildcard curiefense/src/**.rs curiefense-lua/src/**.rs)

PHONY: all

all: ../lua/shared-objects/curiefense.hs5.so ../lua/shared-objects/curiefense.so

ctarget:
	mkdir ctarget

ctarget/registry: ctarget
	mkdir -p ctarget/registry

ctarget/target: ctarget
	mkdir -p ctarget/target

ctarget/target/release/libcuriefense_lua.so: container ctarget/registry ctarget/target $(RSOURCES)
	docker run --rm -v `pwd`:/home/builder/rust -v `pwd`/ctarget/registry:/home/builder/.cargo/registry  -w /home/builder/rust  -t curiefense/rustbuild:latest /home/builder/.cargo/bin/cargo build --release --target-dir ctarget/target

ctarget/target/debug/libcuriefense_lua.so: container ctarget/registry ctarget/target $(RSOURCES)
	docker run --rm -v `pwd`:/home/builder/rust -v `pwd`/ctarget/registry:/home/builder/.cargo/registry  -w /home/builder/rust  -t curiefense/rustbuild:latest /home/builder/.cargo/bin/cargo build --target-dir ctarget/target

target/release/libcuriefense_lua.so: Cargo.toml curiefense/Cargo.toml curiefense-lua/Cargo.toml $(RSOURCES)
	cargo build --release

../lua/shared-objects/curiefense.hs5.so: target/release/libcuriefense_lua.so
	cp target/release/libcuriefense_lua.so ../lua/shared-objects/curiefense.hs5.so

../lua/shared-objects/curiefense.so: ctarget/target/release/libcuriefense_lua.so
	cp ctarget/target/release/libcuriefense_lua.so ../lua/shared-objects/curiefense.so

.PHONY: all container test test-nodeps

container:
	tar c Dockerfile  |  docker build -t curiefense/rustbuild -f Dockerfile -

test: ctarget/target/debug/libcuriefense_lua.so test-nodeps
	echo DONE

test-nodeps:
	docker run -v `pwd`/ctarget/target/debug/libcuriefense_lua.so:/usr/local/lib/lua/5.1/curiefense.so \
						 -v `pwd`/../lua:/home/builder/lua \
						 -v `pwd`/luatests/redis.lua:/home/builder/lua/redis.lua \
						 -v `pwd`/../lua/shared-objects/luahs.so:/usr/local/lib/lua/5.1/luahs.so \
						 -v `pwd`/../lua/shared-objects/libinjection.so:/usr/local/lib/lua/5.1/libinjection.so \
						 -v `pwd`/../lua/shared-objects/grasshopper.so:/usr/local/lib/lua/5.1/grasshopper.so \
						 -v `pwd`/../../images/confserver/bootstrap/confdb-initial-data/master/config:/config/current/config \
						 -v `pwd`/luatests/test.lua:/home/builder/test.lua \
						 -v `pwd`/luatests/run.sh:/home/builder/run.sh \
						 -v `pwd`/luatests/config/json:/config/current/config/json \
						 -v `pwd`/luatests:/home/builder/luatests \
						 --rm \
						 -w /home/builder -t curiefense/rustbuild:latest \
						 /bin/sh /home/builder/run.sh

gowaftest: ctarget/target/debug/libcuriefense_lua.so gowaftest-nodeps
	echo DONE

gowaftest-nodeps:
	docker run -v `pwd`/ctarget/target/debug/libcuriefense_lua.so:/usr/local/lib/lua/5.1/curiefense.so \
						-v `pwd`/../lua:/home/builder/lua \
						-v `pwd`/luatests/redis.lua:/home/builder/lua/redis.lua \
						-v `pwd`/../lua/shared-objects/luahs.so:/usr/local/lib/lua/5.1/luahs.so \
						-v `pwd`/../lua/shared-objects/libinjection.so:/usr/local/lib/lua/5.1/libinjection.so \
						-v `pwd`/../lua/shared-objects/grasshopper.so:/usr/local/lib/lua/5.1/grasshopper.so \
						-v `pwd`/../../images/confserver/bootstrap/confdb-initial-data/master/config:/config/current/config \
						-v `pwd`/luatests/test.lua:/home/builder/test.lua \
						-v `pwd`/luatests/run.sh:/home/builder/run.sh \
						-v `pwd`/luatests:/home/builder/luatests \
						--rm \
						-w /home/builder -t curiefense/rustbuild:latest \
						/bin/sh /home/builder/run.sh GOWAF

