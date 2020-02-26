# Interface embedding

```
go run main.go
```

Output:

```
ChildFunction()
ParentFunction()
```


## Embedded and embedding interafaces have same func

```
go run same_func.go
```

Output: 

```
ChildFunction()
ParentFunction()
Action()
```

## Different signatures

```
go run same_func2.go
```

Output:

```
./same_func2.go:11:2: duplicate method Action
./same_func2.go:34:4: cannot use concreate literal (type concreate) as type child in assignment:
        concreate does not implement child (wrong type for Action method)
                have Action() string
                want Action() int
./same_func2.go:37:3: ambiguous selector c.Action
```

## Diamond embedding

```
go run diamond.go
```

Output:

```
./diamond.go:5:6: invalid recursive type d1
```

## Tree cycle embedding

```
                           ┌───────┐                
                           │       │                
                           │       │                
┌────────┐            ┌────────┐   │                
│   d4   │            │   d5   │   │                
└────────┘            └────────┘   │                
     ▲                     ▲       │                
     └──────────┬──────────┘       │        ───────┐
                │                  │   ┌────────┐  │
                │                  │   │   d6   │  │
           ┌────────┐              │   └────────┘  │
           │   d3   │              │        ▲      │
           └────────┘              │        │      │
                ▲            ┌─────┴────────┘      │
     ┌──────────┴────────────┤                     │
     │                       │                     │
┌────────┐              ┌────────┐                 │
│   d1   │              │   d2   │                 │
└────────┘              └────────┘                 │
     ▲                                             │
     └─────────────────────────────────────────────┘
```

```
go run tree_cycle_embedding.go
```

Output:

```
./tree_cycle_embedding.go:5:6: invalid recursive type d1
```
