package fourteen

import (
	"testing"
)

func TestPartOne(t *testing.T) {
	answer := PartOne("./data")
	if answer != 3095 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	answer := PartTwo("./data")
	if answer != 3152788426516 {
		t.Fail()
	}
}
