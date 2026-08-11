package choose_ratio_by_resolution

import (
	"testing"
)

func TestChooseRatioByResolution(t *testing.T) {
	tests := []struct {
		name            string
		screenWidth     float64
		screenHeight    float64
		supportedRatios []PredefinedRatio
		expected        PredefinedRatio
	}{
		{
			name:            "empty supported ratios returns empty struct",
			screenWidth:     1920,
			screenHeight:    1080,
			supportedRatios: []PredefinedRatio{},
			expected:        PredefinedRatio{},
		},
		{
			name:            "zero screen height returns first ratio",
			screenWidth:     1920,
			screenHeight:    0,
			supportedRatios: SupportedRatios,
			expected:        PredefinedRatio{Name: "1:1 (1K)", Ratio: "1:1", Width: 1024, Height: 1024, KName: "1K"},
		},
		{
			name:            "16:9 resolution (1920x1080) matches 16:9",
			screenWidth:     1920,
			screenHeight:    1080,
			supportedRatios: SupportedRatios,
			expected:        PredefinedRatio{Name: "16:9 (1K)", Ratio: "16:9", Width: 1376, Height: 768, KName: "1K"},
		},
		{
			name:            "16:9 resolution (2560x1440) matches 16:9",
			screenWidth:     2560,
			screenHeight:    1440,
			supportedRatios: SupportedRatios,
			expected:        PredefinedRatio{Name: "16:9 (1K)", Ratio: "16:9", Width: 1376, Height: 768, KName: "1K"},
		},
		{
			name:            "4:3 resolution (1024x768) matches 4:3",
			screenWidth:     1024,
			screenHeight:    768,
			supportedRatios: SupportedRatios,
			expected:        PredefinedRatio{Name: "4:3 (1K)", Ratio: "4:3", Width: 1200, Height: 896, KName: "1K"},
		},
		{
			name:            "1:1 square resolution matches 1:1",
			screenWidth:     1000,
			screenHeight:    1000,
			supportedRatios: SupportedRatios,
			expected:        PredefinedRatio{Name: "1:1 (1K)", Ratio: "1:1", Width: 1024, Height: 1024, KName: "1K"},
		},
		{
			name:            "9:16 portrait resolution matches 9:16",
			screenWidth:     1080,
			screenHeight:    1920,
			supportedRatios: SupportedRatios,
			expected:        PredefinedRatio{Name: "9:16 (1K)", Ratio: "9:16", Width: 768, Height: 1376, KName: "1K"},
		},
		{
			name:            "21:9 ultrawide resolution matches 21:9",
			screenWidth:     2560,
			screenHeight:    1080,
			supportedRatios: SupportedRatios,
			expected:        PredefinedRatio{Name: "21:9 (1K)", Ratio: "21:9", Width: 1584, Height: 672, KName: "1K"},
		},
		{
			name:            "3:2 resolution matches 3:2",
			screenWidth:     1500,
			screenHeight:    1000,
			supportedRatios: SupportedRatios,
			expected:        PredefinedRatio{Name: "3:2 (1K)", Ratio: "3:2", Width: 1264, Height: 848, KName: "1K"},
		},
		{
			name:            "2:3 portrait resolution matches 2:3",
			screenWidth:     1000,
			screenHeight:    1500,
			supportedRatios: SupportedRatios,
			expected:        PredefinedRatio{Name: "2:3 (1K)", Ratio: "2:3", Width: 848, Height: 1264, KName: "1K"},
		},
		{
			name:         "custom ratios list picks closest",
			screenWidth:  1920,
			screenHeight: 1080,
			supportedRatios: []PredefinedRatio{
				{Name: "1:1", Ratio: "1:1", Width: 1024, Height: 1024, KName: "1K"},
				{Name: "4:3", Ratio: "4:3", Width: 1200, Height: 896, KName: "1K"},
			},
			expected: PredefinedRatio{Name: "4:3", Ratio: "4:3", Width: 1200, Height: 896, KName: "1K"},
		},
		{
			name:         "single ratio in list returns that ratio",
			screenWidth:  1920,
			screenHeight: 1080,
			supportedRatios: []PredefinedRatio{
				{Name: "1:1", Ratio: "1:1", Width: 1024, Height: 1024, KName: "1K"},
			},
			expected: PredefinedRatio{Name: "1:1", Ratio: "1:1", Width: 1024, Height: 1024, KName: "1K"},
		},
		{
			name:         "skips ratio with zero height",
			screenWidth:  1920,
			screenHeight: 1080,
			supportedRatios: []PredefinedRatio{
				{Name: "invalid", Ratio: "0:0", Width: 1920, Height: 0, KName: ""},
				{Name: "16:9", Ratio: "16:9", Width: 1376, Height: 768, KName: "1K"},
			},
			expected: PredefinedRatio{Name: "16:9", Ratio: "16:9", Width: 1376, Height: 768, KName: "1K"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := ChooseRatioByResolution(tt.screenWidth, tt.screenHeight, tt.supportedRatios)
			if result.Name != tt.expected.Name {
				t.Errorf("ChooseRatioByResolution(%v, %v).Name = %v, want %v",
					tt.screenWidth, tt.screenHeight, result.Name, tt.expected.Name)
			}
			if result.Ratio != tt.expected.Ratio {
				t.Errorf("ChooseRatioByResolution(%v, %v).Ratio = %v, want %v",
					tt.screenWidth, tt.screenHeight, result.Ratio, tt.expected.Ratio)
			}
			if result.Width != tt.expected.Width {
				t.Errorf("ChooseRatioByResolution(%v, %v).Width = %v, want %v",
					tt.screenWidth, tt.screenHeight, result.Width, tt.expected.Width)
			}
			if result.Height != tt.expected.Height {
				t.Errorf("ChooseRatioByResolution(%v, %v).Height = %v, want %v",
					tt.screenWidth, tt.screenHeight, result.Height, tt.expected.Height)
			}
			if result.KName != tt.expected.KName {
				t.Errorf("ChooseRatioByResolution(%v, %v).KName = %v, want %v",
					tt.screenWidth, tt.screenHeight, result.KName, tt.expected.KName)
			}
		})
	}
}

func TestChooseRatioByResolution_EmptyResult(t *testing.T) {
	result := ChooseRatioByResolution(1920, 1080, []PredefinedRatio{})

	if result.Name != "" || result.Ratio != "" || result.Width != 0 || result.Height != 0 || result.KName != "" {
		t.Errorf("Expected empty PredefinedRatio, got %+v", result)
	}
}

