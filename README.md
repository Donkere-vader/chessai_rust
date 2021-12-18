# ChessAI

This is the chess algorithm for my High school thesis (profielwerkstuk in dutch).

## Run it

### Set up a python virtual enviroment

```sh
python3 -m venv .env
```

### Activate the enviroment

Max & Linux:

```sh
source .env/bin/activate
```

Windows:

```sh
.env\Scripts\activate
```

### Install dependencies

```sh
pip3 install maturin
```

### Build rust code to python lib

```sh
maturin develop --release
```

### Run the python GUI

```sh
python3 chessai
```
