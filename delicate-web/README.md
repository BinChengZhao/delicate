
<p align="center">
  <a href="https://github.com/BinChengZhao/delicate">
    <img alt="delicate" height="64" src="../doc/delicate_logo.png">
  </a>
</p>

<h1 align="center">delicate</h1>

<div align="center">

`delicate` A lightweight and distributed task scheduling platform written in rust & js.

The front-end project of `delicate` is based on `antd-admin` development, thank you very much guys.

[![MIT](https://img.shields.io/dub/l/vibe-d.svg?style=flat-square)](http://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/BinChengZhao/delicate/pulls)

</div>


English | [简体中文](./docs/zh-cn/README.md) 



## Usage

1. Clone project code.

```bash
git clone https://github.com/BinChengZhao/delicate.git

cd delicate/delicate-web
```

2. Installation dependence.

```bash
sudo npm install --global yarn
sudo yarn global add umi
sudo yarn install
sudo yarn build
```

3. Deploy:
Next, we can upload the static file to the server. If you use Nginx as the Web server, you can configure it in `ngnix.conf`:
```
server
	{
		listen       80;

        # Specify an accessible domain name
		server_name web.delicate-rs.com;

        # The directory where the compiled files are stored
		root  /home/www/delicate-web/dist;

        # Proxy delicate-scheduler server .
		location /api {
            proxy_set_header   X-Forwarded-For $remote_addr;
            proxy_set_header   Host $http_host;
            proxy_pass         http://*.*.*.*:8090;
        }

        # Because the front end uses BrowserHistory, it will 
		# route back to index.html
		location / {
				index  index.html;
				try_files $uri $uri/ /index.html;
		}
	}
```

4. After the startup is complete, open a browser and visit [http://yourdomain.com](http://yourdomain.com), If you need to change the startup port, you can configure it in the `.env` file, And you can set the backend request address in `delicate-web/src/utils/envConfig.js`.



## Browsers support

Modern browsers.

| [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/edge/edge_48x48.png" alt="IE / Edge" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>IE / Edge | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/firefox/firefox_48x48.png" alt="Firefox" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Firefox | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/chrome/chrome_48x48.png" alt="Chrome" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Chrome | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/safari/safari_48x48.png" alt="Safari" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Safari | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/opera/opera_48x48.png" alt="Opera" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Opera |
| --------- | --------- | --------- | --------- | --------- | 
|IE11, Edge| last 2 versions| last 2 versions| last 2 versions| last 2 versions

## Contributing

We very much welcome your contribution, you can build together with us in the following ways :)  

- Use `delicate` in your daily work.
- Submit [GitHub issues](http://github.com/BinChengZhao/delicate/issues) to report bugs or ask questions.
- Propose [Pull Request](http://github.com/BinChengZhao/delicate/pulls) to improve our code.
