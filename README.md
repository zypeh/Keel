# keel

### TODO
[ ] Code Style in doc
[ ] Project architecture overview
[ ] Standardise AST (both struct and json format)

### Code Style

**.clang-format**
```
---
Language: Cpp
IndentWidth: 8
TabWidth:    8
ColumnLimit: 109
UseTab:      Never

AlignTrailingComments: true
AlignOperands:         true

SortIncludes: true
SpaceBeforeAssignmentOperators: true
SpaceAfterCStyleCast: true
SpaceBeforeParens: ControlStatements
```

**.flake8**
```
max-line-length = 109
exclude = .git,__pycache__
max-complexity = 10
```
