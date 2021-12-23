package matrix

type Matrix [][]int

var (
	adjacents = []Pos{
		{X: 1, Y: 1},
		{X: 1, Y: 0},
		{X: 1, Y: -1},
		{X: 0, Y: -1},
		{X: 0, Y: 1},
		{X: -1, Y: 1},
		{X: -1, Y: 0},
		{X: -1, Y: -1},
	}
)

func NewMatrix(size int) Matrix {
	return make(Matrix, size)
}

func (matrix Matrix) Dimens() (int, int) {
	return len(matrix[0]), len(matrix)
}

func (matrix Matrix) Neighbours(pos Pos, includeDiagonals bool) []Pos {
	w, h := matrix.Dimens()
	n := make([]Pos, 0)

	for _, adjacent := range adjacents {
		if !includeDiagonals && isDiagonal(adjacent) {
			continue
		}
		neighbour := Pos{X: pos.X + adjacent.X, Y: pos.Y + adjacent.Y}
		if neighbour.X >= 0 && neighbour.X < w && neighbour.Y >= 0 && neighbour.Y < h {
			n = append(n, neighbour)
		}
	}

	return n
}

func isDiagonal(pos Pos) bool {
	if pos.X == 1 && pos.Y == 1 {
		return true
	}
	if pos.X == 1 && pos.Y == -1 {
		return true
	}
	if pos.X == -1 && pos.Y == 1 {
		return true
	}
	if pos.X == -1 && pos.Y == -1 {
		return true
	}
	return false
}
