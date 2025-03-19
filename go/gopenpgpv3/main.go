package main

import (
	"fmt"

	"github.com/ProtonMail/gopenpgp/v3/crypto"
)

func main() {
	const pubkey = `-----BEGIN PGP PUBLIC KEY BLOCK-----
Version: BCPG v1.41

mQENBFc7DpwBCACRPrsPoV7DlbA1J9Q963LATRLB3kTlOwOOgpKnPOmsL8+RVpmu
qsf16B5ET4IUdfxRW2z7bnjPjbnu3W2xaG1MaoNVEMkscHSL/Q3uDw9uO35JbD9/
vaXSgabmz9iCf93ByzFvtFRX4kzDrbpsFGr5YTe52KB9xn9qwWIzj+Zgfu5Kivkg
w2CFB6jVqwyVdY3RONAgsInC1nTKWY9c+ByXuIsi9bXuN9TIdm61U5tYfWoD91aL
6HyYy6QD4sC61TsIHlUZOyTI/Ey+iOwfducRHzOcwOEOpIB46fTYOz3HgaR9Ji2r
8PLUrCpbgSBrwg3gs0AUdN9Ns+5yEaLXDQqtABEBAAG0NXNlYl9wdWJsaWNfdGVz
dF9zaWduX2VuY3J5cHRfMjA0OCA8QklTU3VwcG9ydEBzZWIuc2U+iQEiBBMBAgAM
BQJXOw6cBYltukiAAAoJEPTCLrlNdfpTRUMH/1rxWw74oH67vW2yWy4tdZbmFCY1
mKGQ8r2R7tPpni+bFlmc+cqj83Yic19ZwepAc+eeFuDQUf/whp4/bQABppzNtbi8
0IUuwjYMZRajQyOcbQdrU3VtgFG/XQtMIvQjPr7xTT9DjNUOsUswgVYwwbvzStoa
5SSCyO7Df09l4Jr4QauWFS+XeWaBTP/ygmfpsRe+mxqjLGK7tSpQaWA9c0fdo8Pl
I40lsZF5hOuyEKh3k47ofmZgU3GsZSRDHvszOF7reY58syq/zb+ynNLH0rV3Gvju
NGHqbLtHZGsZuGMOawPW6aRG0CiQyAO+5MHOlcKSN9PYPSejPSwwTq+iR70=
=gjTf
-----END PGP PUBLIC KEY BLOCK-----`

	publicKey, err := crypto.NewKeyFromArmored(pubkey)
	if err != nil {
		panic(err)
	}

	pgp := crypto.PGP()
	// Encrypt plaintext message using a public key
	encHandle, err := pgp.Encryption().Recipient(publicKey).New()
	if err != nil {
		panic(err)
	}
	pgpMessage, err := encHandle.Encrypt([]byte("my message"))
	if err != nil {
		panic(err)
	}
	armored, err := pgpMessage.ArmorBytes()
	if err != nil {
		panic(err)
	}
	fmt.Println("DEBUG: ", string(armored))
}
