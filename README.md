# Push over over args
## Syntax
Just run it with args and it will send a message
  
``` 
./push_over_over_args Hello devices it's me
```
It would return with a success or failure

## Setup
Where the executable is located, you would need to create a file called .env
In that file you would want to type
```
USER=<USER key pushover>
TOKEN=<TOKEN key pushover>
``` 
## To do
- Better error handling 
- Add automatic User and Token saving
- Add more args for title or anything else
