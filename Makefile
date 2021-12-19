.DEFAULT_GOAL:= all

all: 11 12 13

.PHONY: 11
11:
	cd go/eleven && go test -v

.PHONY: 12
12:
	cd go/twelve && go test -v

.PHONY: 13
13:
	cd go/thirteen && go test -v
