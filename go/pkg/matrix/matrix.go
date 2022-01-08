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
		Rows: r,
		Cols: c,
		Data: data,
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
	return m.Data[i*m.Cols+j]
}

// Set sets the element at row i, column j to the value v. It will panic if i
// or j are out of bounds for the matrix.
func (m *Dense) Set(i, j int, v interface{}) {
	m.checkBounds(i, j)
	m.Data[i*m.Cols+j] = v
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
	copy(m.Data[i*m.Cols:i*m.Cols+m.Cols], src)
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
