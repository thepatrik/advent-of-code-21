package twelve

import (
	"strings"
	"unicode"

	"github.com/thepatrik/advent-of-code-21/go/pkg/parser"
)

func PartOne(filename string) int {
	paths := parse(filename)
	return count(paths, "start", []string{}, true)
}

func PartTwo(filename string) int {
	paths := parse(filename)
	return count(paths, "start", []string{}, false)
}

func count(paths map[string][]string, current string, path []string, twice bool) int {
	if current == "end" {
		return 1
	}

	contains := func(strslice []string, s string) bool {
		for _, str := range strslice {
			if str == s {
				return true
			}
		}
		return false
	}

	isLower := func(s string) bool {
		for _, r := range s {
			if !unicode.IsLower(r) && unicode.IsLetter(r) {
				return false
			}
		}
		return true
	}

	if isLower(current) && contains(path, current) {
		if twice || current == "start" {
			return 0
		}
		twice = true
	}

	path = append(path, current)

	sum := 0
	for _, v := range paths[current] {
		sum += count(paths, v, path, twice)
	}

	//lint:ignore SA4006 this is used (recursion)
	path = path[:len(path)-1]
	return sum
}

func parse(filename string) map[string][]string {
	strslice := parser.ReadFile(filename)
	paths := make(map[string][]string)
	for _, line := range strslice {
		split := strings.Split(line, "-")
		if _, ok := paths[split[0]]; !ok {
			paths[split[0]] = make([]string, 0)
		}
		if _, ok := paths[split[1]]; !ok {
			paths[split[1]] = make([]string, 0)
		}

		paths[split[0]] = append(paths[split[0]], split[1])
		paths[split[1]] = append(paths[split[1]], split[0])
	}
	return paths
}
