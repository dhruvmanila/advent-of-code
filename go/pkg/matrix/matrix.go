// Package matrix provides a generic implementation of matrix structure.
package matrix

// Matrix is the basic matrix interface type.
type Matrix interface {
	// Dims returns the dimensions of a Matrix.
	Dims() (r, c int)

	// At returns the value of a matrix element at row i, column j.
	// It will panic if i or j are out of bounds for the matrix.
	At(i, j int) interface{}

	// Set alters the matrix element at row i, column j to v.
	// It will panic if i or j are out of bounds for the matrix.
	Set(i, j int, v interface{})

	// T returns the transpose of the Matrix.
	T() Matrix
}

// Dense is a generic dense matrix representation.
type Dense struct {
	// Rows and Cols are the total number of rows and columns in the matrix.
	Rows int
	Cols int

	// Stride is the number of elements between beginnings of successive
	// array elements. In other words, it tells us how many elements to skip
	// to move to the next position along a certain axis.
	Stride int

	// Data is an array of elements contained in the matrix. The order is from
	// top to bottom, left to right.
	Data []interface{}
}

// NewDense creates a new Dense matrix with r rows and c columns. If data == nil,
// a new slice is allocated for the backing slice. If len(data) == r*c, data is
// used as the backing slice, and changes to the elements of the returned Matrix
// will be reflected in data. If neither of these is true, NewDense will panic.
// NewDense will panic if either r or c is zero.
//
// The data must be arranged in row-major order, i.e. the (i*c + j)-th
// element in the data slice is the {i, j}-th element in the matrix.
func NewDense(r, c int, data []interface{}) *Dense {
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
		data = make([]interface{}, r*c)
	}
	return &Dense{
		Rows:   r,
		Cols:   c,
		Stride: c,
		Data:   data,
	}
}

// Dims returns the number of rows and columns in the matrix.
func (m *Dense) Dims() (r, c int) {
	return m.Rows, m.Cols
}

// At returns the value of a matrix element at row i, column j. It will panic
// if i or j are out of bounds for the matrix.
func (m *Dense) At(i, j int) interface{} {
	m.checkBounds(i, j)
	return m.Data[i*m.Stride+j]
}

// Set sets the element at row i, column j to the value v. It will panic if i
// or j are out of bounds for the matrix.
func (m *Dense) Set(i, j int, v interface{}) {
	m.checkBounds(i, j)
	m.Data[i*m.Stride+j] = v
}

// SetRow sets the values in the specified row i of the matrix to the values
// in src. len(src) must equal the number of columns in the receiver. It will
// panic if i is out of bounds for the matrix.
func (m *Dense) SetRow(i int, src []interface{}) {
	if i >= m.Rows || i < 0 {
		panic(ErrRowAccess)
	}
	if len(src) != m.Cols {
		panic(ErrRowLength)
	}
	copy(m.Data[i*m.Stride:i*m.Stride+m.Cols], src)
}

// AppendRow appends a new row at the end of the matrix with the values in src.
// If the receiver is not empty, then it will panic if len(src) is not equal to
// the number of columns in the receiver.
func (m *Dense) AppendRow(src []interface{}) {
	m.Rows++
	switch {
	case m.IsEmpty():
		m.Cols = len(src)
		m.Stride = m.Cols
	case len(src) != m.Cols:
		panic(ErrRowLength)
	}
	m.Data = append(m.Data, src...)
}

// RowView returns a slice for the specified row backed by the same array as
// backing the receiver.
func (m *Dense) RowView(i int) interface{} {
	if i >= m.Rows || i < 0 {
		panic(ErrRowAccess)
	}
	return m.Data[i*m.Stride : i*m.Stride+m.Cols]
}

// SliceRow returns a slice of the specified row `r` starting at `from`
// (inclusive) upto `to` (exclusive). The same rule applies for the slice
// parameters as governed by the language except this requires both the start
// and stop index.
//
// It will panic if `r`, `from`, `to` is out of bounds with an exception that
// `to` can equal to the number of columns.
func (m *Dense) SliceRow(r, from, to int) interface{} {
	if r >= m.Rows || r < 0 {
		panic(ErrRowAccess)
	}
	if from < 0 || to < 0 || from >= m.Cols || to > m.Cols {
		panic("matrix: out of bound slice")
	}
	return m.Data[r*m.Stride+from : r*m.Stride+to]
}

// IsEmpty returns whether the receiver is empty.
func (m *Dense) IsEmpty() bool {
	return m.Stride == 0
}

// T performs an implicit transpose by returning the receiver inside a Transpose.
func (m *Dense) T() Matrix {
	return Transpose{Matrix: m}
}

func (m *Dense) checkBounds(i, j int) {
	if i >= m.Rows || i < 0 {
		panic(ErrRowAccess)
	}
	if j >= m.Cols || j < 0 {
		panic(ErrColAccess)
	}
}

// Transpose is a type for performing an implicit matrix transpose. It implements
// the Matrix interface, returning values from the transpose of the matrix within.
type Transpose struct {
	Matrix Matrix
}

// Dims returns the dimensions of the transposed matrix. The number of rows returned
// is the number of columns in the Matrix field, and the number of columns is
// the number of rows in the Matrix field.
func (t Transpose) Dims() (r, c int) {
	c, r = t.Matrix.Dims()
	return r, c
}

// At returns the value of the element at row i and column j of the transposed
// matrix, that is, row j and column i of the Matrix field. It will panic if
// i or j are out of bounds for the matrix.
func (t Transpose) At(i, j int) interface{} {
	return t.Matrix.At(j, i)
}

// Set sets the element at row i, column j of the transposed matrix to the value
// v, that is, row j and column i of the Matrix field. It will panic if i or j
// are out of bounds for the matrix.
func (t Transpose) Set(i, j int, v interface{}) {
	t.Matrix.Set(j, i, v)
}

// T performs an implicit transpose by returning the Matrix field.
func (t Transpose) T() Matrix {
	return t.Matrix
}
