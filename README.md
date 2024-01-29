# SOMT - Simple File Management Tool

## Avalable commands : 
```
dir_read - files look up location
limit - number of rows printed
print - print rows
help - shows basic comand
sort - sorts by file size (asc/desc)
ends_with - name slice that file should end with
grouped - returns grouped list by FILE & FOLDER type
largest - composed shortcut command for retrieving FILE/FOLDER from specific location.
          options: --location, --limit
          example: largest=folder --location=/home --limit=10
```

### Query example :
`largest=folder --location=E:\tmp --limit=10`

#### Result:
```
[FOLDER] [E:] - [0.000085] MB
[FOLDER] [E:\tmp] - [0.000085] MB
[FOLDER] [E:\tmp\test] - [0.000019] MB
```