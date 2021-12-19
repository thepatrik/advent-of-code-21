package thirteen

import (
	"testing"
)

func TestPartOne(t *testing.T) {
	answer := PartOne("./data")
	if answer != 631 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	paper := PartTwo("./data")
	paper.Print()
}
