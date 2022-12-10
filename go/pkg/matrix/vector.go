package matrix

// VecDense represents a column vector.
type VecDense[T any] struct {
	N    int
	Inc  int
	Data []T
}

// NewVecDense creates a new VecDense of length n. If data == nil,
// a new slice is allocated for the backing slice. If len(data) == n, data is
// used as the backing slice, and changes to the elements of the returned VecDense
// will be reflected in data. If neither of these is true, NewVecDense will panic.
// NewVecDense will panic if n is zero.
func NewVecDense[T any](n int, data []T) *VecDense[T] {
	if n <= 0 {
		if n == 0 {
			panic(ErrZeroLength)
		}
		panic(ErrNegativeDimension)
	}
	if data != nil && len(data) != n {
		panic(ErrShape)
	}
	if data == nil {
		data = make([]T, n)
	}
	return &VecDense[T]{
		N:    n,
		Inc:  1,
		Data: data,
	}
}

// At returns the element at row i.
// It panics if i is out of bounds.
func (v *VecDense[T]) At(i int) T {
	if i >= v.N {
		panic(ErrRowAccess)
	}
	return v.Data[i*v.Inc]
}

// Set sets the element at row i to the value val.
// It panics if i is out of bounds.
func (v *VecDense[T]) Set(i int, val T) {
	if i >= v.N {
		panic(ErrRowAccess)
	}
	v.Data[i*v.Inc] = val
}

// Dims returns the number of rows and columns in the matrix.
// Columns is always 1 for the vector.
func (v *VecDense[T]) Dims() (r, c int) {
	if v.IsEmpty() {
		return 0, 0
	}
	return v.N, 1
}

// Len returns the length of the vector.
func (v *VecDense[T]) Len() int {
	return v.N
}

// IsEmpty returns whether the receiver is empty.
func (v *VecDense[T]) IsEmpty() bool {
	return v.Inc == 0
}

// RowViewOf reflects the row i of the Matrix m, into the receiver
// backed by the same underlying data. The receiver must either be
// empty or have length equal to the number of columns of m.
func (v *VecDense[T]) RowViewOf(m *Dense[T], i int) {
	if i >= m.Rows || i < 0 {
		panic(ErrRowAccess)
	}
	if !v.IsEmpty() && v.N != m.Cols {
		panic(ErrShape)
	}

	v.Inc = 1
	v.Data = m.Data[i*m.Stride : i*m.Stride+m.Cols]
	v.N = m.Cols
}

// ColViewOf reflects the column j of the Matrix m, into the receiver
// backed by the same underlying data. The receiver must either be
// empty or have length equal to the number of rows of m.
func (v *VecDense[T]) ColViewOf(m *Dense[T], j int) {
	if j >= m.Cols || j < 0 {
		panic(ErrColAccess)
	}
	if !v.IsEmpty() && v.N != m.Rows {
		panic(ErrShape)
	}

	v.Inc = m.Stride
	v.Data = m.Data[j : j+(m.Rows-1)*m.Stride+1]
	v.N = m.Rows
}

// Copy copies the elements of x into the elements of y:
//
//	y[i] = x[i] for all i.
//
// Copy will panic if the lengths of x and y do not match.
func vectorCopy[T any](x, y VecDense[T]) {
	if x.N != y.N {
		panic(ErrVectorLength)
	}
	vectorCopyImpl(x.N, x.Data, x.Inc, y.Data, y.Inc)
}

func vectorCopyImpl[T any](n int, x []T, incX int, y []T, incY int) {
	if incX == 0 {
		panic("matrix: zero x index increment")
	}
	if incY == 0 {
		panic("matrix: zero y index increment")
	}
	if n < 1 {
		if n == 0 {
			return
		}
		panic("matrix: n < 0")
	}
	if (incX > 0 && len(x) <= (n-1)*incX) || (incX < 0 && len(x) <= (1-n)*incX) {
		panic("matrix: insufficient length of x")
	}
	if (incY > 0 && len(y) <= (n-1)*incY) || (incY < 0 && len(y) <= (1-n)*incY) {
		panic("matrix: insufficient length of y")
	}
	if incX == 1 && incY == 1 {
		copy(y[:n], x[:n])
		return
	}
	var ix, iy int
	if incX < 0 {
		ix = (-n + 1) * incX
	}
	if incY < 0 {
		iy = (-n + 1) * incY
	}
	for i := 0; i < n; i++ {
		y[iy] = x[ix]
		ix += incX
		iy += incY
	}
}
