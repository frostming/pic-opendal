# pic-od

[English](./README.md)

基于 [Apache OpenDAL](https://github.com/apache/opendal) 的图片上传命令行工具。

## 特性

- 支持多种云存储后端（S3、GCS、Azure Blob、OSS、COS 等）
- 多配置文件支持，可配置不同的存储目标
- 可自定义文件名格式，支持模板变量
- 支持环境变量，方便 CI/CD 集成

## 安装

```bash
cargo install pic-od
```

## 配置

在 `~/.config/pic-od/config.toml` 创建配置文件：

```toml
current_profile = "default"

[profiles.default]
type = "s3"
bucket = "my-bucket"
region = "us-east-1"
access_key_id = "YOUR_ACCESS_KEY"
secret_access_key = "YOUR_SECRET_KEY"
root = "/images"
base_url = "https://cdn.example.com"
filename_format = "{date}/{stem}.{ext}"

[profiles.backup]
type = "gcs"
bucket = "backup-bucket"
credential_path = "/path/to/credentials.json"
base_url = "https://storage.googleapis.com/backup-bucket"
```

### 支持的存储后端

- `s3` - Amazon S3 及兼容服务
- `gcs` - Google Cloud Storage
- `azblob` - Azure Blob Storage
- `oss` - 阿里云 OSS
- `cos` - 腾讯云 COS
- `obs` - 华为云 OBS
- `fs` - 本地文件系统
- `webdav` - WebDAV
- 更多...

### 文件名格式变量

- `{name}` - 原文件名（含扩展名）
- `{stem}` - 原文件名（不含扩展名）
- `{ext}` - 文件扩展名
- `{date}` - 当前日期（YYYYMMDD）

## 使用方法

### 上传图片

```bash
# 上传单个图片
pic-od upload image.png

# 上传多个图片
pic-od upload image1.png image2.jpg image3.gif

# 使用指定配置
pic-od upload -t backup image.png
pic-od upload --profile backup image.png

# 使用环境变量
PIC_OD_PROFILE=backup pic-od upload image.png
```

### 管理配置

```bash
# 列出所有配置（* 表示当前使用的配置）
pic-od list

# 设置当前配置
pic-od profile backup
```

## 集成

### Typora

1. 打开 Typora 偏好设置 → 图像
2. 图像上传服务选择「自定义命令」
3. 命令设置为：`pic-od upload`

现在插入图片时，Typora 会自动使用 `pic-od` 上传并插入 URL。

## 许可证

Apache-2.0
