.DEFAULT_GOAL:= all

all: 13

13:
	cd go/thirteen && go test -v

.PHONY: 13