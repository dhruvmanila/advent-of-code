package matrix

import "errors"

var (
	ErrZeroLength        = errors.New("matrix: zero length in matrix dimension")
	ErrNegativeDimension = errors.New("matrix: negative dimension")
	ErrShape             = errors.New("matrix: dimension mismatch")
	ErrRowAccess         = errors.New("matrix: row index out of range")
	ErrColAccess         = errors.New("matrix: column index out of range")
	ErrRowLength         = errors.New("matrix: row length mismatch")
	ErrColLength         = errors.New("matrix: column length mismatch")
	ErrVectorLength      = errors.New("matrix: vector length mismatch")
)
