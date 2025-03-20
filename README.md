# Survex-helper usage

## Basic usage commands

´survex-helper COMMAND OPTIONS ´

|command|description|
|---------|---------|
| **create** | Creates a new project |
| **print** | Print only |

## Example config

```yaml
project:
  name: my_project
  author: My Name
options:
  fix: 
    point: A1
    lat: 111111.11
    lon: 222222.22
    elevation: 333.0
  entrance:
    - B1
    - A1
  equate:
    points:
      - from: A17
        to: C1
      - from: B4
        to: A1
```
