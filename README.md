# Image_processing_microservice_demo
An image processing microservice application demo wrote in Rust with tonic.

## 项目动机(胡说八道，可跳过)
我需要测试Rust的microservice的性能，但是我没有找到现成的开源项目。我首先尝试用Rust重写C++的microservice项目以及拆分现有的Rust monolithic的项目，发现有一定的困难。由于我所需的测试对app复杂性要求较低，因此简单写了一个图像处理的microservice。

这里的写法完全基于我自己对microservice的理解，可能结构并不算很合适，但是对我的测试足够了。

这里假设每个microservice节点都是分别开发的，因此每个节点单独列为一个项目，其中有一些重复的部分。我认为这是符合microservice场景需求的。

项目具有一定的可扩展性，但是总体来说要改较多地方。TODO：抽象出一个Rust的microservice框架。

项目的依赖关系过于简单，项目的场景也不是很适合microservice架构，但是是可以改进得到一个相对复杂的图像处理应用的。

最初规划时client应该是有一个简单的web界面，配合http协议与server的api gateway交互的。后来发现这样要添加不少胶水，干脆统一用gRPC了。

目前这个架构的service节点在启动时需要保证一定的依赖关系，否则节点启动会失败；但是在启动之后可以动态更新其中的节点。TODO：启动时可以单独启动

主要参考资料：tonic文档和教程，chatgpt

## 运行
1. 首先需要分别启动server中`api_gateway`之外的节点，分别`cd`进入后`cargo run`启动
2. 同样的方式启动`api_gateway`
3. 启动client进行测试

## TODO
- [ ] 添加基于Token的AuthN和AuthZ
- [ ] 在API Gateway中加入负载均衡等功能（应该可以替换成Nginx）
- [ ] README中添加server的依赖关系图
- [ ] 添加存储功能（上传下载图片？）
- [ ] 添加结构，不止单层依赖
- [ ] 重构proto结构，使得请求不单一
- [ ] 更改地址，目前写死本机地址测试用
- [ ] 增加读取配置文件的功能
- [ ] 增加log，至少microservice节点输出点提示...
