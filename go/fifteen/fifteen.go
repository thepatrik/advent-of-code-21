package fifteen

import (
	"github.com/thepatrik/advent-of-code-21/go/pkg/graph"
	"github.com/thepatrik/advent-of-code-21/go/pkg/matrix"
	"github.com/thepatrik/advent-of-code-21/go/pkg/parser"
)

func PartOne(filename string) int {
	matrix := parse(filename)
	return calculatePath(matrix)
}

func PartTwo(filename string) int {
	original := parse(filename)
	matrix := growMatrix(original, 5)
	return calculatePath(matrix)
}

func calculatePath(m matrix.Matrix) int {
	w, h := m.Dimens()
	graph := graph.NewGraph()

	for y := range m {
		for x, originWeight := range m[y] {
			origin := matrix.Pos{X: x, Y: y}

			for _, neighbour := range m.Neighbours(origin, false) {
				weight := originWeight + m[neighbour.Y][neighbour.X]
				graph.AddEdge(origin.String(), neighbour.String(), weight)
			}
		}
	}

	topLeft := matrix.Pos{X: 0, Y: 0}
	bottomRight := matrix.Pos{X: w - 1, Y: h - 1}
	_, nodes := graph.Path(topLeft.String(), bottomRight.String())

	sum := 0
	for i := 1; i < len(nodes); i++ {
		node := nodes[i]
		pos, _ := matrix.NewPosFromString(node)
		sum += m[pos.Y][pos.X]
	}

	return sum
}

func growMatrix(original matrix.Matrix, size int) [][]int {
	w, h := original.Dimens()
	m := make([][]int, size*w)

	for i, line := range original {
		for x := 0; x < size; x++ {
			m[i+(w*x)] = make([]int, 5*len(line))
		}

		for j, number := range line {
			for x := 0; x < size; x++ {
				for y := 0; y < size; y++ {
					n := number + x + y
					if n > 9 {
						n -= 9
					}

					m[i+(w*x)][j+(h*y)] = n
				}
			}
		}
	}

	return m
}

func parse(filename string) [][]int {
	strslice := parser.ReadFile(filename)
	m := make([][]int, 0)
	for _, line := range strslice {
		row := make([]int, 0)
		for _, c := range line {
			row = append(row, int(c-'0'))
		}
		m = append(m, row)
	}
	return m
}
