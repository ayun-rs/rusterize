### 开始

```bash
cp -r .docker/ ./
```

### docker

- build

```bash
docker build -t rust-docker:1.80.0 .
```

- run

```bash
docker run -p 3000:3000 --name=rust-docker-server-1 rust-docker:1.80.0
```

- clean

```bash
docker rm rust-docker:1.80.0
docker rmi -f rust-docker:1.80.0
```

- status

```bash
docker ps -a
```

### docker-compose

- .env

```text
RUST_VERSION=1.80.0
APP_NAME=rust-docker
APP_PORT=3000
```

注意: `APP_NAME` 是项目所在目录/文件夹的名称，也是默认构建后最终执行文件的名称。`APP_NAME` 必须与项目目录/文件夹的名称保持一致，或与`Cargo.toml`文件中的`[[bin]] name`保持一致。

- build

``bash
docker-compose build --build-arg RUST_VERSION=1.80.0 --build-arg APP_NAME=rust-docker --build-arg APP_PORT=3000
``

- up

```bash
docker-compose up
```

- clean

```bash
docker-compose down --rmi local
```