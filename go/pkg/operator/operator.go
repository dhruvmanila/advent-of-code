// Package operator provides a set of generic functions corresponding to the
// intrinsic operators of Golang.
//
// For example, operator.Add(x, y) is equivalent to the expression x + y.
// This is similar to Python's operator module provided in the standard library.
package operator

import "golang.org/x/exp/constraints"

// Numeric is a constraint that permits any numeric type. This includes all
// signed and unsigned integer, floating-point and complex numeric type.
type Numeric interface {
	constraints.Integer | constraints.Float | constraints.Complex
}

type (
	// Func1 is a function which accepts a single parameter of any type and
	// returns a value of the same type.
	Func1[T any] func(x T) T

	// Func2 is a function which accepts two parameters of any type and
	// returns a value of the same type.
	Func2[T any] func(x, y T) T

	// FuncBool is a function which accepts two parameters of any type and
	// returns a boolean value.
	FuncBool[T any] func(x, y T) bool
)

func Add[T Numeric | ~string](x, y T) T   { return x + y }
func Sub[T Numeric](x, y T) T             { return x - y }
func Mul[T Numeric](x, y T) T             { return x * y }
func Div[T Numeric](x, y T) T             { return x / y }
func Mod[T constraints.Integer](x, y T) T { return x % y }
func Pos[T Numeric](x T) T                { return +x }
func Neg[T Numeric](x T) T                { return -x }

func Eq[T comparable](x, y T) bool          { return x == y }
func Lt[T constraints.Ordered](x, y T) bool { return x < y }
func Le[T constraints.Ordered](x, y T) bool { return x <= y }
func Gt[T constraints.Ordered](x, y T) bool { return x > y }
func Ge[T constraints.Ordered](x, y T) bool { return x >= y }

func And[T constraints.Integer](x, y T) T    { return x & y }
func Or[T constraints.Integer](x, y T) T     { return x | y }
func Xor[T constraints.Integer](x, y T) T    { return x ^ y }
func Rshift[T constraints.Integer](x, y T) T { return x >> y }
func Lshift[T constraints.Integer](x, y T) T { return x << y }
