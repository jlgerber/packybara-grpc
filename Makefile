EXE := packybara-grpc-server
MODE ?= release
TARGET ?= ~/bin/.

build:
ifeq ($(MODE),release)
	cargo build --release
else
	cargo build
endif

install:
	cp target/${MODE}/${EXE} ${TARGET}

all: build install