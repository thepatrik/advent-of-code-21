package eleven

import "github.com/thepatrik/advent-of-code-21/go/pkg/parser"

var (
	adjacents = []Pos{
		{x: 0, y: -1},
		{x: 0, y: 1},
		{x: 1, y: 1},
		{x: 1, y: 0},
		{x: 1, y: -1},
		{x: -1, y: 1},
		{x: -1, y: 0},
		{x: -1, y: -1},
	}
)

type Pos struct {
	x int
	y int
}

type octopi [][]int

func (octopi octopi) Neighbours(p Pos) []Pos {
	neighbours := make([]Pos, 0)
	w, h := len(octopi), len(octopi[p.x])
	for _, adjacent := range adjacents {
		neighbour := Pos{x: p.x + adjacent.x, y: p.y + adjacent.y}
		if neighbour.x >= 0 && neighbour.x < w && neighbour.y >= 0 && neighbour.y < h {
			neighbours = append(neighbours, neighbour)
		}
	}
	return neighbours
}

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

func step(octopi octopi) map[Pos]bool {
	flashmap := make(map[Pos]bool)

	for y := 0; y < len(octopi); y++ {
		for x := 0; x < len(octopi[y]); x++ {
			pos := Pos{x: x, y: y}
			if !flashmap[pos] {
				octopi[pos.x][pos.y] += 1
				if octopi[pos.x][pos.y] == 10 {
					flash(octopi, pos, flashmap)
				}
			}
		}
	}

	return flashmap
}

func flash(octopi octopi, pos Pos, flashmap map[Pos]bool) {
	flashmap[pos] = true
	octopi[pos.x][pos.y] = 0

	for _, neighbour := range octopi.Neighbours(pos) {
		if !flashmap[neighbour] {
			octopi[neighbour.x][neighbour.y] += 1
			if octopi[neighbour.x][neighbour.y] == 10 {
				flash(octopi, neighbour, flashmap)
			}
		}
	}
}

func parse(filename string) octopi {
	strslice := parser.ReadFile(filename)
	octopi := make(octopi, 0)
	for _, line := range strslice {
		row := make([]int, 0)
		for _, c := range line {
			row = append(row, int(c-'0'))
		}
		octopi = append(octopi, row)
	}
	return octopi
}
