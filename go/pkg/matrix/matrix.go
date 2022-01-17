// Package matrix provides a generic implementation of matrix structure.
package matrix

// Matrix is the basic matrix interface type.
type Matrix[T any] interface {
	// Dims returns the dimensions of a Matrix.
	Dims() (r, c int)

	// At returns the value of a matrix element at row i, column j.
	// It will panic if i or j are out of bounds for the matrix.
	At(i, j int) T

	// Set alters the matrix element at row i, column j to v.
	// It will panic if i or j are out of bounds for the matrix.
	Set(i, j int, v T)

	// T returns the transpose of the Matrix.
	T() Matrix[T]
}

// Dense is a generic dense matrix representation.
type Dense[T any] struct {
	// Rows and Cols are the total number of rows and columns in the matrix.
	Rows int
	Cols int

	// Stride is the number of elements between beginnings of successive
	// array elements. In other words, it tells us how many elements to skip
	// to move to the next position along a certain axis.
	Stride int

	// Data is an array of elements contained in the matrix. The order is from
	// top to bottom, left to right.
	Data []T
}

// NewDense creates a new Dense matrix with r rows and c columns. If data == nil,
// a new slice is allocated for the backing slice. If len(data) == r*c, data is
// used as the backing slice, and changes to the elements of the returned Matrix
// will be reflected in data. If neither of these is true, NewDense will panic.
// NewDense will panic if either r or c is zero.
//
// The data must be arranged in row-major order, i.e. the (i*c + j)-th
// element in the data slice is the {i, j}-th element in the matrix.
func NewDense[T any](r, c int, data []T) *Dense[T] {
	if r <= 0 || c <= 0 {
		if r == 0 || c == 0 {
			panic(ErrZeroLength)
		}
		panic(ErrNegativeDimension)
	}
	if data != nil && r*c != len(data) {
		panic(ErrShape)
	}
	if data == nil {
		data = make([]T, r*c)
	}
	return &Dense[T]{
		Rows:   r,
		Cols:   c,
		Stride: c,
		Data:   data,
	}
}

// Dims returns the number of rows and columns in the matrix.
func (m *Dense[T]) Dims() (r, c int) {
	return m.Rows, m.Cols
}

// At returns the value of a matrix element at row i, column j. It will panic
// if i or j are out of bounds for the matrix.
func (m *Dense[T]) At(i, j int) T {
	m.checkBounds(i, j)
	return m.Data[i*m.Stride+j]
}

// Set sets the element at row i, column j to the value v. It will panic if i
// or j are out of bounds for the matrix.
func (m *Dense[T]) Set(i, j int, v T) {
	m.checkBounds(i, j)
	m.Data[i*m.Stride+j] = v
}

// SetRow sets the values in the specified row i of the matrix to the values
// in src. len(src) must equal the number of columns in the receiver. It will
// panic if i is out of bounds for the matrix. Use `AppendRow` to append a new
// row to the matrix.
func (m *Dense[T]) SetRow(i int, src []T) {
	if len(src) != m.Cols {
		panic(ErrRowLength)
	}
	copy(m.RowView(i), src)
}

// AppendRow appends a new row at the end of the matrix with the values in src.
// If the receiver is empty, as determined by the `IsEmpty()` method, then it
// will be initialized as per the given src. It will panic if len(src) is not
// equal to the number of columns in a non-empty receiver.
func (m *Dense[T]) AppendRow(src []T) {
	switch {
	case m.IsEmpty():
		m.Cols = len(src)
		m.Stride = m.Cols
	case len(src) != m.Cols:
		panic(ErrRowLength)
	}
	m.Rows++
	m.Data = append(m.Data, src...)
}

// RowView returns a slice for the specified row backed by the same array as
// backing the receiver.
func (m *Dense[T]) RowView(i int) []T {
	if i >= m.Rows || i < 0 {
		panic(ErrRowAccess)
	}
	return m.Data[i*m.Stride : i*m.Stride+m.Cols]
}

// SliceRow returns a slice of the specified row `r` from `start` (inclusive)
// upto `stop` (exclusive). The same rule applies for the slice parameters as
// governed by the language except this requires both the start and stop index.
//
// It will panic if `r`, `start`, `stop` is out of bounds with an exception that
// `stop` can be equal to the number of columns.
func (m *Dense[T]) SliceRow(r, start, stop int) []T {
	if r >= m.Rows || r < 0 {
		panic(ErrRowAccess)
	}
	if start < 0 || stop < 0 || start >= m.Cols || stop > m.Cols {
		panic("matrix: out of bound slice")
	}
	return m.Data[r*m.Stride+start : r*m.Stride+stop]
}

// IsEmpty returns whether the receiver is empty.
func (m *Dense[T]) IsEmpty() bool {
	return m.Stride == 0
}

// T performs an implicit transpose by returning the receiver inside a Transpose.
func (m *Dense[T]) T() Matrix[T] {
	return Transpose[T]{Matrix: m}
}

func (m *Dense[T]) checkBounds(i, j int) {
	if i >= m.Rows || i < 0 {
		panic(ErrRowAccess)
	}
	if j >= m.Cols || j < 0 {
		panic(ErrColAccess)
	}
}

// Transpose is a type for performing an implicit matrix transpose. It implements
// the Matrix interface, returning values from the transpose of the matrix within.
type Transpose[T any] struct {
	Matrix Matrix[T]
}

// Dims returns the dimensions of the transposed matrix. The number of rows returned
// is the number of columns in the Matrix field, and the number of columns is
// the number of rows in the Matrix field.
func (t Transpose[T]) Dims() (r, c int) {
	c, r = t.Matrix.Dims()
	return r, c
}

// At returns the value of the element at row i and column j of the transposed
// matrix, that is, row j and column i of the Matrix field. It will panic if
// i or j are out of bounds for the matrix.
func (t Transpose[T]) At(i, j int) T {
	return t.Matrix.At(j, i)
}

// Set sets the element at row i, column j of the transposed matrix to the value
// v, that is, row j and column i of the Matrix field. It will panic if i or j
// are out of bounds for the matrix.
func (t Transpose[T]) Set(i, j int, v T) {
	t.Matrix.Set(j, i, v)
}

// T performs an implicit transpose by returning the Matrix field.
func (t Transpose[T]) T() Matrix[T] {
	return t.Matrix
}
