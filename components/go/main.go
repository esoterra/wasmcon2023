package main

import (
	greetAdapter "app/gen"
)

func init() {
	a := GreetAdapterImpl{}
	greetAdapter.SetHandler(a)
}

type GreetAdapterImpl struct {
}

func (i GreetAdapterImpl) greet() string {
	return greetAdapter.imports.greet() + " and Go!"
}

//go:generate wit-bindgen tiny-go ../../wit/world.wit --world greeter-adapter --out-dir=gen
func main() {}