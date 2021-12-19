package strings

import (
	"strings"
)

func Intersection(s1, s2 string) int {
	sum := 0
	for _, r := range s1 {
		if strings.ContainsRune(s2, r) {
			sum += 1
		}
	}
	return sum
}
