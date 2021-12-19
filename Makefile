.DEFAULT_GOAL:= all

all: 1 2 3 7 8 9 10 11 12 13

.PHONY: 1
1:
	cd rust && cargo test --release --bin 1

.PHONY: 2
2:
	cd rust && cargo test --release --bin 2

.PHONY: 3
3:
	cd rust && cargo test --release --bin 3

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
