package main

import (
	"crypto/tls"
	"crypto/x509"
	"fmt"
	"io/ioutil"
	"os"
	"strconv"

	"github.com/parnurzeal/gorequest"
)

func main() {
	// Compose envs
	url := os.Getenv("URL")
	method := os.Getenv("METHOD")
	body := os.Getenv("REQUEST_BODY")
	username := os.Getenv("BASIC_AUTH_USERNAME")
	password := os.Getenv("BASIC_AUTH_PASSWORD")
	certFile := os.Getenv("CERT_PATH")
	skipVerify := os.Getenv("SKIP_VERIFY")

	fmt.Println("URL:", url)
	fmt.Println("METHOD:", method)
	fmt.Println("REQUEST_BODY:", body)
	fmt.Println("USERNAME:", username)
	fmt.Println("PASSWORD:", password)
	fmt.Println("CERT_PATH:", certFile)
	fmt.Println("SKIP_VERIFY:", skipVerify)

	if len(url) == 0 {
		panic("URL must be available")
	}

	if len(method) == 0 {
		method = gorequest.GET
	}

	// Send request
	request := gorequest.New()
	if len(username) > 0 || len(password) > 0 {
		request = request.SetBasicAuth(username, password)
	}
	if len(certFile) > 0 {
		config, err := getCA(certFile)
		if err != nil {
			panic(fmt.Sprintf("Error when sending request: %v", err))
		}
		request = request.TLSClientConfig(config)
	}
	if len(body) > 0 {
	}
	if skip, _ := strconv.ParseBool(skipVerify); skip {
		request = request.TLSClientConfig(&tls.Config{InsecureSkipVerify: true})
	}
	request = request.CustomMethod(method, url)
	var resp gorequest.Response
	var respBody string
	var errs []error
	if len(body) > 0 {
		resp, respBody, errs = request.Send(body).End()
	} else {
		resp, respBody, errs = request.Send(nil).End()
	}
	if len(errs) > 0 {
		panic(fmt.Sprintf("Error when sending request: %v", errs))
	}
	fmt.Printf("Response code: %v\nResponseData: %s", resp.StatusCode, respBody)
}

func getCA(certFile string) (*tls.Config, error) {
	// Get the SystemCertPool, continue with an empty pool on error
	rootCAs, _ := x509.SystemCertPool()
	if rootCAs == nil {
		rootCAs = x509.NewCertPool()
	}

	// Read in the cert file
	certs, err := ioutil.ReadFile(certFile)
	if err != nil {
		return nil, fmt.Errorf("Failed to append %q to RootCAs: %v", certFile, err)
	}

	// Append our cert to the system pool
	if ok := rootCAs.AppendCertsFromPEM(certs); !ok {
		return nil, fmt.Errorf("No certs appended, using system certs only")
	}
	return &tls.Config{
		RootCAs: rootCAs,
	}, nil
}
