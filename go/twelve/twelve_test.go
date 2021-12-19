package twelve

import "testing"

func TestPartOne(t *testing.T) {
	answer := PartOne("./data")
	if answer != 4720 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	answer := PartTwo("./data")
	if answer != 147848 {
		t.Fail()
	}
}
