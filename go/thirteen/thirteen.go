package thirteen

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/thepatrik/advent-of-code-21/go/pkg/parser"
)

type Pos struct {
	x int
	y int
}

type Fold struct {
	index int
	axis  rune
}

type Paper map[Pos]bool

func NewPaper(pos []Pos) Paper {
	paper := make(Paper)
	for _, p := range pos {
		paper[p] = true
	}
	return paper
}

func (paper Paper) Print() {
	w, h := paper.Dimens()
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			if paper[Pos{x: x, y: y}] {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func (paper Paper) Dimens() (int, int) {
	w := 0
	h := 0
	for pos, dot := range paper {
		if !dot {
			continue
		}
		if pos.x > w {
			w = pos.x
		}
		if pos.y > h {
			h = pos.y
		}
	}
	return w + 1, h + 1
}

func (paper Paper) Count() int {
	sum := 0
	for _, dot := range paper {
		if dot {
			sum++
		}
	}
	return sum
}

func (paper Paper) Fold(fold Fold) {
	for pos, dot := range paper {
		if !dot {
			continue
		}
		if fold.axis == 'x' && pos.x >= fold.index {
			paper[pos] = false
			paper[Pos{x: fold.index*2 - pos.x, y: pos.y}] = true
		} else if fold.axis == 'y' && pos.y >= fold.index {
			paper[pos] = false
			paper[Pos{x: pos.x, y: fold.index*2 - pos.y}] = true
		}
	}
}

func PartOne(filename string) int {
	positions, folds := parse(filename)

	paper := NewPaper(positions)
	paper.Fold(folds[0])
	return paper.Count()
}

func PartTwo(filename string) Paper {
	positions, folds := parse(filename)

	paper := NewPaper(positions)
	for _, fold := range folds {
		paper.Fold(fold)
	}
	return paper
}

func parse(filename string) ([]Pos, []Fold) {
	strslice := parser.ReadFile(filename)
	pos := make([]Pos, 0)
	folds := make([]Fold, 0)
	for _, line := range strslice {
		if line == "" {
			continue
		}

		foldSplitX := strings.Split(line, "fold along x=")
		foldSplitY := strings.Split(line, "fold along y=")
		if len(foldSplitX) > 1 {
			x, _ := strconv.Atoi(foldSplitX[1])
			folds = append(folds, Fold{axis: 'x', index: x})
		} else if len(foldSplitY) > 1 {
			y, _ := strconv.Atoi(foldSplitY[1])
			folds = append(folds, Fold{axis: 'y', index: y})
		} else {
			split := strings.Split(line, ",")
			x, _ := strconv.Atoi(split[0])
			y, _ := strconv.Atoi(split[1])
			pos = append(pos, Pos{x: x, y: y})
		}
	}
	return pos, folds
}
