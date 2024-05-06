
# Ghostport

Ghostport is a wip clone of portspoof

## Installation

```bash
git clone https://git.suicidal.network/sad/ghostport.git
cd ghostport 
cargo build
cd target/debug 
./ghostport -s ../../signatures.txt
```

or you can run with cargo 
```bash
git clone https://git.suicidal.network/sad/ghostport.git
cd ghostport 
cargo run -- -s signatures.txt
```

this will run on port 8888 with listening address 127.0.0.1



## Usage
```bash
./ghostport -h # for help
Usage: ghostport [OPTIONS]

Options:
  -s, --signatures <FILE>  Path to the signatures [default: signatures]
  -l, --listen <ADDRESS>   Address to listen on [default: 127.0.0.1:8888]
  -d, --debug              Enable debug logging
  -v, --verbose            Enable verbose logging
  -q, --quiet              Enable quiet logging
  -V, --version            Print version information
  -h, --help               Print hel
```
# example 
```bash
./ghostport -s signatures.txt -l 10.13.37.3:8888 -v 
```
nmap show off, you will need to route your traffic first 
```
nmap -A 10.13.37.3 -vvvvvvvvvv -sV -T4 -Pn
```
results will show as: 
```
Host discovery disabled (-Pn). All addresses will be marked 'up' and scan times may be slower.
Starting Nmap 7.94 ( https://nmap.org ) at 2024-05-06 21:01 UTC
NSE: Loaded 156 scripts for scanning.
NSE: Script Pre-scanning.
NSE: Starting runlevel 1 (of 3) scan.
Initiating NSE at 21:01
Completed NSE at 21:01, 0.00s elapsed
NSE: Starting runlevel 2 (of 3) scan.
Initiating NSE at 21:01
Completed NSE at 21:01, 0.00s elapsed
NSE: Starting runlevel 3 (of 3) scan.
Initiating NSE at 21:01
Completed NSE at 21:01, 0.00s elapsed
Initiating Parallel DNS resolution of 1 host. at 21:01
Completed Parallel DNS resolution of 1 host. at 21:01, 0.14s elapsed
DNS resolution of 1 IPs took 0.14s. Mode: Async [#: 1, OK: 0, NX: 1, DR: 0, SF: 0, TR: 1, CN: 0]
Initiating Connect Scan at 21:01
Scanning 10.13.37.3 [1000 ports]
Discovered open port 3306/tcp on 10.13.37.3
Discovered open port 587/tcp on 10.13.37.3
Discovered open port 5900/tcp on 10.13.37.3
Discovered open port 554/tcp on 10.13.37.3
Discovered open port 143/tcp on 10.13.37.3
Discovered open port 135/tcp on 10.13.37.3
Discovered open port 110/tcp on 10.13.37.3
Discovered open port 25/tcp on 10.13.37.3
Discovered open port 445/tcp on 10.13.37.3
Discovered open port 256/tcp on 10.13.37.3
Discovered open port 53/tcp on 10.13.37.3
Discovered open port 1723/tcp on 10.13.37.3
Discovered open port 21/tcp on 10.13.37.3
Discovered open port 993/tcp on 10.13.37.3
Discovered open port 199/tcp on 10.13.37.3
Discovered open port 1720/tcp on 10.13.37.3
Discovered open port 23/tcp on 10.13.37.3
Discovered open port 80/tcp on 10.13.37.3
Discovered open port 111/tcp on 10.13.37.3
Discovered open port 3389/tcp on 10.13.37.3
Discovered open port 995/tcp on 10.13.37.3
Discovered open port 8888/tcp on 10.13.37.3
Discovered open port 113/tcp on 10.13.37.3
Discovered open port 22/tcp on 10.13.37.3
Discovered open port 1025/tcp on 10.13.37.3
Discovered open port 443/tcp on 10.13.37.3
Discovered open port 8080/tcp on 10.13.37.3
Discovered open port 139/tcp on 10.13.37.3
Discovered open port 32785/tcp on 10.13.37.3
Discovered open port 6009/tcp on 10.13.37.3
Discovered open port 4343/tcp on 10.13.37.3
Discovered open port 1494/tcp on 10.13.37.3
Discovered open port 2106/tcp on 10.13.37.3
Discovered open port 49161/tcp on 10.13.37.3
Discovered open port 9968/tcp on 10.13.37.3
Discovered open port 65000/tcp on 10.13.37.3
Discovered open port 41511/tcp on 10.13.37.3
Discovered open port 1309/tcp on 10.13.37.3
Discovered open port 1148/tcp on 10.13.37.3
Discovered open port 427/tcp on 10.13.37.3
Discovered open port 5718/tcp on 10.13.37.3
Discovered open port 7435/tcp on 10.13.37.3
Discovered open port 700/tcp on 10.13.37.3
Discovered open port 2301/tcp on 10.13.37.3
Discovered open port 1236/tcp on 10.13.37.3
Discovered open port 1021/tcp on 10.13.37.3
Discovered open port 3261/tcp on 10.13.37.3
Discovered open port 1147/tcp on 10.13.37.3
Discovered open port 1417/tcp on 10.13.37.3
Discovered open port 1218/tcp on 10.13.37.3
Discovered open port 30951/tcp on 10.13.37.3
Discovered open port 8651/tcp on 10.13.37.3
Discovered open port 3871/tcp on 10.13.37.3
Discovered open port 5906/tcp on 10.13.37.3
Discovered open port 2111/tcp on 10.13.37.3
Discovered open port 9003/tcp on 10.13.37.3
Discovered open port 1098/tcp on 10.13.37.3
Discovered open port 2251/tcp on 10.13.37.3
Discovered open port 714/tcp on 10.13.37.3
Discovered open port 541/tcp on 10.13.37.3
Discovered open port 27355/tcp on 10.13.37.3
Discovered open port 3283/tcp on 10.13.37.3
Discovered open port 306/tcp on 10.13.37.3
Discovered open port 2004/tcp on 10.13.37.3
Discovered open port 44501/tcp on 10.13.37.3
Discovered open port 1044/tcp on 10.13.37.3
Discovered open port 1050/tcp on 10.13.37.3
Discovered open port 9101/tcp on 10.13.37.3
Discovered open port 49160/tcp on 10.13.37.3
Discovered open port 777/tcp on 10.13.37.3
Discovered open port 3269/tcp on 10.13.37.3
Discovered open port 1999/tcp on 10.13.37.3
Discovered open port 2020/tcp on 10.13.37.3
Discovered open port 2725/tcp on 10.13.37.3
Discovered open port 2875/tcp on 10.13.37.3
Discovered open port 6969/tcp on 10.13.37.3
Discovered open port 8093/tcp on 10.13.37.3
Discovered open port 1130/tcp on 10.13.37.3
Discovered open port 1594/tcp on 10.13.37.3
Discovered open port 9090/tcp on 10.13.37.3
Discovered open port 3128/tcp on 10.13.37.3
Discovered open port 83/tcp on 10.13.37.3
Discovered open port 4900/tcp on 10.13.37.3
Discovered open port 1688/tcp on 10.13.37.3
Discovered open port 56738/tcp on 10.13.37.3
Discovered open port 6543/tcp on 10.13.37.3
Discovered open port 5801/tcp on 10.13.37.3
Discovered open port 3/tcp on 10.13.37.3
Discovered open port 902/tcp on 10.13.37.3
Discovered open port 1192/tcp on 10.13.37.3
Discovered open port 15000/tcp on 10.13.37.3
Discovered open port 1126/tcp on 10.13.37.3
Discovered open port 1117/tcp on 10.13.37.3
Discovered open port 1175/tcp on 10.13.37.3
Discovered open port 49999/tcp on 10.13.37.3
Discovered open port 990/tcp on 10.13.37.3
Discovered open port 8089/tcp on 10.13.37.3
Discovered open port 32776/tcp on 10.13.37.3
Discovered open port 9290/tcp on 10.13.37.3
Discovered open port 8654/tcp on 10.13.37.3
Discovered open port 1259/tcp on 10.13.37.3
Discovered open port 617/tcp on 10.13.37.3
Discovered open port 5962/tcp on 10.13.37.3
Discovered open port 7/tcp on 10.13.37.3
Discovered open port 1114/tcp on 10.13.37.3
Discovered open port 705/tcp on 10.13.37.3
Discovered open port 3546/tcp on 10.13.37.3
Discovered open port 2022/tcp on 10.13.37.3
Discovered open port 31337/tcp on 10.13.37.3
Discovered open port 1271/tcp on 10.13.37.3
Discovered open port 1057/tcp on 10.13.37.3
Discovered open port 1719/tcp on 10.13.37.3
Discovered open port 5226/tcp on 10.13.37.3
Discovered open port 2394/tcp on 10.13.37.3
Discovered open port 2013/tcp on 10.13.37.3
Discovered open port 7200/tcp on 10.13.37.3
Discovered open port 51103/tcp on 10.13.37.3
Discovered open port 3814/tcp on 10.13.37.3
Discovered open port 8222/tcp on 10.13.37.3
Discovered open port 48080/tcp on 10.13.37.3
Discovered open port 4000/tcp on 10.13.37.3
Discovered open port 9071/tcp on 10.13.37.3
Discovered open port 1151/tcp on 10.13.37.3
Discovered open port 1032/tcp on 10.13.37.3
Discovered open port 27000/tcp on 10.13.37.3
Discovered open port 3052/tcp on 10.13.37.3
Discovered open port 8002/tcp on 10.13.37.3
Discovered open port 1244/tcp on 10.13.37.3
Discovered open port 4242/tcp on 10.13.37.3
Discovered open port 49157/tcp on 10.13.37.3
Discovered open port 19842/tcp on 10.13.37.3
Discovered open port 2170/tcp on 10.13.37.3
Discovered open port 82/tcp on 10.13.37.3
Discovered open port 9500/tcp on 10.13.37.3
Discovered open port 8181/tcp on 10.13.37.3
Discovered open port 1022/tcp on 10.13.37.3
Discovered open port 1717/tcp on 10.13.37.3
Discovered open port 3001/tcp on 10.13.37.3
Discovered open port 5989/tcp on 10.13.37.3
Discovered open port 49175/tcp on 10.13.37.3
Discovered open port 32783/tcp on 10.13.37.3
Discovered open port 9485/tcp on 10.13.37.3
Discovered open port 19801/tcp on 10.13.37.3
Discovered open port 7201/tcp on 10.13.37.3
Discovered open port 6789/tcp on 10.13.37.3
Discovered open port 8090/tcp on 10.13.37.3
Discovered open port 2607/tcp on 10.13.37.3
Discovered open port 1068/tcp on 10.13.37.3
Discovered open port 8400/tcp on 10.13.37.3
Discovered open port 7019/tcp on 10.13.37.3
Discovered open port 5810/tcp on 10.13.37.3
Discovered open port 5087/tcp on 10.13.37.3
Discovered open port 5904/tcp on 10.13.37.3
Discovered open port 2030/tcp on 10.13.37.3
Discovered open port 1533/tcp on 10.13.37.3
Discovered open port 2811/tcp on 10.13.37.3
Discovered open port 6699/tcp on 10.13.37.3
Discovered open port 8008/tcp on 10.13.37.3
Discovered open port 648/tcp on 10.13.37.3
Discovered open port 555/tcp on 10.13.37.3
Discovered open port 20222/tcp on 10.13.37.3
Discovered open port 57294/tcp on 10.13.37.3
Discovered open port 5440/tcp on 10.13.37.3
Discovered open port 8042/tcp on 10.13.37.3
Discovered open port 15742/tcp on 10.13.37.3
Discovered open port 1110/tcp on 10.13.37.3
Discovered open port 8402/tcp on 10.13.37.3
Discovered open port 1935/tcp on 10.13.37.3
Discovered open port 1154/tcp on 10.13.37.3
Discovered open port 2869/tcp on 10.13.37.3
Discovered open port 11967/tcp on 10.13.37.3
Discovered open port 1300/tcp on 10.13.37.3
Discovered open port 1100/tcp on 10.13.37.3
Discovered open port 1054/tcp on 10.13.37.3
Discovered open port 10180/tcp on 10.13.37.3
Discovered open port 8333/tcp on 10.13.37.3
Discovered open port 13/tcp on 10.13.37.3
Discovered open port 722/tcp on 10.13.37.3
Discovered open port 3013/tcp on 10.13.37.3
Discovered open port 7999/tcp on 10.13.37.3
Discovered open port 3918/tcp on 10.13.37.3
Discovered open port 1037/tcp on 10.13.37.3
Discovered open port 3300/tcp on 10.13.37.3
Discovered open port 12000/tcp on 10.13.37.3
Discovered open port 593/tcp on 10.13.37.3
Discovered open port 7004/tcp on 10.13.37.3
Discovered open port 6502/tcp on 10.13.37.3
Discovered open port 6112/tcp on 10.13.37.3
Discovered open port 6669/tcp on 10.13.37.3
Discovered open port 9/tcp on 10.13.37.3
Discovered open port 3031/tcp on 10.13.37.3
Discovered open port 4445/tcp on 10.13.37.3
Discovered open port 33/tcp on 10.13.37.3
Discovered open port 8194/tcp on 10.13.37.3
Discovered open port 9011/tcp on 10.13.37.3
Discovered open port 8192/tcp on 10.13.37.3
Discovered open port 7025/tcp on 10.13.37.3
Discovered open port 625/tcp on 10.13.37.3
Discovered open port 5915/tcp on 10.13.37.3
Discovered open port 2998/tcp on 10.13.37.3
Discovered open port 32781/tcp on 10.13.37.3
Discovered open port 5998/tcp on 10.13.37.3
Discovered open port 1060/tcp on 10.13.37.3
Discovered open port 8010/tcp on 10.13.37.3
Discovered open port 32780/tcp on 10.13.37.3
Discovered open port 49159/tcp on 10.13.37.3
Discovered open port 3322/tcp on 10.13.37.3
Discovered open port 8200/tcp on 10.13.37.3
Discovered open port 85/tcp on 10.13.37.3
Discovered open port 8045/tcp on 10.13.37.3
Discovered open port 5862/tcp on 10.13.37.3
Discovered open port 500/tcp on 10.13.37.3
Discovered open port 1010/tcp on 10.13.37.3
Discovered open port 8031/tcp on 10.13.37.3
Discovered open port 3301/tcp on 10.13.37.3
Discovered open port 9103/tcp on 10.13.37.3
Discovered open port 1900/tcp on 10.13.37.3
Discovered open port 1163/tcp on 10.13.37.3
Discovered open port 4446/tcp on 10.13.37.3
Discovered open port 646/tcp on 10.13.37.3
Discovered open port 1233/tcp on 10.13.37.3
Discovered open port 2042/tcp on 10.13.37.3
Discovered open port 1092/tcp on 10.13.37.3
Discovered open port 1077/tcp on 10.13.37.3
Discovered open port 10215/tcp on 10.13.37.3
Discovered open port 9999/tcp on 10.13.37.3
Discovered open port 389/tcp on 10.13.37.3
Discovered open port 10012/tcp on 10.13.37.3
Discovered open port 1028/tcp on 10.13.37.3
Discovered open port 1061/tcp on 10.13.37.3
Discovered open port 4001/tcp on 10.13.37.3
Discovered open port 9009/tcp on 10.13.37.3
Discovered open port 50003/tcp on 10.13.37.3
Discovered open port 301/tcp on 10.13.37.3
Discovered open port 9091/tcp on 10.13.37.3
Discovered open port 50500/tcp on 10.13.37.3
Discovered open port 1062/tcp on 10.13.37.3
Discovered open port 9100/tcp on 10.13.37.3
Discovered open port 2034/tcp on 10.13.37.3
Discovered open port 888/tcp on 10.13.37.3
Discovered open port 3007/tcp on 10.13.37.3
Discovered open port 4449/tcp on 10.13.37.3
Discovered open port 16992/tcp on 10.13.37.3
Discovered open port 5822/tcp on 10.13.37.3
Discovered open port 1248/tcp on 10.13.37.3
Discovered open port 49154/tcp on 10.13.37.3
Discovered open port 3211/tcp on 10.13.37.3
Discovered open port 5100/tcp on 10.13.37.3
Discovered open port 28201/tcp on 10.13.37.3
Discovered open port 1064/tcp on 10.13.37.3
Discovered open port 10082/tcp on 10.13.37.3
Discovered open port 5877/tcp on 10.13.37.3
Discovered open port 3826/tcp on 10.13.37.3
Discovered open port 515/tcp on 10.13.37.3
Discovered open port 5952/tcp on 10.13.37.3
Discovered open port 4045/tcp on 10.13.37.3
Discovered open port 9080/tcp on 10.13.37.3
Discovered open port 7920/tcp on 10.13.37.3
Discovered open port 416/tcp on 10.13.37.3
Discovered open port 667/tcp on 10.13.37.3
Discovered open port 27352/tcp on 10.13.37.3
Discovered open port 1524/tcp on 10.13.37.3
Discovered open port 1095/tcp on 10.13.37.3
Discovered open port 2288/tcp on 10.13.37.3
Discovered open port 15004/tcp on 10.13.37.3
Discovered open port 6100/tcp on 10.13.37.3
Discovered open port 9081/tcp on 10.13.37.3
Discovered open port 1104/tcp on 10.13.37.3
Discovered open port 548/tcp on 10.13.37.3
Discovered open port 4004/tcp on 10.13.37.3
Discovered open port 2119/tcp on 10.13.37.3
Discovered open port 3551/tcp on 10.13.37.3
Discovered open port 49153/tcp on 10.13.37.3
Discovered open port 5500/tcp on 10.13.37.3
Discovered open port 5555/tcp on 10.13.37.3
Discovered open port 32774/tcp on 10.13.37.3
Discovered open port 1755/tcp on 10.13.37.3
Discovered open port 616/tcp on 10.13.37.3
Discovered open port 8021/tcp on 10.13.37.3
Discovered open port 1045/tcp on 10.13.37.3
Discovered open port 5051/tcp on 10.13.37.3
Discovered open port 3889/tcp on 10.13.37.3
Discovered open port 4848/tcp on 10.13.37.3
Discovered open port 7512/tcp on 10.13.37.3
Discovered open port 2701/tcp on 10.13.37.3
Discovered open port 1183/tcp on 10.13.37.3
Discovered open port 2105/tcp on 10.13.37.3
Discovered open port 1234/tcp on 10.13.37.3
Discovered open port 2500/tcp on 10.13.37.3
Discovered open port 1137/tcp on 10.13.37.3
Discovered open port 5061/tcp on 10.13.37.3
Discovered open port 4443/tcp on 10.13.37.3
Discovered open port 9944/tcp on 10.13.37.3
Discovered open port 5811/tcp on 10.13.37.3
Discovered open port 5679/tcp on 10.13.37.3
Discovered open port 11110/tcp on 10.13.37.3
Discovered open port 16016/tcp on 10.13.37.3
Discovered open port 425/tcp on 10.13.37.3
Discovered open port 8300/tcp on 10.13.37.3
Discovered open port 9415/tcp on 10.13.37.3
Discovered open port 7911/tcp on 10.13.37.3
Discovered open port 8100/tcp on 10.13.37.3
Discovered open port 52673/tcp on 10.13.37.3
Discovered open port 1096/tcp on 10.13.37.3
Discovered open port 7777/tcp on 10.13.37.3
Discovered open port 3801/tcp on 10.13.37.3
Discovered open port 32784/tcp on 10.13.37.3
Discovered open port 3404/tcp on 10.13.37.3
Discovered open port 9050/tcp on 10.13.37.3
Discovered open port 1056/tcp on 10.13.37.3
Discovered open port 5815/tcp on 10.13.37.3
Discovered open port 1583/tcp on 10.13.37.3
Discovered open port 7625/tcp on 10.13.37.3
Discovered open port 1185/tcp on 10.13.37.3
Discovered open port 27353/tcp on 10.13.37.3
Discovered open port 6001/tcp on 10.13.37.3
Discovered open port 42510/tcp on 10.13.37.3
Discovered open port 1047/tcp on 10.13.37.3
Discovered open port 100/tcp on 10.13.37.3
Discovered open port 6059/tcp on 10.13.37.3
Discovered open port 1002/tcp on 10.13.37.3
Discovered open port 3324/tcp on 10.13.37.3
Discovered open port 3390/tcp on 10.13.37.3
Discovered open port 9666/tcp on 10.13.37.3
Discovered open port 5988/tcp on 10.13.37.3
Discovered open port 4003/tcp on 10.13.37.3
Discovered open port 179/tcp on 10.13.37.3
Discovered open port 1080/tcp on 10.13.37.3
Discovered open port 1217/tcp on 10.13.37.3
Discovered open port 1041/tcp on 10.13.37.3
Discovered open port 65129/tcp on 10.13.37.3
Discovered open port 2605/tcp on 10.13.37.3
Discovered open port 4129/tcp on 10.13.37.3
Discovered open port 2041/tcp on 10.13.37.3
Discovered open port 6025/tcp on 10.13.37.3
Discovered open port 5225/tcp on 10.13.37.3
Discovered open port 10778/tcp on 10.13.37.3
Discovered open port 1052/tcp on 10.13.37.3
Discovered open port 32782/tcp on 10.13.37.3
Discovered open port 5902/tcp on 10.13.37.3
Discovered open port 6881/tcp on 10.13.37.3
Discovered open port 125/tcp on 10.13.37.3
Discovered open port 2702/tcp on 10.13.37.3
Discovered open port 514/tcp on 10.13.37.3
Discovered open port 16001/tcp on 10.13.37.3
Discovered open port 7103/tcp on 10.13.37.3
Discovered open port 7402/tcp on 10.13.37.3
Discovered open port 7938/tcp on 10.13.37.3
Discovered open port 10024/tcp on 10.13.37.3
Discovered open port 1164/tcp on 10.13.37.3
Discovered open port 1027/tcp on 10.13.37.3
Discovered open port 6839/tcp on 10.13.37.3
Discovered open port 2522/tcp on 10.13.37.3
Discovered open port 3011/tcp on 10.13.37.3
Discovered open port 8011/tcp on 10.13.37.3
Discovered open port 2800/tcp on 10.13.37.3
Discovered open port 18101/tcp on 10.13.37.3
Discovered open port 32777/tcp on 10.13.37.3
Discovered open port 34573/tcp on 10.13.37.3
Discovered open port 16113/tcp on 10.13.37.3
Discovered open port 2144/tcp on 10.13.37.3
Discovered open port 1108/tcp on 10.13.37.3
Discovered open port 1301/tcp on 10.13.37.3
Discovered open port 5950/tcp on 10.13.37.3
Discovered open port 7001/tcp on 10.13.37.3
Discovered open port 99/tcp on 10.13.37.3
Discovered open port 5432/tcp on 10.13.37.3
Discovered open port 259/tcp on 10.13.37.3
Discovered open port 3371/tcp on 10.13.37.3
Discovered open port 1097/tcp on 10.13.37.3
Discovered open port 2021/tcp on 10.13.37.3
Discovered open port 8383/tcp on 10.13.37.3
Discovered open port 2492/tcp on 10.13.37.3
Discovered open port 3827/tcp on 10.13.37.3
Discovered open port 81/tcp on 10.13.37.3
Discovered open port 8087/tcp on 10.13.37.3
Discovered open port 458/tcp on 10.13.37.3
Discovered open port 2040/tcp on 10.13.37.3
Discovered open port 5850/tcp on 10.13.37.3
Discovered open port 1658/tcp on 10.13.37.3
Discovered open port 711/tcp on 10.13.37.3
Discovered open port 10243/tcp on 10.13.37.3
Discovered open port 3077/tcp on 10.13.37.3
Discovered open port 444/tcp on 10.13.37.3
Discovered open port 89/tcp on 10.13.37.3
Discovered open port 2557/tcp on 10.13.37.3
Discovered open port 6666/tcp on 10.13.37.3
Discovered open port 481/tcp on 10.13.37.3
Discovered open port 5922/tcp on 10.13.37.3
Discovered open port 3260/tcp on 10.13.37.3
Discovered open port 3880/tcp on 10.13.37.3
Discovered open port 1086/tcp on 10.13.37.3
Discovered open port 2200/tcp on 10.13.37.3
Discovered open port 55055/tcp on 10.13.37.3
Discovered open port 843/tcp on 10.13.37.3
Discovered open port 6668/tcp on 10.13.37.3
Discovered open port 366/tcp on 10.13.37.3
Discovered open port 5001/tcp on 10.13.37.3
Discovered open port 6101/tcp on 10.13.37.3
Discovered open port 406/tcp on 10.13.37.3
Discovered open port 44176/tcp on 10.13.37.3
Discovered open port 5678/tcp on 10.13.37.3
Discovered open port 2100/tcp on 10.13.37.3
Discovered open port 5910/tcp on 10.13.37.3
Discovered open port 1443/tcp on 10.13.37.3
Discovered open port 5120/tcp on 10.13.37.3
Discovered open port 1310/tcp on 10.13.37.3
Discovered open port 4/tcp on 10.13.37.3
Discovered open port 4567/tcp on 10.13.37.3
Discovered open port 5431/tcp on 10.13.37.3
Discovered open port 3333/tcp on 10.13.37.3
Discovered open port 2910/tcp on 10.13.37.3
Discovered open port 45100/tcp on 10.13.37.3
Discovered open port 563/tcp on 10.13.37.3
Discovered open port 1213/tcp on 10.13.37.3
Discovered open port 1029/tcp on 10.13.37.3
Discovered open port 65389/tcp on 10.13.37.3
Discovered open port 8500/tcp on 10.13.37.3
Discovered open port 49158/tcp on 10.13.37.3
Discovered open port 2103/tcp on 10.13.37.3
Discovered open port 5566/tcp on 10.13.37.3
Discovered open port 4126/tcp on 10.13.37.3
Discovered open port 1580/tcp on 10.13.37.3
Discovered open port 52822/tcp on 10.13.37.3
Discovered open port 5060/tcp on 10.13.37.3
Discovered open port 2967/tcp on 10.13.37.3
Discovered open port 3914/tcp on 10.13.37.3
Discovered open port 3323/tcp on 10.13.37.3
Discovered open port 1296/tcp on 10.13.37.3
Discovered open port 2920/tcp on 10.13.37.3
Discovered open port 8193/tcp on 10.13.37.3
Discovered open port 1131/tcp on 10.13.37.3
Discovered open port 800/tcp on 10.13.37.3
Discovered open port 20828/tcp on 10.13.37.3
Discovered open port 8899/tcp on 10.13.37.3
Discovered open port 52869/tcp on 10.13.37.3
Discovered open port 9111/tcp on 10.13.37.3
Discovered open port 1074/tcp on 10.13.37.3
Discovered open port 14442/tcp on 10.13.37.3
Discovered open port 9040/tcp on 10.13.37.3
Discovered open port 5633/tcp on 10.13.37.3
Discovered open port 49155/tcp on 10.13.37.3
Discovered open port 8009/tcp on 10.13.37.3
Discovered open port 6580/tcp on 10.13.37.3
Discovered open port 20221/tcp on 10.13.37.3
Discovered open port 1328/tcp on 10.13.37.3
Discovered open port 9002/tcp on 10.13.37.3
Discovered open port 6646/tcp on 10.13.37.3
Discovered open port 5963/tcp on 10.13.37.3
Discovered open port 40911/tcp on 10.13.37.3
Discovered open port 8652/tcp on 10.13.37.3
Discovered open port 880/tcp on 10.13.37.3
Discovered open port 10626/tcp on 10.13.37.3
Discovered open port 407/tcp on 10.13.37.3
Discovered open port 12345/tcp on 10.13.37.3
Discovered open port 2401/tcp on 10.13.37.3
Discovered open port 7007/tcp on 10.13.37.3
Discovered open port 4998/tcp on 10.13.37.3
Discovered open port 720/tcp on 10.13.37.3
Discovered open port 7000/tcp on 10.13.37.3
Discovered open port 16993/tcp on 10.13.37.3
Discovered open port 21571/tcp on 10.13.37.3
Discovered open port 5907/tcp on 10.13.37.3
Discovered open port 1687/tcp on 10.13.37.3
Discovered open port 30000/tcp on 10.13.37.3
Discovered open port 1083/tcp on 10.13.37.3
Discovered open port 9000/tcp on 10.13.37.3
Discovered open port 787/tcp on 10.13.37.3
Discovered open port 1113/tcp on 10.13.37.3
Discovered open port 1839/tcp on 10.13.37.3
Discovered open port 17877/tcp on 10.13.37.3
Discovered open port 15003/tcp on 10.13.37.3
Discovered open port 4444/tcp on 10.13.37.3
Discovered open port 30/tcp on 10.13.37.3
Discovered open port 280/tcp on 10.13.37.3
Discovered open port 10003/tcp on 10.13.37.3
Discovered open port 43/tcp on 10.13.37.3
Discovered open port 6346/tcp on 10.13.37.3
Discovered open port 50006/tcp on 10.13.37.3
Discovered open port 3000/tcp on 10.13.37.3
Discovered open port 1087/tcp on 10.13.37.3
Discovered open port 106/tcp on 10.13.37.3
Discovered open port 2160/tcp on 10.13.37.3
Discovered open port 8088/tcp on 10.13.37.3
Discovered open port 8873/tcp on 10.13.37.3
Discovered open port 3369/tcp on 10.13.37.3
Discovered open port 5102/tcp on 10.13.37.3
Discovered open port 27715/tcp on 10.13.37.3
Discovered open port 5298/tcp on 10.13.37.3
Discovered open port 32773/tcp on 10.13.37.3
Discovered open port 163/tcp on 10.13.37.3
Discovered open port 1082/tcp on 10.13.37.3
Discovered open port 1070/tcp on 10.13.37.3
Discovered open port 3869/tcp on 10.13.37.3
Discovered open port 5004/tcp on 10.13.37.3
Discovered open port 13783/tcp on 10.13.37.3
Discovered open port 4321/tcp on 10.13.37.3
Discovered open port 1501/tcp on 10.13.37.3
Discovered open port 2009/tcp on 10.13.37.3
Discovered open port 3878/tcp on 10.13.37.3
Discovered open port 1065/tcp on 10.13.37.3
Discovered open port 1049/tcp on 10.13.37.3
Discovered open port 32769/tcp on 10.13.37.3
Discovered open port 9877/tcp on 10.13.37.3
Discovered open port 19780/tcp on 10.13.37.3
Discovered open port 3703/tcp on 10.13.37.3
Discovered open port 27356/tcp on 10.13.37.3
Discovered open port 1864/tcp on 10.13.37.3
Discovered open port 6566/tcp on 10.13.37.3
Discovered open port 1186/tcp on 10.13.37.3
Discovered open port 1149/tcp on 10.13.37.3
Discovered open port 37/tcp on 10.13.37.3
Discovered open port 8086/tcp on 10.13.37.3
Discovered open port 5101/tcp on 10.13.37.3
Discovered open port 5631/tcp on 10.13.37.3
Discovered open port 3517/tcp on 10.13.37.3
Discovered open port 1947/tcp on 10.13.37.3
Discovered open port 1059/tcp on 10.13.37.3
Discovered open port 5730/tcp on 10.13.37.3
Discovered open port 119/tcp on 10.13.37.3
Discovered open port 2048/tcp on 10.13.37.3
Discovered open port 1079/tcp on 10.13.37.3
Discovered open port 2525/tcp on 10.13.37.3
Discovered open port 5510/tcp on 10.13.37.3
Discovered open port 10629/tcp on 10.13.37.3
Discovered open port 24444/tcp on 10.13.37.3
Discovered open port 6692/tcp on 10.13.37.3
Discovered open port 3659/tcp on 10.13.37.3
Discovered open port 3995/tcp on 10.13.37.3
Discovered open port 1311/tcp on 10.13.37.3
Discovered open port 5666/tcp on 10.13.37.3
Discovered open port 1216/tcp on 10.13.37.3
Discovered open port 1122/tcp on 10.13.37.3
Discovered open port 1503/tcp on 10.13.37.3
Discovered open port 161/tcp on 10.13.37.3
Discovered open port 3689/tcp on 10.13.37.3
Discovered open port 801/tcp on 10.13.37.3
Discovered open port 6005/tcp on 10.13.37.3
Discovered open port 999/tcp on 10.13.37.3
Discovered open port 900/tcp on 10.13.37.3
Discovered open port 1641/tcp on 10.13.37.3
Discovered open port 2399/tcp on 10.13.37.3
Discovered open port 146/tcp on 10.13.37.3
Discovered open port 1700/tcp on 10.13.37.3
Discovered open port 9535/tcp on 10.13.37.3
Discovered open port 1984/tcp on 10.13.37.3
Discovered open port 992/tcp on 10.13.37.3
Discovered open port 1247/tcp on 10.13.37.3
Discovered open port 1089/tcp on 10.13.37.3
Discovered open port 50800/tcp on 10.13.37.3
Discovered open port 1024/tcp on 10.13.37.3
Discovered open port 1974/tcp on 10.13.37.3
Discovered open port 783/tcp on 10.13.37.3
Discovered open port 1174/tcp on 10.13.37.3
Discovered open port 1863/tcp on 10.13.37.3
Discovered open port 3809/tcp on 10.13.37.3
Discovered open port 9110/tcp on 10.13.37.3
Discovered open port 1063/tcp on 10.13.37.3
Discovered open port 2323/tcp on 10.13.37.3
Discovered open port 7106/tcp on 10.13.37.3
Discovered open port 1971/tcp on 10.13.37.3
Discovered open port 3851/tcp on 10.13.37.3
Discovered open port 3325/tcp on 10.13.37.3
Discovered open port 1043/tcp on 10.13.37.3
Discovered open port 981/tcp on 10.13.37.3
Discovered open port 3005/tcp on 10.13.37.3
Discovered open port 51493/tcp on 10.13.37.3
Discovered open port 7443/tcp on 10.13.37.3
Discovered open port 16080/tcp on 10.13.37.3
Discovered open port 1069/tcp on 10.13.37.3
Discovered open port 6002/tcp on 10.13.37.3
Discovered open port 18988/tcp on 10.13.37.3
Discovered open port 4005/tcp on 10.13.37.3
Discovered open port 50636/tcp on 10.13.37.3
Discovered open port 5200/tcp on 10.13.37.3
Discovered open port 3003/tcp on 10.13.37.3
Discovered open port 1801/tcp on 10.13.37.3
Discovered open port 5544/tcp on 10.13.37.3
Discovered open port 9898/tcp on 10.13.37.3
Discovered open port 6788/tcp on 10.13.37.3
Discovered open port 3030/tcp on 10.13.37.3
Discovered open port 2602/tcp on 10.13.37.3
Discovered open port 1138/tcp on 10.13.37.3
Discovered open port 1434/tcp on 10.13.37.3
Discovered open port 10000/tcp on 10.13.37.3
Discovered open port 8082/tcp on 10.13.37.3
Discovered open port 32779/tcp on 10.13.37.3
Discovered open port 5414/tcp on 10.13.37.3
Discovered open port 10010/tcp on 10.13.37.3
Discovered open port 24/tcp on 10.13.37.3
Discovered open port 13782/tcp on 10.13.37.3
Discovered open port 6004/tcp on 10.13.37.3
Discovered open port 20005/tcp on 10.13.37.3
Discovered open port 10009/tcp on 10.13.37.3
Discovered open port 1334/tcp on 10.13.37.3
Discovered open port 9943/tcp on 10.13.37.3
Discovered open port 5925/tcp on 10.13.37.3
Discovered open port 79/tcp on 10.13.37.3
Discovered open port 55056/tcp on 10.13.37.3
Discovered open port 1023/tcp on 10.13.37.3
Discovered open port 15002/tcp on 10.13.37.3
Discovered open port 1972/tcp on 10.13.37.3
Discovered open port 1107/tcp on 10.13.37.3
Discovered open port 13456/tcp on 10.13.37.3
Discovered open port 3828/tcp on 10.13.37.3
Discovered open port 2196/tcp on 10.13.37.3
Discovered open port 5802/tcp on 10.13.37.3
Discovered open port 6567/tcp on 10.13.37.3
Discovered open port 340/tcp on 10.13.37.3
Discovered open port 545/tcp on 10.13.37.3
Discovered open port 5825/tcp on 10.13.37.3
Discovered open port 1076/tcp on 10.13.37.3
Discovered open port 7921/tcp on 10.13.37.3
Discovered open port 20031/tcp on 10.13.37.3
Discovered open port 57797/tcp on 10.13.37.3
Discovered open port 987/tcp on 10.13.37.3
Discovered open port 2001/tcp on 10.13.37.3
Discovered open port 1102/tcp on 10.13.37.3
Discovered open port 1556/tcp on 10.13.37.3
Discovered open port 8291/tcp on 10.13.37.3
Discovered open port 3905/tcp on 10.13.37.3
Discovered open port 1026/tcp on 10.13.37.3
Discovered open port 10025/tcp on 10.13.37.3
Discovered open port 8994/tcp on 10.13.37.3
Discovered open port 6003/tcp on 10.13.37.3
Discovered open port 1072/tcp on 10.13.37.3
Discovered open port 49176/tcp on 10.13.37.3
Discovered open port 1039/tcp on 10.13.37.3
Discovered open port 668/tcp on 10.13.37.3
Discovered open port 691/tcp on 10.13.37.3
Discovered open port 52848/tcp on 10.13.37.3
Discovered open port 50001/tcp on 10.13.37.3
Discovered open port 255/tcp on 10.13.37.3
Discovered open port 23502/tcp on 10.13.37.3
Discovered open port 60020/tcp on 10.13.37.3
Discovered open port 8099/tcp on 10.13.37.3
Discovered open port 33899/tcp on 10.13.37.3
Discovered open port 4125/tcp on 10.13.37.3
Discovered open port 5030/tcp on 10.13.37.3
Discovered open port 1461/tcp on 10.13.37.3
Discovered open port 25735/tcp on 10.13.37.3
Discovered open port 1998/tcp on 10.13.37.3
Discovered open port 1111/tcp on 10.13.37.3
Discovered open port 32771/tcp on 10.13.37.3
Discovered open port 64623/tcp on 10.13.37.3
Discovered open port 34572/tcp on 10.13.37.3
Discovered open port 6106/tcp on 10.13.37.3
Discovered open port 1277/tcp on 10.13.37.3
Discovered open port 3370/tcp on 10.13.37.3
Discovered open port 1165/tcp on 10.13.37.3
Discovered open port 35500/tcp on 10.13.37.3
Discovered open port 17/tcp on 10.13.37.3
Discovered open port 6901/tcp on 10.13.37.3
Discovered open port 1091/tcp on 10.13.37.3
Discovered open port 666/tcp on 10.13.37.3
Discovered open port 9200/tcp on 10.13.37.3
Discovered open port 1035/tcp on 10.13.37.3
Discovered open port 8085/tcp on 10.13.37.3
Discovered open port 3784/tcp on 10.13.37.3
Discovered open port 901/tcp on 10.13.37.3
Discovered open port 5987/tcp on 10.13.37.3
Discovered open port 34571/tcp on 10.13.37.3
Discovered open port 10004/tcp on 10.13.37.3
Discovered open port 2049/tcp on 10.13.37.3
Discovered open port 254/tcp on 10.13.37.3
Discovered open port 5999/tcp on 10.13.37.3
Discovered open port 1112/tcp on 10.13.37.3
Discovered open port 2191/tcp on 10.13.37.3
Discovered open port 264/tcp on 10.13.37.3
Discovered open port 5960/tcp on 10.13.37.3
Discovered open port 6006/tcp on 10.13.37.3
Discovered open port 4550/tcp on 10.13.37.3
Discovered open port 1721/tcp on 10.13.37.3
Discovered open port 3017/tcp on 10.13.37.3
Discovered open port 26/tcp on 10.13.37.3
Discovered open port 18040/tcp on 10.13.37.3
Discovered open port 6156/tcp on 10.13.37.3
Discovered open port 1078/tcp on 10.13.37.3
Discovered open port 2121/tcp on 10.13.37.3
Discovered open port 4006/tcp on 10.13.37.3
Discovered open port 14000/tcp on 10.13.37.3
Discovered open port 1093/tcp on 10.13.37.3
Discovered open port 2047/tcp on 10.13.37.3
Discovered open port 3971/tcp on 10.13.37.3
Discovered open port 1/tcp on 10.13.37.3
Discovered open port 8290/tcp on 10.13.37.3
Discovered open port 631/tcp on 10.13.37.3
Discovered open port 464/tcp on 10.13.37.3
Discovered open port 7937/tcp on 10.13.37.3
Discovered open port 6389/tcp on 10.13.37.3
Discovered open port 7002/tcp on 10.13.37.3
Discovered open port 9878/tcp on 10.13.37.3
Discovered open port 8649/tcp on 10.13.37.3
Discovered open port 3580/tcp on 10.13.37.3
Discovered open port 1088/tcp on 10.13.37.3
Discovered open port 3168/tcp on 10.13.37.3
Discovered open port 3221/tcp on 10.13.37.3
Discovered open port 1812/tcp on 10.13.37.3
Discovered open port 465/tcp on 10.13.37.3
Discovered open port 9593/tcp on 10.13.37.3
Discovered open port 44443/tcp on 10.13.37.3
Discovered open port 6007/tcp on 10.13.37.3
Discovered open port 1145/tcp on 10.13.37.3
Discovered open port 1121/tcp on 10.13.37.3
Discovered open port 4002/tcp on 10.13.37.3
Discovered open port 63331/tcp on 10.13.37
.3
Discovered open port 5405/tcp on 10.13.37.3
Discovered open port 19101/tcp on 10.13.37.3
Discovered open port 1036/tcp on 10.13.37.3
Discovered open port 2003/tcp on 10.13.37.3
Discovered open port 1119/tcp on 10.13.37.3
Completed Connect Scan at 21:01, 8.04s elapsed (1000 total ports)
Initiating Service scan at 21:01
Scanning 708 services on 10.13.37.3
```

# routing traffic to ghostport

```bash
INTERFACE="eth0" # change to your interface
iptables -t nat -A PREROUTING -i $INTERFACE -p tcp -m tcp -m multiport --dports 1:65535 -j REDIRECT --to-ports 8888
```
this will redirect all traffic to ghostport on port 8888 rotating the signatures



# signatures
signatures are in the todo list 
we need to parse regex and give random responses on regex matches


