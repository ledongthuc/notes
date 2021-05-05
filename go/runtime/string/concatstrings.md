# Concat strings

str1 + str2 + str3 + ...

## 1. Empty string is skipped

 - When we concat a empty string together `"" + "" + ""`, runtime checks and return empty string without process anything.

## 2. Max length

 - Before process any logic of concat string, runtime verify total length of final result. If final string length's reach over max size of `int`, runtime throws error.
