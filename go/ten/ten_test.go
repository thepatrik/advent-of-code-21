package nine

import "testing"

func TestPartOne(t *testing.T) {
	answer := PartOne("./data")
	if answer != 216297 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	answer := PartTwo("./data")
	if answer != 2165057169 {
		t.Fail()
	}
}
