package main

import (
	"testing"
)

func Test_validatePassword(t *testing.T) {
	type args struct {
		containPos    int64
		notContainPos int64
		checking      byte
		password      string
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "InValid",
			args: args{
				containPos:    1,
				notContainPos: 3,
				checking:      'a',
				password:      "abcde",
			},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := validatePassword(tt.args.containPos, tt.args.notContainPos, tt.args.checking, tt.args.password); got != tt.want {
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
		name              string
		args              args
		wantContainPos    int64
		wantNotContainPos int64
		wantIsValid       bool
	}{
		{
			name:              "Valid",
			args:              args{rawRange: "6-10"},
			wantContainPos:    6,
			wantNotContainPos: 10,
			wantIsValid:       true,
		},
		{
			name:              "invalid",
			args:              args{rawRange: "6-10-"},
			wantContainPos:    0,
			wantNotContainPos: 0,
			wantIsValid:       false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotContainPos, wantNotContainPos, gotIsValid := parseRange(tt.args.rawRange)
			if gotContainPos != tt.wantContainPos {
				t.Errorf("parseRange() gotContainPos = %v, want %v", gotContainPos, tt.wantContainPos)
			}
			if wantNotContainPos != tt.wantNotContainPos {
				t.Errorf("parseRange() gotNotContainPos = %v, want %v", wantNotContainPos, tt.wantNotContainPos)
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
				rawRule:  "1-3 a",
				password: "abcde",
			},
			want: true,
		},
		{
			name: "valid 2",
			args: args{
				rawRule:  "1-3 a",
				password: "cbade",
			},
			want: true,
		},
		{
			name: "invalid",
			args: args{
				rawRule:  "1-3 b",
				password: "cdefg",
			},
			want: false,
		},
		{
			name: "invalid",
			args: args{
				rawRule:  "2-9 c",
				password: "ccccccccc",
			},
			want: false,
		},
		{
			name: "invalid - out of range contain",
			args: args{
				rawRule:  "6-7 a",
				password: "abcde",
			},
			want: false,
		},
		{
			name: "invalid - out of range not-contain",
			args: args{
				rawRule:  "3-6 c",
				password: "abcde",
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
