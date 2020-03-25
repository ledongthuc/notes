package main

import (
	"net/http"
	"os"
	"strings"

	"github.com/labstack/echo"
)

func main() {
	e := echo.New()
	for _, route := range getPaths() {
		e.GET(route.path, func(c echo.Context) error {
			return c.String(http.StatusOK, route.message)
		})
	}
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})
	e.Logger.Fatal(e.Start(":8080"))
}

type route struct {
	path    string
	message string
}

type routes []route

const EnvName = "CUSTOM_ROUTES"

func getPaths() routes {
	env := os.Getenv(EnvName)
	if len(env) == 0 {
		return routes{}
	}
	var result routes
	for _, pathPair := range strings.Split(env, ";") {
		pairs := strings.SplitN(pathPair, ":", 2)
		if len(pairs) < 2 {
			continue
		}
		result = append(result, route{
			path:    pairs[0],
			message: pairs[1],
		})
	}
	return result
}
