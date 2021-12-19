package eight

import (
	"strconv"
	gostrings "strings"

	"github.com/thepatrik/advent-of-code-21/go/pkg/parser"
	"github.com/thepatrik/advent-of-code-21/go/pkg/strings"
)

type Entry struct {
	input  []string
	output []string
}

type Rule func(s string) bool

func PartOne(filename string) int {
	entries := parse(filename)
	sum := 0
	for _, entry := range entries {
		for _, segment := range entry.output {
			switch len(segment) {
			case 2, 3, 4, 7:
				sum += 1
			}
		}
	}

	return sum
}

func PartTwo(filename string) int {
	entries := parse(filename)
	sum := 0
	for _, entry := range entries {
		sum += Output(entry)
	}

	return sum
}

func Output(entry Entry) int {
	find := func(rule Rule) string {
		for _, input := range entry.input {
			if rule(input) {
				return strings.SortString(input)
			}
		}
		return ""
	}

	one := find(func(s string) bool { return len(s) == 2 })
	four := find(func(s string) bool { return len(s) == 4 })
	seven := find(func(s string) bool { return len(s) == 3 })
	eight := find(func(s string) bool { return len(s) == 7 })

	two := find(func(s string) bool {
		return len(s) == 5 && strings.Intersection(four, s) != 3 && strings.Intersection(one, s) != 5
	})
	three := find(func(s string) bool {
		return len(s) == 5 && strings.Intersection(four, s) == 3 && strings.Intersection(one, s) == 2
	})
	five := find(func(s string) bool {
		return len(s) == 5 && strings.Intersection(four, s) == 3 && strings.Intersection(one, s) == 1
	})
	six := find(func(s string) bool { return len(s) == 6 && strings.Intersection(one, s) == 1 })
	nine := find(func(s string) bool {
		return len(s) == 6 && strings.Intersection(one, s) == 2 && strings.Intersection(five, s) == 5
	})
	zero := find(func(s string) bool {
		return len(s) == 6 && strings.Intersection(one, s) == 2 && strings.Intersection(five, s) == 4
	})

	output := ""

	for _, o := range entry.output {
		switch strings.SortString(o) {
		case zero:
			output += "0"
		case one:
			output += "1"
		case two:
			output += "2"
		case three:
			output += "3"
		case four:
			output += "4"
		case five:
			output += "5"
		case six:
			output += "6"
		case seven:
			output += "7"
		case eight:
			output += "8"
		case nine:
			output += "9"
		}
	}

	i, _ := strconv.Atoi(output)

	return i
}

func parse(filename string) []Entry {
	strslice := parser.ReadFile("./data")
	entries := make([]Entry, 0)
	for _, line := range strslice {
		split := gostrings.Split(gostrings.TrimSpace(line), "|")
		rhs := split[0]
		input := gostrings.Split(gostrings.TrimSpace(rhs), " ")
		lhs := split[1]
		output := gostrings.Split(gostrings.TrimSpace(lhs), " ")
		entry := Entry{
			input:  input,
			output: output,
		}
		entries = append(entries, entry)
	}
	return entries
}
