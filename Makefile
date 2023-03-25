NAME = music-box
PLATFORM := arm-unknown-linux-gnueabihf
VERSION = $(shell cargo pkgid | cut -d\# -f2 | cut -d: -f2)
CROSS_VERSION = $(shell cross --version | head -n 1 | cut -d ' ' -f 2)

RS := $(shell find ./src -name '*.rs')
OUT = ./target
TAR = $(addprefix ${NAME}-$(VERSION)-, $(addsuffix .tar.gz, $(PLATFORM)))

CROSS_PLATFORM := $(addprefix $(OUT)/, $(addsuffix /release/${NAME}, $(PLATFORM)))
TARGET = $(OUT)/%/release/${NAME}
CROSS_FLAGS = --release --target=$*

$(PLATFORM):
	docker build --build-arg CROSS_PLATFORM=$@ --build-arg CROSS_VERSION=${CROSS_VERSION} -t ${NAME}-$@:latest .
	rustup target add $@

init: $(PLATFORM)

$(CROSS_PLATFORM): $(TARGET): $(RS) Cargo.toml
	cross build $(CROSS_FLAGS)

$(TAR): ${NAME}-$(VERSION)-%.tar.gz: release
	@rm -f ${NAME}-$(VERSION)-$*.tar.gz
	@tar czvf $@ -C ./target/$*/release ${NAME}

stop:
	ssh pi@${NAME}.local 'sudo systemctl stop ${NAME} && pkill -9 ${NAME} || echo '

start:
	ssh pi@${NAME}.local 'sudo systemctl start ${NAME}'

status:
	ssh pi@${NAME}.local 'sudo systemctl status ${NAME}'

ssh:
	ssh pi@${NAME}.local

rsync:
	rsync -azP loops pi@${NAME}.local:~/.local/share/music_box
	rsync -azP speech pi@${NAME}.local:~/.local/share/music_box
	rsync -azP pitch pi@${NAME}.local:~/.local/share/music_box

release: $(CROSS_PLATFORM)

install: $(CROSS_PLATFORM)
	@make stop 
	rsync $< pi@${NAME}.local:~
	@make start

debug: $(CROSS_PLATFORM)
	@make stop 
	rsync $< pi@${NAME}.local:~
	ssh pi@${NAME}.local 'RUST_LOG=debug RUST_BACKTRACE=full ./${NAME}'

clean:
	@cargo clean
	@rm ${NAME}-$(VERSION)-*.tar.gz

.PHONY: clean release debug install stop start status ssh init
.DEFAULT: release