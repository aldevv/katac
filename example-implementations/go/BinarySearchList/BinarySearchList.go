package binarysearchlist

func BinarySearch(haystack []int, needle int) bool {
	lo := 0
	hi := len(haystack)

	for lo < hi {
		mid := lo + (hi-lo)/2
		val := haystack[mid]

		if val == needle {
			return true
		} else if val < needle {
			lo = mid + 1
		} else {
			hi = mid
		}
	}

	return false
}
