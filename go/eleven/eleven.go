package eleven

import (
	"github.com/thepatrik/advent-of-code-21/go/pkg/matrix"
	"github.com/thepatrik/advent-of-code-21/go/pkg/parser"
)

func PartOne(filename string) int {
	octopi := parse(filename)
	flashes := 0

	for i := 0; i < 100; i++ {
		flashmap := step(octopi)
		flashes += len(flashmap)
	}

	return flashes
}

func PartTwo(filename string) int {
	octopi := parse(filename)
	max := len(octopi) * len(octopi[0])

	for i := 1; ; i++ {
		flashmap := step(octopi)
		if len(flashmap) == max {
			return i
		}
	}
}

func step(octopi matrix.Matrix) map[matrix.Pos]bool {
	flashmap := make(map[matrix.Pos]bool)

	for y := 0; y < len(octopi); y++ {
		for x := 0; x < len(octopi[y]); x++ {
			pos := matrix.Pos{X: x, Y: y}
			if !flashmap[pos] {
				octopi[pos.X][pos.Y] += 1
				if octopi[pos.X][pos.Y] == 10 {
					flash(octopi, pos, flashmap)
				}
			}
		}
	}

	return flashmap
}

func flash(octopi matrix.Matrix, pos matrix.Pos, flashmap map[matrix.Pos]bool) {
	flashmap[pos] = true
	octopi[pos.X][pos.Y] = 0

	for _, neighbour := range octopi.Neighbours(pos, true) {
		if !flashmap[neighbour] {
			octopi[neighbour.X][neighbour.Y] += 1
			if octopi[neighbour.X][neighbour.Y] == 10 {
				flash(octopi, neighbour, flashmap)
			}
		}
	}
}

func parse(filename string) matrix.Matrix {
	strslice := parser.ReadFile(filename)
	octopi := make(matrix.Matrix, 0)
	for _, line := range strslice {
		row := make([]int, 0)
		for _, c := range line {
			row = append(row, int(c-'0'))
		}
		octopi = append(octopi, row)
	}
	return octopi
}
