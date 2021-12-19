.DEFAULT_GOAL:= all

all: 7 8 9 10 11 12 13

.PHONY: 7
7:
	cd rust && cargo test --release --bin 7

.PHONY: 8
8:
	cd go/eight && go test -v

.PHONY: 9
9:
	cd go/nine && go test -v

.PHONY: 10
10:
	cd go/ten && go test -v

.PHONY: 11
11:
	cd go/eleven && go test -v

.PHONY: 12
12:
	cd go/twelve && go test -v

.PHONY: 13
13:
	cd go/thirteen && go test -v
