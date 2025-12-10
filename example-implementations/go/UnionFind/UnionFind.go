package unionfind

type UnionFind struct {
	count  int
	parent []int
	rank   []int
}

func (uf *UnionFind) Count() int { return uf.count }

func NewUnionFind(size int) *UnionFind {
	parent := make([]int, size)
	rank := make([]int, size)

	for i := 0; i < size; i++ {
		parent[i] = i
		rank[i] = 0
	}

	return &UnionFind{
		count:  size,
		parent: parent,
		rank:   rank,
	}
}

func (uf *UnionFind) Find(p int) int {
	if uf.parent[p] != p {
		uf.parent[p] = uf.Find(uf.parent[p])
	}
	return uf.parent[p]
}

func (uf *UnionFind) Union(p int, q int) {
	rootP := uf.Find(p)
	rootQ := uf.Find(q)

	if rootP == rootQ {
		return
	}

	if uf.rank[rootP] < uf.rank[rootQ] {
		uf.parent[rootP] = rootQ
	} else if uf.rank[rootP] > uf.rank[rootQ] {
		uf.parent[rootQ] = rootP
	} else {
		uf.parent[rootQ] = rootP
		uf.rank[rootP]++
	}

	uf.count--
}

func (uf *UnionFind) Connected(p int, q int) bool {
	return uf.Find(p) == uf.Find(q)
}
