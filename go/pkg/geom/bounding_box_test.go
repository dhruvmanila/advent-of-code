package geom

import (
	"reflect"
	"testing"
)

func TestBoundingBox2DContains(t *testing.T) {
	testCases := []struct {
		name     string
		x, y     int
		expected bool
	}{
		{name: "outside point", x: -2, y: 3, expected: false},
		{name: "corner point", x: 0, y: 0, expected: true},
		{name: "edge point", x: 0, y: 2, expected: true},
		{name: "inside point", x: 1, y: 2, expected: true},
	}

	bbox := NewBoundingBox2D(0, 5, 0, 5)
	for _, c := range testCases {
		t.Run(c.name, func(t *testing.T) {
			if actual := bbox.Contains(c.x, c.y); actual != c.expected {
				t.Errorf("\nbbox: %#v\nexpected: %v\nactual: %v\n", bbox, c.expected, actual)
			}
		})
	}
}

func TestBoundingBox2DIntersection(t *testing.T) {
	testCases := []struct {
		name     string
		other    *BoundingBox2D
		expected *BoundingBox2D
	}{
		{
			name:     "does not intersect #1",
			other:    NewBoundingBox2D(0, 4, 2, 8),
			expected: nil,
		},
		{
			name:     "does not intersect #2",
			other:    NewBoundingBox2D(10, 18, 18, 22),
			expected: nil,
		},
		{
			name:     "intersect at bottom left corner",
			other:    NewBoundingBox2D(3, 9, 4, 10),
			expected: NewBoundingBox2D(5, 9, 5, 10),
		},
		{
			name:     "intersect at top right corner",
			other:    NewBoundingBox2D(11, 18, 14, 20),
			expected: NewBoundingBox2D(11, 15, 14, 15),
		},
		{
			name:     "intersect at left edge",
			other:    NewBoundingBox2D(2, 10, 5, 15),
			expected: NewBoundingBox2D(5, 10, 5, 15),
		},
		{
			name:     "intersect at bottom edge",
			other:    NewBoundingBox2D(5, 15, 3, 9),
			expected: NewBoundingBox2D(5, 15, 5, 9),
		},
		{
			name:     "fully contained within",
			other:    NewBoundingBox2D(8, 12, 6, 9),
			expected: NewBoundingBox2D(8, 12, 6, 9),
		},
	}

	bbox := NewBoundingBox2D(5, 15, 5, 15)
	for _, c := range testCases {
		t.Run(c.name, func(t *testing.T) {
			actual := bbox.Intersection(c.other)
			if !reflect.DeepEqual(actual, c.expected) {
				t.Errorf("\nbbox: %#v\nother: %#v\nexpected: %v\nactual: %v\n", bbox, c.other, c.expected, actual)
			}
		})
	}
}

func TestBoundingBox2DArea(t *testing.T) {
	bbox := NewBoundingBox2D(1, 4, 2, 7)
	area, expected := bbox.Area(), 24
	if area != expected {
		t.Errorf("\nbbox: %#v\nexpected: %v\nactual: %v\n", bbox, expected, area)
	}
}

func TestBoundingBox3DContains(t *testing.T) {
	testCases := []struct {
		name     string
		x, y, z  int
		expected bool
	}{
		{name: "outside point", x: 2, y: 3, z: 8, expected: false},
		{name: "corner point", x: 0, y: 0, z: 0, expected: true},
		{name: "edge point", x: 0, y: 2, z: 0, expected: true},
		{name: "inside point", x: 1, y: 2, z: 3, expected: true},
	}

	bbox := NewBoundingBox3D(0, 5, 0, 5, 0, 5)
	for _, c := range testCases {
		t.Run(c.name, func(t *testing.T) {
			if actual := bbox.Contains(c.x, c.y, c.z); actual != c.expected {
				t.Errorf("\nbbox: %#v\nexpected: %v\nactual: %v\n", bbox, c.expected, actual)
			}
		})
	}
}

func TestBoundingBox3DVolume(t *testing.T) {
	bbox := NewBoundingBox3D(1, 4, 2, 7, 3, 5)
	volume, expected := bbox.Volume(), 72
	if volume != expected {
		t.Errorf("\nbbox: %#v\nexpected: %v\nactual: %v\n", bbox, expected, volume)
	}
}
