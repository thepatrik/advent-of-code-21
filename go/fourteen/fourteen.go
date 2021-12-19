package fourteen

import (
	"strings"

	"github.com/thepatrik/advent-of-code-21/go/pkg/parser"
)

func PartOne(filename string) int {
	template, rules := parse(filename)
	return count(template, rules, 10)
}

func PartTwo(filename string) int {
	template, rules := parse(filename)
	return count(template, rules, 40)
}

func count(template string, rules map[string]string, steps int) int {
	pairs := make(map[string]int)

	for i := 1; i < len(template); i++ {
		pairs[template[i-1:i+1]]++
	}

	for i := 0; i < steps; i++ {
		next := make(map[string]int)
		for pair, n := range pairs {
			if rule, ok := rules[pair]; ok {
				left, right := pair[0:1]+rule, rule+pair[1:2]
				next[left] += n
				next[right] += n
			} else {
				next[pair] += n
			}
		}
		pairs = next
	}

	chars := make(map[string]int)
	for pair, n := range pairs {
		chars[pair[0:1]] += n
	}

	chars[template[len(template)-1:]] += 1

	min, max := minMax(chars)

	return max - min
}

func minMax(m map[string]int) (int, int) {
	min, max := 0, 0

	for _, v := range m {
		if min == 0 || v < min {
			min = v
		}
		if v > max {
			max = v
		}
	}

	return min, max
}

func parse(filename string) (string, map[string]string) {
	strslice := parser.ReadFile(filename)
	rules := make(map[string]string)
	template := ""
	for i, line := range strslice {
		if i == 0 {
			template = line
			continue
		}

		if line == "" {
			continue
		}

		split := strings.Split(line, " -> ")
		key := split[0][0:2]
		rules[key] = split[1][0:1]
	}
	return template, rules
}
