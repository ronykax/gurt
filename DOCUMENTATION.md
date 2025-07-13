# gurt 🥀
gurt is an esolang (eso)

---

## syntax 🤖
---

### 💠 `yo` – declare a variable
```gurt
yo x is 10
yo name is "gurt"
```

### 💠 `is` – declare a variable
```gurt
yo x is 69
```

### 💠 `bet` – define a function
```
bet say_hi():
    yap("hi bestie")
```

### 💠 `yap()` – print stuff
```
yap("hello world")
```

### 💠 `f_around`, `find_out`, `at_last` – try/except/finally
```
f_around:
    yo x is 1 / 0
find_out:
    yap("you messed up")
at_last:
    yap("done trying")
```

### 💠 `lockin` – for loop
```
lockin i is range(3):
    yap(i)
```

### 💠 `sus` – if condition
```
yo x is 5

sus x > 3:
    yap("valid")
```

### 💠 `ong` – while loop
```
yo count is 0

ong count < 3:
    yap(count)
    yo count is count + 1
```

### 💠 `get_out` – break
```
lockin i is range(5):
    sus i is 3:
        get_out
    yap(i)
```

---

## running gurt 🚀
```
gurt path/to/file.gurt
```