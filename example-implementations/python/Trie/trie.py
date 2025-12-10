"""
Trie (Prefix Tree) implementation.
Used for efficient string prefix searches and autocomplete.
"""

from typing import Dict, List, Optional


class TrieNode:
    """Node in the trie."""

    def __init__(self):
        self.children: Dict[str, 'TrieNode'] = {}
        self.is_end = False


class Trie:
    """Trie data structure for string storage and prefix search."""

    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        """Insert a word into the trie."""
        curr = self.root

        for ch in word:
            if ch not in curr.children:
                curr.children[ch] = TrieNode()
            curr = curr.children[ch]

        curr.is_end = True

    def delete(self, word: str) -> None:
        """Delete a word from the trie."""
        self._delete_helper(self.root, word, 0)

    def _delete_helper(self, node: TrieNode, word: str, idx: int) -> bool:
        """Helper function to recursively delete a word."""
        if idx == len(word):
            if not node.is_end:
                return False
            node.is_end = False
            return len(node.children) == 0

        ch = word[idx]
        if ch not in node.children:
            return False

        child = node.children[ch]
        should_delete = self._delete_helper(child, word, idx + 1)

        if should_delete:
            del node.children[ch]
            return len(node.children) == 0 and not node.is_end

        return False

    def find(self, prefix: str) -> List[str]:
        """Find all words with the given prefix."""
        curr = self.root

        # Navigate to the prefix node
        for ch in prefix:
            if ch not in curr.children:
                return []
            curr = curr.children[ch]

        # Collect all words with this prefix
        result: List[str] = []
        self._collect_words(curr, prefix, result)
        return result

    def _collect_words(self, node: TrieNode, prefix: str, result: List[str]) -> None:
        """Recursively collect all words from a given node."""
        if node.is_end:
            result.append(prefix)

        for ch, child in node.children.items():
            self._collect_words(child, prefix + ch, result)
