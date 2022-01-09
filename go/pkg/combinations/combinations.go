package combinations

// AllInt returns all combinations for a given integer array.
//
// This is essentially a powerset of the given set except that the empty set is
// not considered.
func AllInt(set []int) [][]int {
	length := uint(len(set))
	possible := (1 << length) - 1
	subsets := make([][]int, 0, possible)

	// Go through all possible combinations of objects from 1 (only first
	// object in subset) to 2^length (all objects in subset).
	for subsetBits := 1; subsetBits <= possible; subsetBits++ {
		var subset []int
		for object := uint(0); object < length; object++ {
			// Checks if object is contained in subset by checking if bit
			// 'object' is set in subsetBits.
			if (subsetBits>>object)&1 == 1 {
				subset = append(subset, set[object])
			}
		}
		subsets = append(subsets, subset)
	}
	return subsets
}
