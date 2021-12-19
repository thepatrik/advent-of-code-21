package nine

import (
	"fmt"
	"sort"

	"github.com/thepatrik/advent-of-code-21/go/pkg/parser"
	"github.com/thepatrik/advent-of-code-21/go/pkg/stack"
)

type Bracket int

const (
	Round Bracket = iota
	Square
	Curly
	Angle
)

type ParseError struct {
	Char rune
	Pos  int
	Text string
}

func (p *ParseError) Error() string {
	return fmt.Sprintf("err on pos %d, err %v", p.Pos, p.Text)
}

func PartOne(filename string) int {
	lines := parse(filename)
	sum := 0

	for _, line := range lines {
		err := findErr(line)
		if err != nil {
			switch errType := err.(type) {
			case *ParseError:
				switch errType.Char {
				case ')':
					sum += 3
				case ']':
					sum += 57
				case '}':
					sum += 1197
				case '>':
					sum += 25137
				}
			}
		}
	}
	return sum
}

func PartTwo(filename string) int {
	lines := parse(filename)
	patches := make([][]rune, 0)

	for _, line := range lines {
		err := findErr(line)
		if err == nil {
			patch := patch(line)
			patches = append(patches, patch)
		}
	}

	scores := make([]int, 0)

	for _, patch := range patches {
		score := 0
		for _, char := range patch {
			score *= 5
			switch char {
			case ')':
				score += 1
			case ']':
				score += 2
			case '}':
				score += 3
			case '>':
				score += 4
			}
		}
		scores = append(scores, score)
	}

	sort.Ints(scores)

	return scores[len(scores)/2]
}

func findErr(line []rune) error {
	s := make(stack.RuneStack, 0)

	for pos, char := range line {
		switch char {
		case '[', '(', '{', '<':
			s = s.Push(char)
		case ']', ')', '}', '>':
			var popped rune
			var err error
			s, popped, err = s.Pop()

			if err == stack.ErrEmptyStack || char != closingChar(popped) {
				return &ParseError{Char: char, Pos: pos, Text: "bracket error"}
			}
		}
	}
	return nil
}

func patch(line []rune) []rune {
	patch := make([]rune, 0)
	patchmap := make(map[Bracket]int)

	score := func(char rune) int {
		switch char {
		case '[', '(', '{', '<':
			return -1
		}
		return 1
	}

	for i := len(line) - 1; i >= 0; i-- {
		char := line[i]
		patchmap[runeToBracket(char)] += score(char)

		for k, v := range patchmap {
			if v < 0 {
				patch = append(patch, bracketToPatch(k))
				patchmap[k] = 0
			}
		}
	}

	return patch
}

func closingChar(char rune) rune {
	switch char {
	case '(':
		return ')'
	case '[':
		return ']'
	case '{':
		return '}'
	case '<':
		return '>'
	}
	panic(fmt.Sprintf("unrecognized char %c", char))
}

func runeToBracket(char rune) Bracket {
	switch char {
	case '(', ')':
		return Round
	case '[', ']':
		return Square
	case '{', '}':
		return Curly
	case '<', '>':
		return Angle
	}
	panic(fmt.Sprintf("unrecognized char %c", char))
}

func bracketToPatch(bracket Bracket) rune {
	switch bracket {
	case Round:
		return ')'
	case Square:
		return ']'
	case Curly:
		return '}'
	case Angle:
		return '>'
	}
	return 1
}

func parse(filename string) [][]rune {
	strslice := parser.ReadFile(filename)
	lines := make([][]rune, 0)
	for _, line := range strslice {
		row := make([]rune, 0)
		for _, c := range line {
			row = append(row, c)
		}
		lines = append(lines, row)
	}
	return lines
}
