# wcheck - Spell check for programmers

wcheck is a spell checker with programmers in mind. It is designed to work on source code files.  

## How to install

### Prerequisites 

You will a rust compiler and the British English word list installed (American English support coming soon).

Instructions for installing rust can be found [here](https://www.rust-lang.org/tools/install)

The British should be installed by default on most Linux systems. However installation can be done with the following:

Debian/Ubuntu: 
```Shell
apt install wbritish
```

Arch: 
```Shell
pacman -S words
```

Void Linux: 
```Shell
xbps-install words-en
```

Fedora/RHEL: 
```Shell
yum install words  
```

### Building from source:
```Shell
git clone https://github.com/SebSherry/wcheck
cd wcheck
cargo install --path .
cp wcheck-reserved-words /usr/share/dict/
```
Note you may need sudo privileges to copy `wcheck-reserved-words` to `/usr/share/dict/`

## Usage
```Shell
Usage: wcheck [OPTIONS] <FILES>... 

Arguments:  
  <FILES>...  Files to be spell checked

Options:
      --baseline   Generates a baseline file of spelling mistakes to be ignored in future checks   
  -r, --recursive  Recursively search directories for files to check   
  -h, --help       Print help   
  -V, --version    Print version
```

## License
This software is licensed under the MIT License. 

See the [LICENSE](LICENSE)