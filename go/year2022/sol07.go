package year2022

import (
	"fmt"
	"path/filepath"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

type fileMode uint8

const (
	modeDir fileMode = 1 << iota
	modeFile
)

const (
	partEdge   = "├── "
	partLine   = "│   "
	partCorner = "└── "
	partBlank  = "    "
)

const (
	commandPrefix       = '$'
	totalDiskSpace uint = 70000000
	requiredSpace  uint = 30000000
)

// fileSystem represents an entire file system.
type fileSystem struct {
	// root is the root node of the file system. This is usually
	// the root directory ("/").
	root *fsNode
}

// fsNode is a node of the file system which either represents a file
// or a directory. The node is aware of it's parent and children.
type fsNode struct {
	Type fileMode
	Path string
	Name string
	Size uint

	parent   *fsNode
	children []*fsNode
}

func (fs *fileSystem) dirSize() map[string]uint {
	m := make(map[string]uint)
	var computeSize func(node *fsNode) uint

	computeSize = func(node *fsNode) uint {
		if node.Type == modeFile {
			return node.Size
		}
		var size uint = 0
		for _, child := range node.children {
			size += computeSize(child)
		}
		m[node.Path] = size
		return size
	}

	m[fs.root.Path] = computeSize(fs.root)
	return m
}

func (fs *fileSystem) String() string {
	var traverse func(node *fsNode, depth int, isLast bool) string

	// depthLast is a mapping of the depth level to a boolean value
	// indicating if it is the last row for that depth or not.
	depthLast := make(map[int]bool)

	traverse = func(node *fsNode, depth int, isLast bool) string {
		// Set the flag stating whether this is the last row for
		// the current depth.
		depthLast[depth] = isLast

		// leading is the leading part of the row excluding the file
		// part and details.
		var leading string

		// Root is at depth 0, so we should start from depth 1.
		for d := 1; d < depth; d++ {
			if depthLast[d] {
				leading += partBlank
			} else {
				leading += partLine
			}
		}

		// filePart is the part added before the file details to denote
		// whether this is the last file in the current stack or not.
		var filePart string

		// Root is at depth 0, so it doesn't include the file part.
		if depth > 0 {
			if isLast {
				filePart = partCorner
			} else {
				filePart = partEdge
			}
		}
		s := fmt.Sprintf("%s%s%s\n", leading, filePart, node.Name)

		// lastIdx is the index of the last child of the current node.
		lastIdx := len(node.children) - 1

		for idx, child := range node.children {
			s += traverse(child, depth+1, idx == lastIdx)
		}

		return s
	}

	return traverse(fs.root, 0, len(fs.root.children) == 0)
}

// newEmptyDir creates an empty directory with the given name. If the
// parent node is not nil, it will add the new node as a child of the
// parent node.
//
// The path of the directory will be the given name if parent is nil,
// else it will be the path by joining the parent path and the name.
// This assumes that the given name is only the last part of the path.
func newEmptyDir(name string, parent *fsNode) *fsNode {
	var path string
	if parent != nil {
		path = filepath.Join(parent.Path, name)
	} else {
		// This must be the root directory.
		path = name
	}
	dir := &fsNode{
		Type:     modeDir,
		Path:     path,
		Name:     name,
		Size:     0,
		parent:   parent,
		children: []*fsNode{},
	}
	if parent != nil {
		parent.children = append(parent.children, dir)
	}
	return dir
}

// createFileSystem will create a new file system by processing the
// given commands.
//
// Only the "cd" and "ls" commands are processed.
func createFileSystem(cmds []*command) *fileSystem {
	fs := &fileSystem{} // create an empty fileSystem

	// currentDir is the current directory node.
	var currentDir *fsNode

CmdLoop:
	for _, cmd := range cmds {
		switch cmd.Name {
		case "cd":
			dir := cmd.Args[1]
			switch dir {
			case "/":
				if fs.root == nil {
					// root directory has no parent
					fs.root = newEmptyDir(dir, nil)
				}
				currentDir = fs.root
			case "..":
				currentDir = currentDir.parent
			default:
				for _, child := range currentDir.children {
					if child.Type == modeDir && child.Name == dir {
						currentDir = child
						continue CmdLoop
					}
				}
				panic("cd: no such directory: " + dir)
			}
		case "ls":
			// Output of the `ls` command is either of the following:
			//   1) <size> <filename>
			//   2) dir <dirname>
			for _, line := range cmd.Stdout {
				first, name, found := strings.Cut(line, " ")
				if !found {
					panic("ls: invalid output: " + line)
				}
				switch first {
				case "dir":
					// This will also add the new directory as a child to
					// the current directory.
					newEmptyDir(name, currentDir)
				default:
					size := util.MustAtoi(first)
					currentDir.children = append(currentDir.children, &fsNode{
						Type:     modeFile,
						Path:     filepath.Join(currentDir.Path, name),
						Name:     name,
						Size:     uint(size),
						parent:   currentDir,
						children: nil,
					})
				}
			}
		}
	}

	return fs
}

// command represents a terminal command which has already ran.
type command struct {
	// Name is the command name which was ran.
	Name string

	// Args contains the command line arguments, including the
	// command itself.
	Args []string

	// Stdout contains the output lines for the given command.
	Stdout []string
}

func (c *command) String() string {
	s := fmt.Sprintf("$ %s", strings.Join(c.Args, " "))
	for _, line := range c.Stdout {
		s += "\n" + line
	}
	return s
}

// parseTerminalOutput parses the terminal output into the commands.
func parseTerminalOutput(lines []string) []*command {
	var cmds []*command

	// prevCmd is the previous command kept as a reference to append
	// the stdout line.
	var prevCmd *command

	for _, line := range lines {
		if line[0] == commandPrefix {
			args := strings.Fields(line[2:])
			prevCmd = &command{
				Name:   args[0],
				Args:   args,
				Stdout: []string{},
			}
			cmds = append(cmds, prevCmd)
		} else {
			prevCmd.Stdout = append(prevCmd.Stdout, line)
		}
	}

	return cmds
}

func Sol07(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	fs := createFileSystem(parseTerminalOutput(lines))
	dirSize := fs.dirSize()

	// minSpaceToDelete is the minimum space to be freed up to run the update.
	minSpaceToDelete := (requiredSpace - (totalDiskSpace - dirSize[fs.root.Path]))

	var totalSize uint = 0
	toDeleteSpace := totalDiskSpace // take the largest number
	for _, size := range dirSize {
		if size <= 100000 {
			totalSize += size
		}
		if size >= minSpaceToDelete {
			toDeleteSpace = util.Min(toDeleteSpace, size)
		}
	}

	fmt.Printf("7.1: %d\n7.2: %d\n", totalSize, toDeleteSpace)
	return nil
}
