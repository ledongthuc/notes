package main

import (
	"app/pro"
	"fmt"

	"github.com/invopop/jsonschema"
)

func main() {
	jsonSchemaBytes, err := jsonschema.Reflect(&pro.Student{}).MarshalJSON()
	fmt.Println(string(jsonSchemaBytes), err)
}
