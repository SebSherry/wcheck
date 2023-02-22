# wcheck - Spell check for programmers

wcheck is a spell checker with programmers in mind. It is designed to work on source code files.  

## How to install

### Prerequisites 

You will a rust compiler and the British or American English word list installed.

Instructions for installing rust can be found [here](https://www.rust-lang.org/tools/install)

The word lists should be installed by default on most Linux systems. However installation can be done with the following:

Debian/Ubuntu: 
```Shell
apt install wbritish
```
Note if you would like to use the American word list, you need to install `wamerican`
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
wcheck file-to-spell-check
```

`wcheck` supports multiple files like so:
```Shell
wcheck file1 file2 ...
```

Traverse directories:
```Shell
wcheck -r dir 
```

Use the American word list:
```Shell
wcheck -A <files> 
```
or 
```Shell
wcheck --american <files> 
```

### Baseline files
When using `wcheck` in a linting context you may wish to create baseline file to ignore certain "intentional mistakes" (such as `idx` as an index variable).
You can generate a baseline file like so: 
```Shell
wcheck --baseline <files> 
```

This generates `.wcheck-baseline` in the current directory. You can specify the name and location of the baseline file like so:
```Shell
wcheck --baseline --baseline-file my-baseline-file <files> 
```

`---baseline-file` also works when running a check:
```Shell
wcheck --baseline-file my-baseline-file <files> 
```

## License
This software is licensed under the MIT License. 

See the [LICENSE](LICENSE)