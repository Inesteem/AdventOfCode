package main

import (
	"fmt"
	"sort"
)

type void struct{}

var entry void

type cache struct {
	cache map[coord3D]void
	list  []coord3D
}

func getCache() cache {
	return cache{
		cache: make(map[coord3D]void, 0),
	}
}

func (c *cache) cacheCube(cube cube) {
	_, ok := c.cache[cube.center]
	assert(!ok, "cubes should be distinct")
	c.cache[cube.center] = entry
	c.list = append(c.list, cube.center)
}

func (c *cache) addKey(key coord3D) {
	c.cache[key] = entry
}
func (c *cache) hasKey(key coord3D) bool {
	_, ok := c.cache[key]
	return ok
}

func (c *cache) sort() {
	sort.Slice(c.list,
		func(i, j int) bool {
			return c.list[i].z < c.list[j].z
		})

}

func (c *cache) contacts(startPos coord3D) bool {
	for x := -1; x <= 1; x++ {
		for y := -1; y <= 1; y++ {
			for z := -1; z <= 1; z++ {
				if x == z && z == y && x == 0 {
					continue
				}
				pos := startPos
				pos.goIn(coord3D{x: x, y: y, z: z})
				if c.hasKey(pos) {
					return true
				}
			}
		}
	}
	return false
}

func (c *cache) steamExpand(blocks *cache, startPos coord3D, dirs []coord3D) {
	for d := range dirs {
		pos := startPos
		pos.goIn(dirs[d])
		if c.hasKey(pos) || blocks.hasKey(pos) {
			continue
		}
		if blocks.contacts(pos) {
			c.addKey(pos)
			c.steamExpand(blocks, pos, dirs)
		}
	}
}
func main() {
	var dirs []coord3D = []coord3D{COORD3D_UP, COORD3D_DOWN, COORD3D_LEFT, COORD3D_RIGHT, COORD3D_BACK, COORD3D_FRONT}
	lines, _ := readLines("input")
	cache := getCache()
	for i := range lines {
		c := parseCube(lines[i])
		cache.cacheCube(c)
	}

	cache.sort()
	steamCache := getCache()
	startPos := cache.list[0]
	startPos.goIn(COORD3D_DOWN)
	steamCache.addKey(startPos)

	steamCache.steamExpand(&cache, startPos, dirs)

	star1, star2 := 0, 0

	for i := range cache.list {
		for d := range dirs {
			key := cache.list[i]
			key.goIn(dirs[d])
			if !cache.hasKey(key) {
				star1 += 1
				if !steamCache.hasKey(key) {
					star2 += 1
				}
			}
		}
	}

	fmt.Println("star1:", star1)
	fmt.Println("star2:", star1-star2)
}
