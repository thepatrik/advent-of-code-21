package nine

import "testing"

func TestPartOne(t *testing.T) {
	answer := PartOne("./data")
	if answer != 600 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	answer := PartTwo("./data")
	if answer != 987840 {
		t.Fail()
	}
}
