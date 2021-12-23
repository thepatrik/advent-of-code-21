package graph

type Edge struct {
	Node   string
	Weight int
}

type Graph struct {
	Nodes map[string][]Edge
}

func NewGraph() *Graph {
	nodes := make(map[string][]Edge)
	return &Graph{Nodes: nodes}
}

func (g *Graph) AddEdge(origin, destiny string, weight int) {
	g.Nodes[origin] = append(g.Nodes[origin], Edge{Node: destiny, Weight: weight})
	g.Nodes[destiny] = append(g.Nodes[destiny], Edge{Node: origin, Weight: weight})
}

func (g *Graph) Edges(node string) []Edge {
	return g.Nodes[node]
}

func (g *Graph) Path(origin, destiny string) (int, []string) {
	h := NewHeap()
	h.push(Path{value: 0, nodes: []string{origin}})
	visits := make(map[string]bool)

	for len(*h.values) > 0 {
		p := h.pop()
		node := p.nodes[len(p.nodes)-1]

		if visits[node] {
			continue
		}

		if node == destiny {
			return p.value, p.nodes
		}

		for _, e := range g.Edges(node) {
			if !visits[e.Node] {
				h.push(Path{value: p.value + e.Weight, nodes: append([]string{}, append(p.nodes, e.Node)...)})
			}
		}

		visits[node] = true
	}

	return 0, nil
}
