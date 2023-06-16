# The birthday reminder
Welcome to the terminal birthday reminder! Every time you open the terminal it 
lists every birthday that is in the next 14 days including how old the person
turns.
That comes in the following format:
```bash
"$NAME has birthday in $DAYS days"
```
And when the birthday finally rolls around:
```bash
"$NAME turns $AGE today!"
```

## How it works
After running `bin/setup` a new directory called `.birthday_reminder/`. In there
you will have a `release/birthday-reminder`. That's where the code lives.  
The first time running it, it will ask you to initialize a 
`~/.birthday_reminder/birthdays.txt` file. There you should choose `y`, otherwise
the program doesn't work.

## birthdays.txt
Inside the `birthdays.txt` you will need to add your birthdays. As of now there
is no command-line tool, but that should come before version 1 is released.
Currently birthdays are added as follows:
```
$NAME=$BIRTHDAY
```
Keep in mind, that `$BIRTHDAY` needs to be in `yyyy.mm.dd` format. Doing this wrong
will cause the program to panic.

## Performance
A fair concern is, how this will affect your performance. Running the `time` command
yields following results on an m1 pro:
```
time ./release/birthday-reminder
No birthdays in the next 14 days!
./release/birthday-reminder  0.00s user 0.00s system 46% cpu 0.011 total
```
Should be fast enough.
