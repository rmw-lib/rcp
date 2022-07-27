添加 节点 别名
获取 别名 路径

---

请求
  META
  任务编号
    文件路径

响应
  INFO
    任务编号
    新任务编号
    文件大小

  如果文件不存在，就响应
  INFO
    任务编号

请求
  GET
  新任务编号
  MB u32 RoaringBitmap 里面是需要的字节
  请求到了本地，先读取 1MB，然后按需求发送

响应
  DATA
  新任务编号
  MB u32
  数据偏移 u16 (单位是 16 个字节，这样 u16 就可以标识 1MB)
  数据

本地数据结构
  缺少的 MB
    RoaringBitmap
  MB
    pos Box<u8>

---

请求
ip
id
文件路径

响应
文件大小

文件传输协议设计

心跳包

时间
周期内发送的字节数
签名
接收的字节数

---

同步目录日志

目录日志

添加 / 修改
  时间 文件

删除
  时间 文件

目录全部列表

目录 - 路径 - 文件 - 哈希

---

api
  挂文件到路径

rust 创建稀疏文件

use async_std::prelude::*;
use async_std::fs::File;
use async_std::fs::OpenOptions;

let file = OpenOptions::new()
    .write(true)
    .open("a.txt")
    .await?;

file.set_len(10).await?;
file.seek(SeekFrom::Start(3)).await?;
file.write_all(b"Hello, world!").await?;

请求文件

基于 UDP 拥塞控制-LEDBAT - 代码先锋网 : https://t.cn/A6aSyeDi

libutp 源码简析 http://www.calvinneo.com/2017/12/05/libutp%E6%BA%90%E7%A0%81%E7%AE%80%E6%9E%90/

[译] [论文] BBR：基于拥塞（而非丢包）的拥塞控制（ACM, 2017）
https://arthurchiao.art/blog/bbr-paper-zh/

从流量控制算法谈网络优化 – 从 CUBIC 到 BBRv2 算法
https://aws.amazon.com/cn/blogs/china/talking-about-network-optimization-from-the-flow-control-algorithm/

接收文件大小 分片哈希 offset 分片哈希

发包间隔 10 毫秒
发包速度 10
发包速度增速 10
收包速度 0

收包速度 = 收包速度 * （1000-距离上一次时间）/ 1000 + 收包速度*100

addr ->
  struct Sender {
    recv_rate: u64,
    prev_recv_rate: u64,
    send_rate: u64,
    sleep: u16,
    task: vec<reader>
  }

btreemap next_time - addr

每 64 个周期，记录一次收包速度
如果收包 >= 发包，发包速度倍增 (如果间隔 >20，时间减半；否则速度加倍，时间 =10)
否则发包 = 收包速度+1，如果收包速度 <=1 ，发包时间加倍，不超过 1000

文件大小 文件哈希
文件哈希 文件哈希切片
