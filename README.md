# Push over over args
## Syntax
Just run it with args and it will send a message
  
``` 
./push_over_over_args Hello devices it's me
```
It would return with a success or failure

## Setup
Compile the rust application and move it to your bin folder or wherever you want it to be located.

Where the executable is located, you would need to create a file called .env
In that file you would want to type
```
USERAPI=<USER key pushover>
TOKEN=<TOKEN key pushover>
``` 
If you would like change the defualt title, you can add an other enviroment variable 
```
TITLE="<Title you would want>"
```
## To do
- Better error handling 
- Add automatic User and Token saving
- Add more args for title or anything else

## Extra comments
This is still in its production stage. And as I'm a 14 year old, I would not trust this to work for a serious project. All that said, hope this does the job for what ever you need it for.
