# artimonist-cli
A tool for generating mnemonics based on diagrams.

**`> artimonist-cli --help`**

```
Usage: artimonist-cli [OPTIONS] <CONTENT> [INDICES]...

Arguments:
  <CONTENT>     diagram chars
  [INDICES]...  diagram indices: (row, col), 0 <= row < 7, 0 <= col < 7

Options:
  -t, --target <TARGET>  generate target [default: mnemonic] [possible values: mnemonic, wif, xpriv, pwd]
  -c, --count <COUNT>    generate count [default: 1]
  -s, --salt <SALT>      salt
  -h, --help             Print help
```

### mnemonic
**`> artimonist-cli 【1$≈⅞£】 (0,0) (1,1) (2,2) (3,3) (4,4) (5,5) (6,6)`**
```
0: melody stove spell vessel gold cricket public ensure family seek piano mechanic trap truck roast bind alley donor runway miracle entry crop act vocal

```

### wif
**`> artimonist-cli --target wif --count 5 【1$≈⅞£】 (0,0) (1,1) (2,2) (3,3) (4,4) (5,5) (6,6)`**
```
0: ( 38FPFdaGWejCRawvsCbSiuj8AgAJGJ1vNz, KzWLnHF5ESw43o6Uy4ZCixVT8LEA4cQnF3m47FQp6WwBn3QzmbsZ )
1: ( 3QRD7qGyF2QAXrRiRuCr6H2D4DEXLRicco, KxpzGv35MPYLFammP7hqGgGCc81g6Gx5cYfLRapPURmSS9oEPJxT )
2: ( 32EUNg5vHukrVkE2PA2hMawfeoeXeoJXGj, L4evBDU2brn915UXnoiisDPjET8xkCuwTo4Ef5C6ki3RPT3vrWzM )
3: ( 35vpqRzaJ83G2DpRBSrWaaN3ELWfNB3BCZ, L1uXQySss2vSkJf6BvqLwjFemwqgKawtKToW922paWrcrV95Vs6P )
4: ( 328Z2MBojc2f4bFo9RvxLJQizP4SbcWLic, L4YQmGr4582nGVirJBd5ktu1KnAN12VzBfx2V22HQgWNUXEDvctJ )
``` 

### xpriv
**`> artimonist-cli --target xpriv --salt artimonist 【1$≈⅞£】 (0,0) (1,1) (2,2) (3,3) (4,4) (5,5) (6,6)`**
```
0: xprv9s21ZrQH143K26RCtmAMZxtw7UJ9EzVwVtftmszx9y1LC4PktDeFNAXFT7Qwdpy1vQHkV5fiRxUbX8NznqvkKJQaU5Nj74hzERT8mdcmPw5
```
### pwd
**`> artimonist-cli -t pwd -c 10 【1$≈⅞£】 (0,0) (1,1) (2,2) (3,3) (4,4) (5,5) (6,6)`**
```
0: FtbFj#LWmrsK@n$UhpK1
1: tkyS%EG5nf%h1Y5U*g75
2: YTv1oFxX4#Nys75LqDhb
3: 3Kq37Cso%1Fcn$8$D7EJ
4: r%nJ4817&Q8L9N*AbbLk
5: wNTDNhzZ&NuM5@9bEP6F
6: &PUcYESkHQ%TimRTJW&8
7: %61pCvURZgqTAePA7HDi
8: Yu%k4b2VjL8GMBFHknvS
9: H6K&BeU34b2ycsZK2cbe
```
