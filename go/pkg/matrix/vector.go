package matrix

type VecDense[T any] struct {
	N    int
	Inc  int
	Data []T
}

// TODO
