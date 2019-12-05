package main

import (
	"crypto/tls"
	"crypto/x509"
	"io/ioutil"
	"os"

	"github.com/labstack/gommon/log"
	"github.com/parnurzeal/gorequest"
)

func main() {
	url := os.Getenv("URL")
	username := os.Getenv("BASIC_AUTH_USERNAME")
	password := os.Getenv("BASIC_AUTH_PASSWORD")
	certFile := os.Getenv("CERT_PATH") // "/Users/thucle/Downloads/optus-screens-adobecqms-net-chain.pem"

	log.Info("URL: ", url)
	log.Info("USERNAME: ", username)
	log.Info("PASSWORD: ", password)
	log.Info("CERT_PATH: ", certFile)

	request := gorequest.New()
	if len(username) > 0 || len(password) > 0 {
		request = request.SetBasicAuth(username, password)
	}
	if len(certFile) > 0 {
		request = request.TLSClientConfig(getCA(certFile))
	}
	resp, body, errs := request.Get(url).Send(nil).End()
	if len(errs) > 0 {
		log.Error("Error when sending request: ", errs)
		return
	}

	log.Infof("Success with code: %v\nResponseData: %s", resp.StatusCode, body)
}

func getCA(certFile string) *tls.Config {
	// Get the SystemCertPool, continue with an empty pool on error
	rootCAs, _ := x509.SystemCertPool()
	if rootCAs == nil {
		rootCAs = x509.NewCertPool()
	}

	// Read in the cert file
	certs, err := ioutil.ReadFile(certFile)
	if err != nil {
		log.Fatalf("Failed to append %q to RootCAs: %v", certFile, err)
	}

	// Append our cert to the system pool
	if ok := rootCAs.AppendCertsFromPEM(certs); !ok {
		log.Fatal("No certs appended, using system certs only")
	}
	return &tls.Config{
		RootCAs: rootCAs,
	}
}
