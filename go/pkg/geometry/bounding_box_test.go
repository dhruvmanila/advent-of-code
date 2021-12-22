package geometry

import "testing"

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
