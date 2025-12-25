# How to use AI to generate image fit to expected resolution

AI image generation today have some limition on output resolution they can generated. At least with online provider.

So we need a flow to make the output image from AI image generation fit with what we need.

In this guide, I will use Google Banana Pro with predefined ratio, resoltion they supported as example. Feel free to change it based on your need.

## Step 1: Choose closed ration/resultion that AI provider supported.

Let say we need a output image with resolution that they don't support yet.
We need a method me pickup the ratio/resolution closed with what you need first

Take a look function `ChooseRatioByResolution()` in `lib.go`

## Step 2: Scale it and Crop it.

My suggestion: Always scaling before croping.

Ideally, we will scale the width and the height of output image from step 1, keep ratio to closed with that we need. Normally, we scale height or width match exactly the expected output, and keep another  with same ratio (but bigger than expected output).

Then crop the (height or width) to match with expected output.

## Bonus 1:

How AI generate the text, position of the text overlay on the image background (x, y, size)

After generated image, we use it as input of an LLM to ask about text, position, size overlay.

## Bonus 2:

How to generate image,  with brand guideline

Example:
