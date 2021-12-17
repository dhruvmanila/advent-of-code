package util

import (
	"reflect"
	"testing"
)

func TestCardinalDirection(t *testing.T) {
	testCases := []struct {
		name             string
		y, x, rows, cols int
		expected         [][]int
	}{
		{
			name:     "upper left corner 1x1 matrix",
			y:        0,
			x:        0,
			rows:     1,
			cols:     1,
			expected: [][]int{},
		},
		{
			name: "upper left corner 2x2 matrix",
			y:    0,
			x:    0,
			rows: 2,
			cols: 2,
			expected: [][]int{
				{0, 1},
				{1, 0},
			},
		},
		{
			name: "upper right corner 2x2 matrix",
			y:    0,
			x:    1,
			rows: 2,
			cols: 2,
			expected: [][]int{
				{1, 1},
				{0, 0},
			},
		},
		{
			name: "down left corner 2x2 matrix",
			y:    1,
			x:    0,
			rows: 2,
			cols: 2,
			expected: [][]int{
				{0, 0},
				{1, 1},
			},
		},
		{
			name: "down left corner 2x2 matrix",
			y:    1,
			x:    1,
			rows: 2,
			cols: 2,
			expected: [][]int{
				{0, 1},
				{1, 0},
			},
		},
		{
			name: "left edge 3x3 matrix",
			y:    1,
			x:    0,
			rows: 3,
			cols: 3,
			expected: [][]int{
				{0, 0},
				{1, 1},
				{2, 0},
			},
		},
		{
			name: "top edge 3x3 matrix",
			y:    0,
			x:    1,
			rows: 3,
			cols: 3,
			expected: [][]int{
				{0, 2},
				{1, 1},
				{0, 0},
			},
		},
		{
			name: "right edge 3x3 matrix",
			y:    1,
			x:    2,
			rows: 3,
			cols: 3,
			expected: [][]int{
				{0, 2},
				{2, 2},
				{1, 1},
			},
		},
		{
			name: "left edge 3x3 matrix",
			y:    2,
			x:    1,
			rows: 3,
			cols: 3,
			expected: [][]int{
				{1, 1},
				{2, 2},
				{2, 0},
			},
		},
		{
			name: "center 3x3 matrix",
			y:    1,
			x:    1,
			rows: 3,
			cols: 3,
			expected: [][]int{
				{0, 1},
				{1, 2},
				{2, 1},
				{1, 0},
			},
		},
	}

	for _, c := range testCases {
		t.Run(c.name, func(t *testing.T) {
			result := CardinalDirection(c.y, c.x, c.rows, c.cols)
			if !reflect.DeepEqual(result, c.expected) {
				t.Errorf("\nExpected: %#v\nGot: %#v\n", c.expected, result)
			}
		})
	}
}
