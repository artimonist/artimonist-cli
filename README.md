# [Artimonist](https://www.artimonist.org)  
### A tool for generating mnemonics and wallets.

### Simple diagram
|【 | | | | | | |
|-|-|-|-|-|-|-|
| |1| | | | | |
| | |$| | | | |
| | | |≈| | | |
| | | | |⅞| | |
| | | | | |£| |
| | | | | | |】|


#### Generate mnemonics
```blank
> artimonist simple  
> row (1) "【"  ""  ""  ""  ""  ""  ""  
> row (2) ""  "1"  ""  ""  ""  ""  ""  
> row (3) ""  ""  "$"  ""  ""  ""  ""  
> row (4) ""  ""  ""  "≈"  ""  ""  ""  
> row (5) ""  ""  ""  ""  "⅞"  ""  ""  
> row (6) ""  ""  ""  ""  ""  "£"  ""  
> row (7) ""  ""  ""  ""  ""  ""  "】"  

Mnemonics: 
(0): tattoo slide more city sample ask tell unfold category spoil mother bottom assume session rib humble school usage ensure game bottom able mind exile
```
> Accepts any Unicode character written as "\u{1234}"  
> With Encryption Key of "123456"  


#### Generate wallets
```blank
> artimonist simple --wallet --amount 5
  
Wifs:
(0): 3QUo3a7XB8u9hQK8qjNpjbjy13NfNyFvBa, 6PYTgnyGxwze4uCPu159m6wDxcmng7P4zpJZXQCUT1c4ULf44U8YyDkGMn
(1): 35doRbCyWPShWdmteEJ22WF5erBuodf36B, 6PYWhNEcMShynPayAaZzVgkQ1PdHtjoqgvpUP73NSJFKnvEuAmFdrUQ1ad
(2): 39fwZPrpp7wrq5dBrXFYs8HnbnS5V8sgJL, 6PYQHCK2RJBYwTxGoth59ejdpMQpa71zjiU3A76EZdVJCo8ytDHZRyBtDf
(3): 3DpJtFBEyXknqme54LpM3WrMncKDiZzRQx, 6PYXMCM7edE4yriik3PfSgRyV2wJVWF19SdPcNs9PReJCBP6Ljsr3yFrdy
(4): 3AQU4ZCDBHK6QFBa9A47MpaCesiRSVvB5a, 6PYPZmdi6rG6dBrR5tyV2EPdDEQqd1WW7qLyVWtD66BQALA2tVnqrezUMc
```
> address and encrypted private key


#### Generate master key
```blank
> artimonist simple --xprv

Xprvs:  
(0): xprv9s21ZrQH143K4NhZJaqTRwAaQztwHG3fUbLur3MBRiVJhEg1ZVQPGzStfYaBJnVtrdsJUGhPCEdYnh9e4K8XLuDP3XhPtMFypV3ujRtgRY4
```


#### Derive wallets
```blank
> artimonist derive "tattoo slide more city sample ask tell unfold category spoil mother bottom assume session rib humble school usage ensure game bottom able mind exile"

[m/49'/0'/0'/0/0]: 3HpzFSi3vDpP4LEMphC1rpiArzAQARuhCz, 6PYS49dBhWXYUGYoJRs1BjiwWP9xFtMvTSUrz3LH25pgkx7vHLRjWhPh77
[m/49'/0'/0'/0/1]: 3LNibRvHXCAfFNtAxsGTdgc4QK6zfux1Xh, 6PYWdYWUHEUWbrpZfsqyqDqLEdKciPU4z2CUP5dTAZUgAA5P3kRpdJfssd
[m/49'/0'/0'/0/2]: 38CJzvL1JTEP4hsotKSeXwWycGSzaadMY4, 6PYQsKHKd5t98dejwB4mPtztciPbdBpMvEYEW7RfgdbkirU9dQgD9R4fzR
[m/49'/0'/0'/0/3]: 35EMZ6LJ7FSqDkUUbrHBzoWrbxvCuAC5iW, 6PYS77m41stXJrnUSjkQDjTKofNxseKw7nisnrwJFb5qiZv45ZfrMXJNAD
[m/49'/0'/0'/0/4]: 36wWQaePbom8zvRFeNcjpr2Jwq99RY88os, 6PYT6LyxaPpmN3CY3DhgdSN1YTFBySN5tYomMuj5AS9i4bD8hGNSGvEhiC
```


#### Derive multisig wallets
```blank
> artimonist derive --m23 --detail xprv9s21ZrQH143K4NhZJaqTRwAaQztwHG3fUbLur3MBRiVJhEg1ZVQPGzStfYaBJnVtrdsJUGhPCEdYnh9e4K8XLuDP3XhPtMFypV3ujRtgRY4

Account xpubs: [m/49'/0'/0'] ~ [m/49'/0'/2']
[m/49'/0'/0']: xpub6BjjJvci8YJwRTSJaWkPezV4qKoTVWtm77jgFG6k6rT137tMht1929iVCZHGKfgQXo865wuswijs7vMqzV9s4hGPK6u8pj8ckeipS9ULPxm
[m/49'/0'/1']: xpub6BjjJvci8YJwUHa9yfYaF3NWg55RsTTkVNywZibPUPn6wiyZ57f5By7RqBhzCY8uL5GQfYatikaVLqyK8DUgi5ZrcGfLJaKraG8uXme82uq
[m/49'/0'/2']: xpub6BjjJvci8YJwUYMbGZG7QkpMVnyMhz9wvmEeXZQ2MzML9WHYyiyDUQFsTXT1DzFdXroLXYfhtSJy9m6n9J5Ye66GyTPgcLgnJU1i54He8f9

Account xprvs: [m/49'/0'/0'] ~ [m/49'/0'/2']
[m/49'/0'/0']: xprv9xkNuR5pJAkeCyMqUVDPHrYLHHxy64Aujtp5Ssh8YWv2AKZDALgtUMQ1MFShNCSLS4v1F1L9XqxFE56aer9watYobVb52UXAhpC7BrEbA8X
[m/49'/0'/1']: xprv9xkNuR5pJAkeFoVgse1ZsuRn83EwTzju8A4LmLBmv4F84veQXaLpeAnwyvgD1ZvuYeYMJDViQGKFUz4EG4xsjoQDq8bE8aJSKso1FMoezta
[m/49'/0'/2']: xprv9xkNuR5pJAkeG4H8AXj73cscwm8sJXS6ZYK3jAzQoepMGhxQSBexvbwPcF4AHWQtbKJd8HdhrC6WznjxdM8CtZYxHzPZQbEThFKKLT29eY8

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

Redeem scripts:
[m/0/0]: 522102150d3144895fe0af428448876dcb9748e893a572ee7f469c865ab140fd65f5ec2102ce54fa0619c5ba7f036faaf432a1d8ec39dd4cae0eed650b787b819bad53822a21038b45906d143409dc98c2373abb8d7a249b1ccdb6aa72445ea163ce8d6b2db4cb53ae
[m/0/1]: 5221039c3bd88f187ed1f9ff6ad7f3624f43d73ba999b907576dd90e70954c77c6c50a2103acaa3cf1dace72e66169c7e23573bbee58ac45ce27eb9ae7843a4aad6e027a532103ff9c3147481a07f87cccdd7b12b419f84e2c1d59041afa0aa727001e16d029cb53ae
[m/0/2]: 52210288377dc2426cccd0a08b1787ac621ae0dedd4399794b66cdd2a3926680f6d1d621030b1f38d1ecbd61584567a66a127498a7966eef31356016a68ee6f338e88b9214210364f16403c8bb1b52621c66bd104800ef109871167ffc12e992342624ee7523ce53ae
[m/0/3]: 52210217dc857c33cc40807584a36ffab5a108cf8c873b80335f992a1629568f4a5cd5210229462108ad8adc198dc2a73d1d28120c82d4442a2fd512f5c166219a0cbb3c672103e5d720a3d40081266f32feaa15e3b7bcf0c2426a587510df97ab6dc13abb782a53ae
[m/0/4]: 522102a6beb4cba2135a80180d1a20d772bdb0ee7667cce0ab81e41f74ba3c4278119b2102de7f0c00ea61ccffd8d9654a4426eeee2571baa5ab2672770c4421f2874d316c21037258d0cf8463e5d8c2215a865c15bff270eb6f31829d4138af1fd21f0bec9a6253ae
```
