package choose_ratio_by_resolution

import (
	"math"
)

type PredefinedRatio struct {
	Name string
	Ratio string
	Width int
	Height int
	KName string
}

var SupportedRatios = []PredefinedRatio{
	// 1:1
	{Name: "1:1 (1K)", Ratio: "1:1", Width: 1024, Height: 1024, KName: "1K"},
	{Name: "1:1 (2K)", Ratio: "1:1", Width: 2048, Height: 2048, KName: "2K"},
	{Name: "1:1 (4K)", Ratio: "1:1", Width: 4096, Height: 4096, KName: "4K"},

	// 2:3
	{Name: "2:3 (1K)", Ratio: "2:3", Width: 848, Height: 1264, KName: "1K"},
	{Name: "2:3 (2K)", Ratio: "2:3", Width: 1696, Height: 2528, KName: "2K"},
	{Name: "2:3 (4K)", Ratio: "2:3", Width: 3392, Height: 5056, KName: "4K"},

	// 3:2
	{Name: "3:2 (1K)", Ratio: "3:2", Width: 1264, Height: 848, KName: "1K"},
	{Name: "3:2 (2K)", Ratio: "3:2", Width: 2528, Height: 1696, KName: "2K"},
	{Name: "3:2 (4K)", Ratio: "3:2", Width: 5056, Height: 3392, KName: "4K"},

	// 3:4
	{Name: "3:4 (1K)", Ratio: "3:4", Width: 896, Height: 1200, KName: "1K"},
	{Name: "3:4 (2K)", Ratio: "3:4", Width: 1792, Height: 2400, KName: "2K"},
	{Name: "3:4 (4K)", Ratio: "3:4", Width: 3584, Height: 4800, KName: "4K"},

	// 4:3
	{Name: "4:3 (1K)", Ratio: "4:3", Width: 1200, Height: 896, KName: "1K"},
	{Name: "4:3 (2K)", Ratio: "4:3", Width: 2400, Height: 1792, KName: "2K"},
	{Name: "4:3 (4K)", Ratio: "4:3", Width: 4800, Height: 3584, KName: "4K"},

	// 4:5
	{Name: "4:5 (1K)", Ratio: "4:5", Width: 928, Height: 1152, KName: "1K"},
	{Name: "4:5 (2K)", Ratio: "4:5", Width: 1856, Height: 2304, KName: "2K"},
	{Name: "4:5 (4K)", Ratio: "4:5", Width: 3712, Height: 4608, KName: "4K"},

	// 5:4
	{Name: "5:4 (1K)", Ratio: "5:4", Width: 1152, Height: 928, KName: "1K"},
	{Name: "5:4 (2K)", Ratio: "5:4", Width: 2304, Height: 1856, KName: "2K"},
	{Name: "5:4 (4K)", Ratio: "5:4", Width: 4608, Height: 3712, KName: "4K"},

	// 9:16
	{Name: "9:16 (1K)", Ratio: "9:16", Width: 768, Height: 1376, KName: "1K"},
	{Name: "9:16 (2K)", Ratio: "9:16", Width: 1536, Height: 2752, KName: "2K"},
	{Name: "9:16 (4K)", Ratio: "9:16", Width: 3072, Height: 5504, KName: "4K"},

	// 16:9
	{Name: "16:9 (1K)", Ratio: "16:9", Width: 1376, Height: 768, KName: "1K"},
	{Name: "16:9 (2K)", Ratio: "16:9", Width: 2752, Height: 1536, KName: "2K"},
	{Name: "16:9 (4K)", Ratio: "16:9", Width: 5504, Height: 3072, KName: "4K"},

	// 21:9
	{Name: "21:9 (1K)", Ratio: "21:9", Width: 1584, Height: 672, KName: "1K"},
	{Name: "21:9 (2K)", Ratio: "21:9", Width: 3168, Height: 1344, KName: "2K"},
	{Name: "21:9 (4K)", Ratio: "21:9", Width: 6336, Height: 2688, KName: "4K"},
}

/*
ChooseRatioByResolution selects the predefined ratio closest to the given screen dimensions.
Parameters:
  - screenWidth: the width of the screen resolution we want to output.
  - screenHeight: the height of the screen resolution we want to output.
  - supportedRatios: a list of pre-defined ratios that we support.

Output:
  - Returns the predefined ratio that is closest to the screen resolution's aspect ratio.
*/
func ChooseRatioByResolution(screenWidth float64, screenHeight float64, supportedRatios []PredefinedRatio) PredefinedRatio {
	if len(supportedRatios) == 0 {
		return PredefinedRatio{}
	}

	if screenHeight == 0 {
		return supportedRatios[0]
	}

	// Calculate the aspect ratio of the screen
	screenRatio := screenWidth / screenHeight

	// Find the closest predefined ratio
	closestRatio := supportedRatios[0]
	minDiff := math.MaxFloat64

	for _, ratio := range supportedRatios {
		if ratio.Height == 0 {
			continue
		}
		predefinedRatio := float64(ratio.Width) / float64(ratio.Height)
		diff := math.Abs(screenRatio - predefinedRatio)
		if diff < minDiff {
			minDiff = diff
			closestRatio = ratio
		}
	}

	return closestRatio
}

type ResizeAndCropImageSuggestion struct {
	Step1ResizeWidth float64
	Step1ResizeHeight float64
	Step2CropWidth float64
	Step2CropHeight float64
}

/*
	GetResizeAndCropImageSuggestion is a function that takes an input width and height and an output width and height and returns a ResizeAndCropImageSuggestion.
	Ideally, it will do 2 steps:
		1. Resize the image (with predefined ratio) to the closest output width and height.
		2. Crop the resized image to the expected output width and height.
	Parameters:
		- inputWidth: the width of the input image.
		- inputHeight: the height of the input image.
		- outputWidth: the width of the output image.
		- outputHeight: the height of the output image.
	Output:
		- Returns a ResizeAndCropImageSuggestion.
*/
func GetResizeAndCropImageSuggestion(inputWidth float64, inputHeight float64, outputWidth float64, outputHeight float64) ResizeAndCropImageSuggestion {
	// Handle edge cases
	if inputWidth == 0 || inputHeight == 0 || outputWidth == 0 || outputHeight == 0 {
		return ResizeAndCropImageSuggestion{}
	}

	// Calculate scale factors to fit the output dimensions
	scaleWidth := outputWidth / inputWidth
	scaleHeight := outputHeight / inputHeight

	// Use the larger scale factor to ensure the resized image covers the entire output area
	// This way we can crop to the exact output dimensions without any gaps
	scale := math.Max(scaleWidth, scaleHeight)

	// Calculate the resized dimensions
	resizeWidth := inputWidth * scale
	resizeHeight := inputHeight * scale

	return ResizeAndCropImageSuggestion{
		Step1ResizeWidth:  resizeWidth,
		Step1ResizeHeight: resizeHeight,
		Step2CropWidth:    outputWidth,
		Step2CropHeight:   outputHeight,
	}
}