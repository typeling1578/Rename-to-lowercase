# Rename-to-lowercase

## How to build
### Linux
Native
```
sudo apt install g++
g++ -std=c++17 -O3 --static main.cpp -o rtlc
```
aarch64 (Cross-compile)
```
sudo apt install g++-aarch64-linux-gnu
aarch64-linux-gnu-g++ -std=c++17 -O3 --static main.cpp -o rtlc
```
### Windows
i686 (Cross-compile)
```
sudo apt install g++-mingw-w64-i686
i686-w64-mingw32-g++ -std=c++17 -O3 --static main.cpp -o rtlc.exe
```
x86_64 (Cross-compile)
```
sudo apt install g++-mingw-w64-x86-64
x86_64-w64-mingw32-g++ -std=c++17 -O3 --static main.cpp -o rtlc.exe
```
### Mac
Native
```
g++ -std=c++17 -O3 main.cpp -o rtlc
```
