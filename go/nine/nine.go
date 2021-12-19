package nine

import (
	"sort"

	"github.com/thepatrik/advent-of-code-21/go/pkg/parser"
)

type Pos struct {
	x int
	y int
}

type heightmap [][]int

func (heightmap heightmap) Height(p Pos, boundsVal int) int {
	if p.x == -1 {
		return boundsVal
	}
	if p.x > len(heightmap)-1 {
		return boundsVal
	}
	if p.y == -1 {
		return boundsVal
	}
	if p.y > len(heightmap[p.x])-1 {
		return boundsVal
	}
	return heightmap[p.x][p.y]
}

func PartOne(filename string) int {
	heightmap := parse(filename)
	lows := make([]int, 0)

	for x, row := range heightmap {
		for y, val := range row {
			u := heightmap.Height(Pos{x: x - 1, y: y}, 10)
			r := heightmap.Height(Pos{x: x, y: y + 1}, 10)
			d := heightmap.Height(Pos{x: x + 1, y: y}, 10)
			l := heightmap.Height(Pos{x: x, y: y - 1}, 10)
			if val < u && val < r && val < d && val < l {
				lows = append(lows, val)
			}
		}
	}

	sum := 0
	for _, low := range lows {
		sum += low + 1
	}

	return sum
}

func PartTwo(filename string) int {
	heightmap := parse(filename)
	basins := make([]int, 0)

	for x, row := range heightmap {
		for y, val := range row {
			u := heightmap.Height(Pos{x: x - 1, y: y}, -1)
			r := heightmap.Height(Pos{x: x, y: y + 1}, -1)
			d := heightmap.Height(Pos{x: x + 1, y: y}, -1)
			l := heightmap.Height(Pos{x: x, y: y - 1}, -1)

			if val < u && val < r && val < d && val < l {
				visits := make(map[Pos]int)
				findBasins(heightmap, Pos{x: x, y: y}, visits)
				sum := 0
				for _, v := range visits {
					sum += v
				}
				basins = append(basins, sum)
			}
		}
	}

	sort.Ints(basins)

	return basins[len(basins)-1] * basins[len(basins)-2] * basins[len(basins)-3]
}

func findBasins(heightmap heightmap, p Pos, visits map[Pos]int) {
	current := heightmap.Height(p, -1)
	visits[p] = 1
	fn := func(p Pos) bool {
		_, found := visits[p]
		if found {
			return false
		}

		next := heightmap.Height(p, -1)

		if next != 9 && next-current > 0 {
			visits[p] = 1
			return true
		}

		return false
	}

	positions := make([]Pos, 0)
	positions = append(positions, Pos{x: p.x - 1, y: p.y})
	positions = append(positions, Pos{x: p.x + 1, y: p.y})
	positions = append(positions, Pos{x: p.x, y: p.y - 1})
	positions = append(positions, Pos{x: p.x, y: p.y + 1})

	next := make([]Pos, 0)

	for _, p := range positions {
		if fn(p) {
			next = append(next, p)
		}
	}

	for _, p := range next {
		findBasins(heightmap, p, visits)
	}
}

func parse(filename string) heightmap {
	strslice := parser.ReadFile(filename)
	heightmap := make(heightmap, 0)
	for _, line := range strslice {
		row := make([]int, 0)
		for _, c := range line {
			row = append(row, int(c-'0'))
		}
		heightmap = append(heightmap, row)
	}
	return heightmap
}
