package ocr

import "github.com/MakeNowJust/heredoc"

var alphabet6 = map[string]string{
	heredoc.Doc(`
		.##.
		#..#
		#..#
		####
		#..#
		#..#`): "A",

	heredoc.Doc(`
		###.
		#..#
		###.
		#..#
		#..#
		###.`): "B",

	heredoc.Doc(`
		.##.
		#..#
		#...
		#...
		#..#
		.##.`): "C",

	heredoc.Doc(`
		####
		#...
		###.
		#...
		#...
		####`): "E",

	heredoc.Doc(`
		####
		#...
		###.
		#...
		#...
		#...`): "F",

	heredoc.Doc(`
		.##.
		#..#
		#...
		#.##
		#..#
		.###`): "G",

	heredoc.Doc(`
		#..#
		#..#
		####
		#..#
		#..#
		#..#`): "H",

	heredoc.Doc(`
		.###
		..#.
		..#.
		..#.
		..#.
		.###`): "I",

	heredoc.Doc(`
		..##
		...#
		...#
		...#
		#..#
		.##.`): "J",

	heredoc.Doc(`
		#..#
		#.#.
		##..
		#.#.
		#.#.
		#..#`): "K",

	heredoc.Doc(`
		#...
		#...
		#...
		#...
		#...
		####`): "L",

	heredoc.Doc(`
		.##.
		#..#
		#..#
		#..#
		#..#
		.##.`): "O",

	heredoc.Doc(`
		###.
		#..#
		#..#
		###.
		#...
		#...`): "P",

	heredoc.Doc(`
		###.
		#..#
		#..#
		###.
		#.#.
		#..#`): "R",

	heredoc.Doc(`
		.###
		#...
		#...
		.##.
		...#
		###.`): "S",

	heredoc.Doc(`
		#..#
		#..#
		#..#
		#..#
		#..#
		.##.`): "U",

	heredoc.Doc(`
		#...
		#...
		.#.#
		..#.
		..#.
		..#.`): "Y",

	heredoc.Doc(`
		####
		...#
		..#.
		.#..
		#...
		####`): "Z",
}
