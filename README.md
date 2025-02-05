# Artimonist
A tool for generating mnemonics based on diagrams.  
Web version: **<https://www.artimonist.org>**

**`> artimonist simple --help`**
```
Use simple diagram of 7 * 7 chars

Usage: artimonist simple [OPTIONS]

Options:
  -t, --target <TARGET>  Target [default: mnemonic] [possible values: mnemonic, wallet, xpriv, password]
  -s, --serial <SERIAL>  Start serial number [default: 0]
  -m, --amount <AMOUNT>  Amount to generate [default: 1]
  -s, --salt <SALT>      Salt
  -h, --help             Print help
```
**`> artimonist complex --help`**
```
Use complex diagram of 7 * 7 strings

Usage: artimonist complex [OPTIONS]

Options:
  -t, --target <TARGET>  Target [default: mnemonic] [possible values: mnemonic, wallet, xpriv, password]
  -s, --serial <SERIAL>  Start serial number [default: 0]
  -m, --amount <AMOUNT>  Amount to generate [default: 1]
  -s, --salt <SALT>      Salt
  -h, --help             Print help
```

Let's draw a diagram like the one below, and generate multi target by it.
```
+----+---+---+---+---+---+----+
| 【 |   |   |   |   |   |    |
|----+---+---+---+---+---+----|
|    | 1 |   |   |   |   |    |
|----+---+---+---+---+---+----|
|    |   | $ |   |   |   |    |
|----+---+---+---+---+---+----|
|    |   |   | ≈ |   |   |    |
|----+---+---+---+---+---+----|
|    |   |   |   | ⅞ |   |    |
|----+---+---+---+---+---+----|
|    |   |   |   |   | £ |    |
|----+---+---+---+---+---+----|
|    |   |   |   |   |   | 】 |
+----+---+---+---+---+---+----+
```
_(Some terminal fonts display different characters width, using standard font maybe display correctly.)_

### mnemonic
**`> artimonist simple`**  
`> row (1) "【" "" "" "" "" "" ""`  
`> row (2) "" "1" "" "" "" "" ""`  
`> row (3) "" "" "$" "" "" "" ""`  
`> row (4) "" "" "" "≈" "" "" ""`  
`> row (5) "" "" "" "" "⅞" "" ""`  
`> row (6) "" "" "" "" "" "£" ""`  
`> row (7) "" "" "" "" "" "" "】"`  
``` 
Mnemonic results:
0: face shoot relax patch verify six lion proud income copy strategy primary person sign hint mango bargain soldier lobster change follow vehicle material harvest
```

### wallet
**`> artimonist simple --target wallet --amount 5`**  
```
0: 3Cp9s5u2e2Y4mWEDQKnjn7XidkFqwCAR16, Kxnp8CMBWth5yBZHURj4qiHoQZbiu2vsppbFMGAWv6c3hajtmMor
1: 3MDfN9tXdozXKRiGbDpgWujk6haJXXVXSS, KzUjZbdPGN8UqJTE9UXzpQugKWRMZwRqE3vCqhwJJs1dJ3qXSz3z
2: 35mY6LGhApUhgqd5xw3FR4ngZhjGvZjHMq, L4KcnHRnJFdRjHDuLHoGjQ1Lf82Fs2WUanGtRuZsYQChKXN9cs1t
3: 3EgqQwGyeYBtZTdbaposrRuszsaPju3oBK, KxLnnzRK3hdfJ7kfkE6kHsyLEMMoWLypchyJw92dFRG6z6fvNqL5
4: 3QhuuovyzenmJfyjL257AgDK2n7CG3DJSi, KygF68fiRUuk8W2c7nf3iA5Mxzi4rdijz49MKAp1aZ2nkLHkWJ3J
``` 

### xpriv
**`> artimonist simple --target xpriv --salt artimonist`**  
```
0: xprv9s21ZrQH143K2NbNten7yUnUKHWKgmqC51sNJYJMhrvyxXcxD6bDk8W33ZGw3nBezrVVLsfaoFC2SuBRCkgX1Hpyn4er6XCGf1L9uTWmpH9
```

### pwd
**`> artimonist simple -t pwd -m 10`**
```
0: sLVP2EgoUWu#8khAuN4F
1: yo%r9stqLShHW8EXbS1A
2: 7xT5kfHDyqrGQkrV9kku
3: aBj1kp7Wus&eyZh3Y%g5
4: pBnRfSRt9FM*rmhmvBkg
5: j@fEyGzSGF5o#38%H#86
6: 1@oYSzj5DR7cvXHavHHX
7: $vfj#S3WjQ4vkn4iPrXf
8: f7mKae76xBMMdKNN3Yt7
9: zVJMgcxXEUZDwYvayXb*
```