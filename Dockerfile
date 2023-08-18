# 使用基础镜像
FROM neondatabase/vm-compute-node-v14

# 安装所需的软件包
# RUN apt-get update && \
#     apt-get install -y netcat curl vim jq && \
#     apt-get clean && \
#     rm -rf /var/lib/apt/lists/*

# 将本地文件复制到镜像中
COPY neondatabase-neon /opt/neondatabase-neon/
COPY compute.sh /
COPY entrypoint.sh /
COPY spec.json /
COPY spec.prep.DOCKER.json /

# 赋予文件读写权限
RUN chmod +x /opt/neondatabase-neon && \
    chmod +x /compute.sh && \
    chmod +x /entrypoint.sh && \
    chmod +x /spec.json && \
    chmod +x /spec.prep.DOCKER.json

# 设置容器的启动命令
CMD ["/entrypoint.sh"]

# 设置镜像以root用户运行
USER root
