package main

import (
	"testing"
)

func Test_validatePassword(t *testing.T) {
	type args struct {
		min         int64
		max         int64
		checkingStr string
		password    string
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "InValid",
			args: args{
				min:         1,
				max:         3,
				checkingStr: "p",
				password:    "ctpppjmdpppppp",
			},
			want: false,
		},
		{
			name: "Valid",
			args: args{
				min:         5,
				max:         6,
				checkingStr: "x",
				password:    "xxxxvxx",
			},
			want: true,
		},
		{
			name: "Valid 2",
			args: args{
				min:         9,
				max:         11,
				checkingStr: "t",
				password:    "tttttttcttm",
			},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := validatePassword(tt.args.min, tt.args.max, tt.args.checkingStr, tt.args.password); got != tt.want {
				t.Errorf("validatePassword() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_parseRange(t *testing.T) {
	type args struct {
		rawRange string
	}
	tests := []struct {
		name        string
		args        args
		wantMin     int64
		wantMax     int64
		wantIsValid bool
	}{
		{
			name:        "Valid",
			args:        args{rawRange: "6-10"},
			wantMin:     6,
			wantMax:     10,
			wantIsValid: true,
		},
		{
			name:        "invalid",
			args:        args{rawRange: "6-10-"},
			wantMin:     0,
			wantMax:     0,
			wantIsValid: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotMin, gotMax, gotIsValid := parseRange(tt.args.rawRange)
			if gotMin != tt.wantMin {
				t.Errorf("parseRange() gotMin = %v, want %v", gotMin, tt.wantMin)
			}
			if gotMax != tt.wantMax {
				t.Errorf("parseRange() gotMax = %v, want %v", gotMax, tt.wantMax)
			}
			if gotIsValid != tt.wantIsValid {
				t.Errorf("parseRange() gotIsValid = %v, want %v", gotIsValid, tt.wantIsValid)
			}
		})
	}
}

func Test_isValidRule(t *testing.T) {
	type args struct {
		rawRule  string
		password string
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "valid",
			args: args{
				rawRule:  "8-11 t",
				password: "tttttttcttm",
			},
			want: true,
		},
		{
			name: "invalid",
			args: args{
				rawRule:  "8-11 a",
				password: "tttttttcttm",
			},
			want: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := isValidRule(tt.args.rawRule, tt.args.password); got != tt.want {
				t.Errorf("isValidRule() = %v, want %v", got, tt.want)
			}
		})
	}
}
