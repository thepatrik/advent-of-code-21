// Kudos to https://github.com/douglasmakey/dijkstra-heap
package graph

import hp "container/heap"

type Path struct {
	value int
	nodes []string
}

type paths []Path

func (p paths) Len() int {
	return len(p)
}

func (p paths) Less(i, j int) bool {
	return p[i].value < p[j].value
}

func (p paths) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p *paths) Push(x interface{}) {
	*p = append(*p, x.(Path))
}

func (p *paths) Pop() interface{} {
	old := *p
	n := len(old)
	x := old[n-1]
	*p = old[0 : n-1]
	return x
}

type Heap struct {
	values *paths
}

func NewHeap() *Heap {
	return &Heap{values: &paths{}}
}

func (h *Heap) push(p Path) {
	hp.Push(h.values, p)
}

func (h *Heap) pop() Path {
	i := hp.Pop(h.values)
	return i.(Path)
}
