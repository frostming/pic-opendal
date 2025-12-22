# pic-od

[中文文档](./README_zh.md)

A CLI tool for uploading images to various cloud storage services using [Apache OpenDAL](https://github.com/apache/opendal).

## Features

- Upload images to multiple cloud storage backends (S3, GCS, Azure Blob, OSS, COS, etc.)
- Multiple profile support for different storage targets
- Customizable filename format with template variables
- Environment variable support for CI/CD integration

## Installation

```bash
cargo install pic-od
```

## Configuration

Create a configuration file at `~/.config/pic-od/config.toml`:

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

### Supported Storage Backends

- `s3` - Amazon S3 and compatible services
- `gcs` - Google Cloud Storage
- `azblob` - Azure Blob Storage
- `oss` - Aliyun OSS
- `cos` - Tencent COS
- `obs` - Huawei OBS
- `fs` - Local filesystem
- `webdav` - WebDAV
- And more...

### Filename Format Variables

- `{name}` - Original filename with extension
- `{stem}` - Original filename without extension
- `{ext}` - File extension
- `{date}` - Current date (YYYYMMDD)

## Usage

### Upload images

```bash
# Upload single image
pic-od upload image.png

# Upload multiple images
pic-od upload image1.png image2.jpg image3.gif

# Use a specific profile
pic-od upload -t backup image.png
pic-od upload --target backup image.png

# Use environment variable
PIC_OD_TARGET=backup pic-od upload image.png
```

### Manage profiles

```bash
# List all profiles (* indicates current)
pic-od list

# Set current profile
pic-od target backup
```

## Integrations

### Typora

1. Open Typora Preferences → Image
2. Choose "Custom Command" as Image Uploader
3. Set command to: `pic-od upload`

Now when you insert an image, Typora will use `pic-od` to upload it and insert the URL.

## License

Apache-2.0
