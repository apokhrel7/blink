package main

import (
	"fmt"
	"sync"
)

// Cache represents a simple thread-safe cache
type Cache struct {
	sync.RWMutex
	data map[string][]byte
}

// TODO: Add proper error handling
func NewCache() *Cache {
	return &Cache{
		data: make(map[string][]byte),
	}
}

// FIXME: Consider adding size limits
func (c *Cache) Set(key string, value []byte) {
	c.Lock()
	defer c.Unlock()
	c.data[key] = value
}

// NOTE: This is a basic implementation
func (c *Cache) Get(key string) ([]byte, bool) {
	c.RLock()
	defer c.RUnlock()
	val, ok := c.data[key]
	return val, ok
}

// ExampleClass demonstrates class-like structure
type ExampleClass struct {
	cache *Cache
}

// ERROR: Need to implement proper initialization
func (e *ExampleClass) ProcessData() {
	fmt.Println("Processing data...")
} 