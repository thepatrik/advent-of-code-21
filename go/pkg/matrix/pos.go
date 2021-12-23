package matrix

import (
	"strconv"
	"strings"
)

type Pos struct {
	X, Y int
}

func (p Pos) String() string {
	return strconv.Itoa(p.X) + "," + strconv.Itoa(p.Y)
}

func NewPosFromString(s string) (*Pos, error) {
	split := strings.Split(s, ",")
	x, err := strconv.Atoi(split[0])
	if err != nil {
		return nil, err
	}

	y, err := strconv.Atoi(split[1])
	if err != nil {
		return nil, err
	}

	return &Pos{X: x, Y: y}, nil
}
