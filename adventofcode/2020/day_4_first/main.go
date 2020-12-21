package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	passports, err := ParsePassports("./passport.txt")
	if err != nil {
		panic(err)
	}
	fmt.Println("Total passports: ", len(passports))
	fmt.Println("Valid passports: ", len(passports.GetValidPassports()))
	fmt.Println("Invalid passports: ", len(passports)-len(passports.GetValidPassports()))
}

type Passport struct {
	BirthYear      string //byr
	IssueYear      string //iyr
	ExpirationYear string //eyr
	Height         string //hgt
	HairColor      string //hcl
	EyeColor       string //ecl
	PassportID     string //pid
	CountryID      string //cid
}

type Passports []Passport

func (p Passport) IsValid() bool {
	if p.Height == "" ||
		p.BirthYear == "" ||
		p.IssueYear == "" ||
		p.ExpirationYear == "" ||
		p.Height == "" ||
		p.HairColor == "" ||
		p.EyeColor == "" ||
		p.PassportID == "" {
		return false
	}
	return true
}

func (ps Passports) GetValidPassports() Passports {
	var validPs Passports
	for _, p := range ps {
		if p.IsValid() {
			validPs = append(validPs, p)
		}
	}
	return validPs
}

func ParsePassports(filePath string) (Passports, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return Passports{}, err
	}
	fscanner := bufio.NewScanner(file)
	passports := Passports{}
	addPassport := func(s string) error {
		passport, err := ParsePassport(strings.TrimSpace(s))
		if err != nil {
			return err
		}
		passports = append(passports, passport)
		return nil
	}
	var lastString string
	for fscanner.Scan() {
		if len(fscanner.Text()) == 0 {
			if err := addPassport(lastString); err != nil {
				return Passports{}, err
			}
			lastString = ""
			continue
		}
		lastString += fscanner.Text() + " "
	}
	if err := addPassport(lastString); err != nil {
		return Passports{}, err
	}
	err = file.Close()
	if err != nil {
		return Passports{}, err
	}
	return passports, nil
}

func ParsePassport(s string) (p Passport, err error) {
	fields := strings.Split(s, " ")
	for _, field := range fields {
		k, v, err := ParseField(field)
		if err != nil {
			return Passport{}, err
		}

		switch k {
		case "byr":
			p.BirthYear = v
		case "iyr":
			p.IssueYear = v
		case "eyr":
			p.ExpirationYear = v
		case "hgt":
			p.Height = v
		case "hcl":
			p.HairColor = v
		case "ecl":
			p.EyeColor = v
		case "pid":
			p.PassportID = v
		case "cid":
			p.CountryID = v
		}
	}

	return p, nil
}

func ParseField(s string) (key, value string, err error) {
	parts := strings.SplitN(s, ":", 2)
	if len(parts) < 2 {
		return "", "", fmt.Errorf("Can't parse field from %v, %+v", s, parts)
	}
	return parts[0], parts[1], nil
}
