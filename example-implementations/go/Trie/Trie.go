package trie

type TrieNode struct {
	children map[rune]*TrieNode
	isEnd    bool
}

type Trie struct {
	root *TrieNode
}

func NewTrie() *Trie {
	return &Trie{
		root: &TrieNode{
			children: make(map[rune]*TrieNode),
			isEnd:    false,
		},
	}
}

func (t *Trie) Insert(item string) {
	curr := t.root

	for _, ch := range item {
		if _, exists := curr.children[ch]; !exists {
			curr.children[ch] = &TrieNode{
				children: make(map[rune]*TrieNode),
				isEnd:    false,
			}
		}
		curr = curr.children[ch]
	}

	curr.isEnd = true
}

func (t *Trie) Delete(item string) {
	t.deleteHelper(t.root, item, 0)
}

func (t *Trie) deleteHelper(node *TrieNode, item string, idx int) bool {
	if idx == len(item) {
		if !node.isEnd {
			return false
		}
		node.isEnd = false
		return len(node.children) == 0
	}

	ch := rune(item[idx])
	child, exists := node.children[ch]
	if !exists {
		return false
	}

	shouldDelete := t.deleteHelper(child, item, idx+1)

	if shouldDelete {
		delete(node.children, ch)
		return len(node.children) == 0 && !node.isEnd
	}

	return false
}

func (t *Trie) Find(partial string) []string {
	curr := t.root

	for _, ch := range partial {
		if _, exists := curr.children[ch]; !exists {
			return []string{}
		}
		curr = curr.children[ch]
	}

	result := []string{}
	t.collectWords(curr, partial, &result)
	return result
}

func (t *Trie) collectWords(node *TrieNode, prefix string, result *[]string) {
	if node.isEnd {
		*result = append(*result, prefix)
	}

	for ch, child := range node.children {
		t.collectWords(child, prefix+string(ch), result)
	}
}
