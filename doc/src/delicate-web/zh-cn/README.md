<p align="center">
  <a href="http://github.com/BinChengZhao/delicate">
    <img alt="delicate-rs" height="64" src="../_media/logo.png">
  </a>
</p>

<h1 align="center"> Delicate </h1>

<div align="center">

一个轻量的分布式的任务调度平台通过 rust & react 编写.

`delicate` 的前端项目是基于 "antd-admin "开发的，非常感谢各位。


[![GitHub issues](https://img.shields.io/github/issues/BinChengZhao/delicate.svg?style=flat-square)](https://github.com/BinChengZhao/delicate/issues)
[![Build](https://github.com/BinChengZhao/delicate/workflows/CI/badge.svg)](
https://github.com/BinChengZhao/delicate/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](
https://github.com/BinChengZhao/delicate)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/BinChengZhao/delicate/pulls)

</div>


## 前端特性

- 国际化，源码中抽离翻译字段，按需加载语言包
- 动态权限，不同权限对应不同菜单
- 优雅美观，Ant Design 设计体系
- Mock 数据，本地数据调试


## 使用

1. 下载项目代码。

```bash
git clone https://github.com/BinChengZhao/delicate.git my-project
cd my-project
```

2. 进入目录安装依赖，国内用户推荐使用 [cnpm](https://cnpmjs.org) 进行加速。

```bash
sudo npm install --global yarn
sudo yarn global add umi
sudo yarn install
sudo yarn build
```


3. 部署:
接下来，我们可以将静态文件上传到服务器。如果你使用Nginx作为Web服务器，你可以在`ngnix.conf`中配置它:

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

4. 启动完成后，打开浏览器，访问[http://yourdomain.com](http://yourdomain.com)，如果你需要改变启动端口，可以在`.env`文件中配置，而且你可以在`delicate-web/src/utils/envConfig.js`中设置`delicate-scheduler`服务端的请求地址。.




> 更多信息请参考 。

## 支持环境

现代浏览器。

| [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/edge/edge_48x48.png" alt="IE / Edge" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>IE / Edge | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/firefox/firefox_48x48.png" alt="Firefox" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Firefox | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/chrome/chrome_48x48.png" alt="Chrome" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Chrome | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/safari/safari_48x48.png" alt="Safari" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Safari | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/opera/opera_48x48.png" alt="Opera" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Opera |
| --------- | --------- | --------- | --------- | --------- | 
|IE11, Edge| last 2 versions| last 2 versions| last 2 versions| last 2 versions

## 参与贡献  

我们非常欢迎你的贡献，你可以通过以下方式和我们一起共建 :smiley:
- 在你的公司或个人项目中使用 AntD Admin。
- 通过 [Issue](http://github.com/BinChengZhao/delicate/issues) 报告 bug 或进行咨询。
- 提交 [Pull Request](http://github.com/BinChengZhao/delicate/pulls) 改进代码。

> 强烈推荐阅读 [《提问的智慧》](https://github.com/ryanhanwu/How-To-Ask-Questions-The-Smart-Way)、[《如何向开源社区提问题》](https://github.com/seajs/seajs/issues/545) 和 [《如何有效地报告 Bug》](http://www.chiark.greenend.org.uk/%7Esgtatham/bugs-cn.html)、[《如何向开源项目提交无法解答的问题》](https://zhuanlan.zhihu.com/p/25795393)，更好的问题更容易获得帮助。