.DEFAULT_GOAL:= all

all: 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15

.PHONY: 1
1:
	cd rust && cargo test --release --bin 1

.PHONY: 2
2:
	cd rust && cargo test --release --bin 2

.PHONY: 3
3:
	cd rust && cargo test --release --bin 3

.PHONY: 4
4:
	cd rust && cargo test --release --bin 4

.PHONY: 5
5:
	cd rust && cargo test --release --bin 5

.PHONY: 6
6:
	cd rust && cargo test --release --bin 6

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

.PHONY: 14
14:
	cd go/fourteen && go test -v

.PHONY: 15
15:
	cd go/fifteen && go test -v
