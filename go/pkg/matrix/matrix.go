// Package matrix provides a generic implementation of matrix structure.
//
// This is similar to https://github.com/gonum/gonum with minimal API,
// new methods and generic implementation.
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
