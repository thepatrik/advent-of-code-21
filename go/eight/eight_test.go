package eight

import "testing"

func TestPartOne(t *testing.T) {
	answer := PartOne("./data")
	if answer != 445 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	answer := PartTwo("./data")
	if answer != 1043101 {
		t.Fail()
	}
}
