# MyWebnote Build Docs

## Deploy with Web Service

- Build Static Assets

```bash
node -v            
#v18.17.1

sudo npm install -g typescript yarn
yarn install --registry=https://registry.npmmirror.com
yarn electron-vite build
# OUTPUT: ./out/renderer/
```

- Start static serve

```bash
python3 -m http.server --directory out/renderer/ 8888
```

- Accessing: [http://localhost:8888/](http://localhost:8888/)
