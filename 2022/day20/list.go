//copied from https://golangbyexample.com/doubly-linked-list-golang/
package main

import (
	"fmt"
	"log"
)

type node struct {
	data int
	prev *node
	next *node
}

func (n *node) getNodeAfter(num int) *node {
	if num == 0 {
		return n
	}
	return n.next.getNodeAfter(num - 1)
}

func (n *node) moveRight(num int) {
	if num == 0 {
		return
	}
	prev := n.prev
	next := n.next
	if next == nil {
		log.Fatal("next node is nil")
	}
	//update node
	n.next = next.next
	n.prev = next

	//update next.next
	if next.next != nil {
		next.next.prev = n
	}
	//update next
	next.prev = prev
	next.next = n

	//update prev
	if prev != nil {
		prev.next = next
	}
	n.moveRight(num - 1)
}
func (n *node) moveLeft(num int) {
	if num == 0 {
		return
	}

	prev := n.prev
	next := n.next
	if prev == nil {
		log.Fatal("prev node is nil")
	}

	//update node
	n.prev = prev.prev
	n.next = prev

	//update prev.prev
	if prev.prev != nil {
		prev.prev.next = n
	}

	//update prev
	prev.next = next
	prev.prev = n

	//update next
	if next != nil {
		next.prev = prev
	}

	n.moveLeft(num - 1)
}

type doublyLinkedList struct {
	len  int
	tail *node
	head *node
}

func initDoublyList() *doublyLinkedList {
	return &doublyLinkedList{}
}

func (d *doublyLinkedList) Append(data int) *node {
	newNode := &node{
		data: data,
	}
	if d.head == nil {
		d.head = newNode
		d.tail = newNode
	} else {
		newNode.prev = d.tail
		d.tail.next = newNode
		d.tail = newNode
	}
	d.len++
	return newNode
}
func (d *doublyLinkedList) TraverseForward() error {
	if d.head == nil {
		return fmt.Errorf("TraverseError: List is empty")
	}
	temp := d.head
	for i := 0; i < d.Size(); i++ {
		fmt.Printf("value = %v, this = %p, prev = %p, next = %p\n", temp.data, temp, temp.prev, temp.next)
		temp = temp.next
	}
	fmt.Println()
	return nil
}

func (d *doublyLinkedList) toRingBuf() {
	if d.tail == nil || d.head == nil {
		log.Fatal("not head or tail")
	}
	if d.tail.next != nil || d.head.prev != nil {
		log.Fatal("invalid state")
	}
	d.tail.next = d.head
	d.head.prev = d.tail
}

func (d *doublyLinkedList) Size() int {
	return d.len
}
