package main

import (
	"reflect"
	"testing"
)

func Test_parse2ElfRanges(t *testing.T) {
	tests := []struct {
		name    string
		args    string
		want    [2]ElfRange
		wantErr bool
	}{
		{
			name: "2-4,6-8",
			args: "2-4,6-8",
			want: [2]ElfRange{{from: 2, to: 4}, {from: 6, to: 8}},
		},
		{
			name: "2-3,4-5",
			args: "2-3,4-5",
			want: [2]ElfRange{{from: 2, to: 3}, {from: 4, to: 5}},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := parse2ElfRanges(tt.args)
			if (err != nil) != tt.wantErr {
				t.Errorf("parse2ElfRanges() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("parse2ElfRanges() = %v, want %v", got, tt.want)
			}
		})
	}
}
