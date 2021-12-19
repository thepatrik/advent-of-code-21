.DEFAULT_GOAL:= all

all: 12 13

.PHONY: 12
12:
	cd go/twelve && go test -v

.PHONY: 13
13:
	cd go/thirteen && go test -v
