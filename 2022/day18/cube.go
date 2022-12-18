package main

import (
	"log"
	"regexp"
)

var coord3DRegex = regexp.MustCompile("([0-9]+),([0-9]+),([0-9]+)")

type cube struct {
	center coord3D
}

func parseCube(line string) cube {

	match := coord3DRegex.FindStringSubmatch(line)
	if got, want := len(match), 4; got != want {
		log.Fatalf("%s does not fit format; -want(%d) +got(%d)", line, want, got)
	}

	return cube{
		center: coord3D{
			x: toInt(match[1]),
			y: toInt(match[2]),
			z: toInt(match[3])}}
}
