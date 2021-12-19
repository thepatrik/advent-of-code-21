package eleven

import "testing"

func TestPartOne(t *testing.T) {
	answer := PartOne("./data")
	if answer != 1667 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	answer := PartTwo("./data")
	if answer != 488 {
		t.Fail()
	}
}
