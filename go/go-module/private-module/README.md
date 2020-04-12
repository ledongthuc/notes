# private module 

Go module support private repo.

## Define private repo

```
go env -w GOPRIVATE=github.com/liminaab/captron_go_lib;
```

It ingore the public checksum from public proxy of Go


## Update required access permission of git with your repo

Get/create access tokens from https://github.com/settings/tokens

and update it to:

```
cat ~/.gitconfig


# This is Git's per-user configuration file.
[url "git@github.com:{org_name}/{repo_name}.git"]
    insteadOf = https://{org_name}/{repo_name}/captron_go_lib
```

## And run it

```
go get github.com/{org_name}/repo_name/
go run main.go
```
