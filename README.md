# Artimonist
A tool for generating mnemonics and wallets.  
Web version: **<https://www.artimonist.org>**

### Help infomation
**`> artimonist --help`**
``` blank
Usage: artimonist <COMMAND>

Commands:
  simple   Use simple diagram of 7 * 7 unicode chars
  complex  Use complex diagram of 7 * 7 unicode strings
  encrypt  Encrypt private key by bip38
  decrypt  Decrypt private key by bip38
  derive   Derive from master key or mnemonic
  bip32    Derive by custom bip32 path
  help     Print this message or the help of the given subcommand(s)
```
**`> artimonist simple --help`**
``` blank
Usage: artimonist simple [OPTIONS]

Options:
  -i, --index <INDEX>      Start index [default: 0]
  -m, --amount <AMOUNT>    Amount to generate [default: 1]
  -f, --file <FILE>        Input diagram from text file
      --unicode            Export unicode view for non-displayable character
      --mnemonic <LENGTH>  Generate bip39 mnemonic [default] [possible values: 12, 15, 18, 21, 24]
      --wif                Generate wallet address and private key [aliases: --wallet]
      --xprv               Generate master key for HD-Wallet [aliases: --master]
      --pwd                Generate password
  -h, --help               Print help
```
**`> artimonist derive --help`**
``` blank
Usage: artimonist derive [OPTIONS] <MNEMONIC|MASTER_KEY>

Arguments:
  <MNEMONIC|MASTER_KEY>  Mnemonic phrase or Master key

Options:
  -a, --account <ACCOUNT>  Account start index [default: 0]
  -i, --index <INDEX>      Address start index [default: 0]
  -m, --amount <AMOUNT>    Amount of address [default: 5]
      --bip44              Use derive path: m/44'/0'/account'/0/index [p2pkh]
      --bip49              Use derive path: m/49'/0'/account'/0/index [p2shwpkh, default]
      --bip84              Use derive path: m/84'/0'/account'/0/index [p2wpkh]
      --m23                Multiple signatures address of 2-3 [derive path: account'/0/index]
      --m35                Multiple signatures address of 3-5 [derive path: account'/0/index]
      --detail             Export account xprv, xpub and redeem scripts of multisig
  -h, --help               Print help
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
_(Some terminal fonts display different characters width, using monospaced font maybe display correctly.)_

### mnemonic
**`> artimonist simple`**  
`> row (1) "【"  ""  ""  ""  ""  ""  ""`  
`> row (2) ""  "1"  ""  ""  ""  ""  ""`  
`> row (3) ""  ""  "$"  ""  ""  ""  ""`  
`> row (4) ""  ""  ""  "≈"  ""  ""  ""`  
`> row (5) ""  ""  ""  ""  "⅞"  ""  ""`  
`> row (6) ""  ""  ""  ""  ""  "£"  ""`  
`> row (7) ""  ""  ""  ""  ""  ""  "】"`  
_(Accepts any Unicode character written as "\u{1234}")_  
_(With salt of '123456')_  
``` blank
(0): tattoo slide more city sample ask tell unfold category spoil mother bottom assume session rib humble school usage ensure game bottom able mind exile
```

### wallet
**`> artimonist simple --wallet --amount 5`**  
``` blank
(0): 3QUo3a7XB8u9hQK8qjNpjbjy13NfNyFvBa, 6PYTgnyGxwze4uCPu159m6wDxcmng7P4zpJZXQCUT1c4ULf44U8YyDkGMn
(1): 35doRbCyWPShWdmteEJ22WF5erBuodf36B, 6PYWhNEcMShynPayAaZzVgkQ1PdHtjoqgvpUP73NSJFKnvEuAmFdrUQ1ad
(2): 39fwZPrpp7wrq5dBrXFYs8HnbnS5V8sgJL, 6PYQHCK2RJBYwTxGoth59ejdpMQpa71zjiU3A76EZdVJCo8ytDHZRyBtDf
(3): 3DpJtFBEyXknqme54LpM3WrMncKDiZzRQx, 6PYXMCM7edE4yriik3PfSgRyV2wJVWF19SdPcNs9PReJCBP6Ljsr3yFrdy
(4): 3AQU4ZCDBHK6QFBa9A47MpaCesiRSVvB5a, 6PYPZmdi6rG6dBrR5tyV2EPdDEQqd1WW7qLyVWtD66BQALA2tVnqrezUMc
``` 

### xprv
**`> artimonist simple --xprv`**  
``` blank
(0): xprv9s21ZrQH143K4NhZJaqTRwAaQztwHG3fUbLur3MBRiVJhEg1ZVQPGzStfYaBJnVtrdsJUGhPCEdYnh9e4K8XLuDP3XhPtMFypV3ujRtgRY4
```
### derive
**`> artimonist derive "tattoo slide more city sample ask tell unfold category spoil mother bottom assume session rib humble school usage ensure game bottom able mind exile"`**  
``` blank
[m/49'/0'/0'/0/0]: 3HpzFSi3vDpP4LEMphC1rpiArzAQARuhCz, 6PYS49dBhWXYUGYoJRs1BjiwWP9xFtMvTSUrz3LH25pgkx7vHLRjWhPh77
[m/49'/0'/0'/0/1]: 3LNibRvHXCAfFNtAxsGTdgc4QK6zfux1Xh, 6PYWdYWUHEUWbrpZfsqyqDqLEdKciPU4z2CUP5dTAZUgAA5P3kRpdJfssd
[m/49'/0'/0'/0/2]: 38CJzvL1JTEP4hsotKSeXwWycGSzaadMY4, 6PYQsKHKd5t98dejwB4mPtztciPbdBpMvEYEW7RfgdbkirU9dQgD9R4fzR
[m/49'/0'/0'/0/3]: 35EMZ6LJ7FSqDkUUbrHBzoWrbxvCuAC5iW, 6PYS77m41stXJrnUSjkQDjTKofNxseKw7nisnrwJFb5qiZv45ZfrMXJNAD
[m/49'/0'/0'/0/4]: 36wWQaePbom8zvRFeNcjpr2Jwq99RY88os, 6PYT6LyxaPpmN3CY3DhgdSN1YTFBySN5tYomMuj5AS9i4bD8hGNSGvEhiC
```

### multisig
**`> artimonist derive --m23 xprv9s21ZrQH143K4NhZJaqTRwAaQztwHG3fUbLur3MBRiVJhEg1ZVQPGzStfYaBJnVtrdsJUGhPCEdYnh9e4K8XLuDP3XhPtMFypV3ujRtgRY4`**  
``` blank
Account xpubs: [m/49'/0'/0'] ~ [m/49'/0'/2']
  xpub6BjjJvci8YJwRTSJaWkPezV4qKoTVWtm77jgFG6k6rT137tMht1929iVCZHGKfgQXo865wuswijs7vMqzV9s4hGPK6u8pj8ckeipS9ULPxm
  xpub6BjjJvci8YJwUHa9yfYaF3NWg55RsTTkVNywZibPUPn6wiyZ57f5By7RqBhzCY8uL5GQfYatikaVLqyK8DUgi5ZrcGfLJaKraG8uXme82uq
  xpub6BjjJvci8YJwUYMbGZG7QkpMVnyMhz9wvmEeXZQ2MzML9WHYyiyDUQFsTXT1DzFdXroLXYfhtSJy9m6n9J5Ye66GyTPgcLgnJU1i54He8f9

Addresses:
┌───────┬────────────────────────────────────┐
│ Path  │ Address                            │
╞═══════╪════════════════════════════════════╡
│ m/0/0 │ 3Ewm3we2SiU4Nan9PKKvD2Mqsm7y1iLeXn │
├───────┼────────────────────────────────────┤
│ m/0/1 │ 3JBHQRtAnTkX2hxKmgYD71xjQ5XqVPTcS6 │
├───────┼────────────────────────────────────┤
│ m/0/2 │ 3Ctj9uSuuCb4gSNcuMdToQBG6LpN8EAdyW │
├───────┼────────────────────────────────────┤
│ m/0/3 │ 3NMV1ABNpuiZm7gcgNLyVNPXkd7MgkfkFA │
├───────┼────────────────────────────────────┤
│ m/0/4 │ 37fb8tLHQzCKPo3thPzapm44bsT7xAdsAB │
└───────┴────────────────────────────────────┘
```