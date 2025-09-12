# 🚀 DevOps Guide - The Hot Pot Spot

## Содержание
1. [Инфраструктура](#инфраструктура)
2. [Контейнеризация](#контейнеризация)
3. [CI/CD Pipeline](#cicd-pipeline)
4. [Мониторинг и логирование](#мониторинг-и-логирование)
5. [Безопасность](#безопасность)
6. [Масштабирование](#масштабирование)
7. [Резервное копирование](#резервное-копирование)
8. [Деплой](#деплой)
9. [Troubleshooting](#troubleshooting)
10. [Автоматизация](#автоматизация)

---

## 🏗 Инфраструктура

### Архитектура инфраструктуры

```mermaid
graph TB
    subgraph "Load Balancer"
        LB[Nginx/HAProxy]
    end
    
    subgraph "Application Layer"
        API1[API Server 1]
        API2[API Server 2]
        API3[API Server 3]
    end
    
    subgraph "Blockchain Layer"
        NODE1[Blockchain Node 1]
        NODE2[Blockchain Node 2]
        NODE3[Blockchain Node 3]
    end
    
    subgraph "Data Layer"
        POSTGRES[(PostgreSQL Primary)]
        POSTGRES_REPLICA[(PostgreSQL Replica)]
        REDIS[(Redis Cache)]
    end
    
    subgraph "Storage Layer"
        IPFS[IPFS Cluster]
        S3[Object Storage]
    end
    
    subgraph "Monitoring"
        PROMETHEUS[Prometheus]
        GRAFANA[Grafana]
        ELK[ELK Stack]
    end
    
    LB --> API1
    LB --> API2
    LB --> API3
    
    API1 --> NODE1
    API2 --> NODE2
    API3 --> NODE3
    
    API1 --> POSTGRES
    API2 --> POSTGRES
    API3 --> POSTGRES
    
    NODE1 --> IPFS
    NODE2 --> IPFS
    NODE3 --> IPFS
    
    API1 --> REDIS
    API2 --> REDIS
    API3 --> REDIS
```

### Требования к инфраструктуре

#### Минимальные требования
- **CPU**: 4 cores
- **RAM**: 8GB
- **Storage**: 100GB SSD
- **Network**: 1Gbps

#### Рекомендуемые требования
- **CPU**: 8+ cores
- **RAM**: 16GB+
- **Storage**: 500GB+ NVMe SSD
- **Network**: 10Gbps

### Облачные провайдеры

#### AWS
```yaml
# terraform/aws/main.tf
provider "aws" {
  region = "eu-west-1"
}

resource "aws_instance" "api_server" {
  ami           = "ami-0c55b159cbfafe1d0"
  instance_type = "t3.large"
  
  vpc_security_group_ids = [aws_security_group.api.id]
  subnet_id              = aws_subnet.private.id
  
  user_data = file("user_data.sh")
  
  tags = {
    Name = "thehotpotspot-api"
    Environment = "production"
  }
}
```

#### Google Cloud
```yaml
# terraform/gcp/main.tf
provider "google" {
  project = "thehotpotspot"
  region  = "europe-west1"
}

resource "google_compute_instance" "api_server" {
  name         = "api-server"
  machine_type = "e2-standard-2"
  zone         = "europe-west1-a"
  
  boot_disk {
    initialize_params {
      image = "ubuntu-2004-lts"
      size  = 100
    }
  }
  
  network_interface {
    network = "default"
    access_config {}
  }
}
```

---

## 🐳 Контейнеризация

### Dockerfile
```dockerfile
# Dockerfile
FROM rust:1.70-slim as builder

# Установка зависимостей
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Копирование кода
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Сборка
RUN cargo build --release

# Production образ
FROM debian:bullseye-slim

# Установка runtime зависимостей
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Копирование бинарника
COPY --from=builder /app/target/release/web_server /usr/local/bin/web_server

# Создание пользователя
RUN useradd -r -s /bin/false appuser
USER appuser

# Экспорт портов
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Запуск
CMD ["web_server"]
```

### Docker Compose
```yaml
# docker-compose.yml
version: '3.8'

services:
  postgres:
    image: postgres:13
    environment:
      POSTGRES_DB: thehotpotspot
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./scripts/init.sql:/docker-entrypoint-initdb.d/init.sql
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

  api:
    build: .
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgresql://postgres:${POSTGRES_PASSWORD}@postgres:5432/thehotpotspot
      REDIS_URL: redis://redis:6379
      RUST_LOG: info
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - api

volumes:
  postgres_data:
  redis_data:
```

### Kubernetes манифесты
```yaml
# k8s/api-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-server
  labels:
    app: api-server
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api-server
  template:
    metadata:
      labels:
        app: api-server
    spec:
      containers:
      - name: api-server
        image: thehotpotspot/api:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: database-secret
              key: url
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: api-service
spec:
  selector:
    app: api-server
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
  type: LoadBalancer
```

---

## 🔄 CI/CD Pipeline

### GitHub Actions
```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:13
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: test_db
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432

    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run tests
      run: cargo test --verbose
      env:
        DATABASE_URL: postgresql://postgres:postgres@localhost:5432/test_db
    
    - name: Run clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
    
    - name: Run rustfmt
      run: cargo fmt --all -- --check

  security:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Run cargo audit
      run: cargo install cargo-audit && cargo audit

  build:
    needs: [test, security]
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v2
    
    - name: Login to Docker Hub
      uses: docker/login-action@v2
      with:
        username: ${{ secrets.DOCKER_USERNAME }}
        password: ${{ secrets.DOCKER_PASSWORD }}
    
    - name: Build and push Docker image
      uses: docker/build-push-action@v3
      with:
        context: .
        push: true
        tags: |
          thehotpotspot/api:latest
          thehotpotspot/api:${{ github.sha }}
        cache-from: type=gha
        cache-to: type=gha,mode=max

  deploy:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Configure kubectl
      uses: azure/k8s-set-context@v3
      with:
        method: kubeconfig
        kubeconfig: ${{ secrets.KUBE_CONFIG }}
    
    - name: Deploy to Kubernetes
      run: |
        kubectl set image deployment/api-server api-server=thehotpotspot/api:${{ github.sha }}
        kubectl rollout status deployment/api-server
        kubectl rollout status deployment/api-server --timeout=600s
```

### GitLab CI
```yaml
# .gitlab-ci.yml
stages:
  - test
  - build
  - deploy

variables:
  CARGO_TERM_COLOR: always
  DOCKER_DRIVER: overlay2

test:
  stage: test
  image: rust:1.70
  services:
    - postgres:13
  variables:
    POSTGRES_DB: test_db
    POSTGRES_USER: postgres
    POSTGRES_PASSWORD: postgres
    DATABASE_URL: postgresql://postgres:postgres@postgres:5432/test_db
  before_script:
    - apt-get update -qq && apt-get install -y -qq libpq-dev
  script:
    - cargo test --verbose
    - cargo clippy --all-targets --all-features -- -D warnings
    - cargo fmt --all -- --check
  cache:
    paths:
      - target/
      - .cargo/

build:
  stage: build
  image: docker:latest
  services:
    - docker:dind
  before_script:
    - docker login -u $CI_REGISTRY_USER -p $CI_REGISTRY_PASSWORD $CI_REGISTRY
  script:
    - docker build -t $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA .
    - docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA
  only:
    - main

deploy:
  stage: deploy
  image: bitnami/kubectl:latest
  script:
    - kubectl set image deployment/api-server api-server=$CI_REGISTRY_IMAGE:$CI_COMMIT_SHA
    - kubectl rollout status deployment/api-server
  only:
    - main
```

---

## 📊 Мониторинг и логирование

### Prometheus конфигурация
```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "rules/*.yml"

scrape_configs:
  - job_name: 'thehotpotspot-api'
    static_configs:
      - targets: ['api-server:8080']
    metrics_path: /metrics
    scrape_interval: 5s

  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter:9187']

  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter:9121']

  - job_name: 'node-exporter'
    static_configs:
      - targets: ['node-exporter:9100']

alerting:
  alertmanagers:
    - static_configs:
        - targets:
          - alertmanager:9093
```

### Grafana дашборды
```json
{
  "dashboard": {
    "title": "The Hot Pot Spot - API Metrics",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])",
            "legendFormat": "{{method}} {{path}}"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          }
        ]
      },
      {
        "title": "Error Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total{status=~\"5..\"}[5m])",
            "legendFormat": "5xx errors"
          }
        ]
      }
    ]
  }
}
```

### ELK Stack конфигурация
```yaml
# docker-compose.monitoring.yml
version: '3.8'

services:
  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:7.15.0
    environment:
      - discovery.type=single-node
      - "ES_JAVA_OPTS=-Xms512m -Xmx512m"
    ports:
      - "9200:9200"
    volumes:
      - elasticsearch_data:/usr/share/elasticsearch/data

  logstash:
    image: docker.elastic.co/logstash/logstash:7.15.0
    volumes:
      - ./logstash.conf:/usr/share/logstash/pipeline/logstash.conf
    ports:
      - "5044:5044"
    depends_on:
      - elasticsearch

  kibana:
    image: docker.elastic.co/kibana/kibana:7.15.0
    ports:
      - "5601:5601"
    environment:
      - ELASTICSEARCH_HOSTS=http://elasticsearch:9200
    depends_on:
      - elasticsearch

  filebeat:
    image: docker.elastic.co/beats/filebeat:7.15.0
    user: root
    volumes:
      - ./filebeat.yml:/usr/share/filebeat/filebeat.yml
      - /var/lib/docker/containers:/var/lib/docker/containers:ro
      - /var/run/docker.sock:/var/run/docker.sock:ro
    depends_on:
      - logstash

volumes:
  elasticsearch_data:
```

### Алерты
```yaml
# rules/alerts.yml
groups:
- name: thehotpotspot
  rules:
  - alert: HighErrorRate
    expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High error rate detected"
      description: "Error rate is {{ $value }} errors per second"

  - alert: HighResponseTime
    expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 1
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High response time"
      description: "95th percentile response time is {{ $value }}s"

  - alert: DatabaseConnectionFailed
    expr: up{job="postgres"} == 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "Database connection failed"
      description: "PostgreSQL is down"
```

---

## 🔒 Безопасность

### SSL/TLS конфигурация
```nginx
# nginx.conf
server {
    listen 443 ssl http2;
    server_name api.thehotpotspot.com;
    
    ssl_certificate /etc/nginx/ssl/cert.pem;
    ssl_certificate_key /etc/nginx/ssl/key.pem;
    
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;
    
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    
    location / {
        proxy_pass http://api-server;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Kubernetes Network Policies
```yaml
# k8s/network-policy.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: api-network-policy
spec:
  podSelector:
    matchLabels:
      app: api-server
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: nginx
    ports:
    - protocol: TCP
      port: 8080
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: postgres
    ports:
    - protocol: TCP
      port: 5432
  - to:
    - podSelector:
        matchLabels:
          app: redis
    ports:
    - protocol: TCP
      port: 6379
```

### Secrets управление
```yaml
# k8s/secrets.yaml
apiVersion: v1
kind: Secret
metadata:
  name: database-secret
type: Opaque
data:
  url: cG9zdGdyZXNxbDovL3VzZXI6cGFzc0Bwb3N0Z3Jlczo1NDMyL2Ri  # base64 encoded
  password: cGFzc3dvcmQ=  # base64 encoded

---
apiVersion: v1
kind: Secret
metadata:
  name: api-secrets
type: Opaque
data:
  jwt-secret: c2VjcmV0LWtleQ==  # base64 encoded
  encryption-key: ZW5jcnlwdGlvbi1rZXk=  # base64 encoded
```

---

## 📈 Масштабирование

### Горизонтальное масштабирование
```yaml
# k8s/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: api-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-server
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### Вертикальное масштабирование
```yaml
# k8s/vpa.yaml
apiVersion: autoscaling.k8s.io/v1
kind: VerticalPodAutoscaler
metadata:
  name: api-vpa
spec:
  targetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-server
  updatePolicy:
    updateMode: "Auto"
  resourcePolicy:
    containerPolicies:
    - containerName: api-server
      minAllowed:
        cpu: 100m
        memory: 128Mi
      maxAllowed:
        cpu: 1000m
        memory: 1Gi
```

### Database масштабирование
```yaml
# k8s/postgres-cluster.yaml
apiVersion: postgresql.cnpg.io/v1
kind: Cluster
metadata:
  name: postgres-cluster
spec:
  instances: 3
  
  postgresql:
    parameters:
      max_connections: "200"
      shared_buffers: "256MB"
      effective_cache_size: "1GB"
  
  bootstrap:
    initdb:
      database: thehotpotspot
      owner: app_user
      secret:
        name: postgres-credentials
  
  storage:
    size: 100Gi
    storageClass: fast-ssd
  
  monitoring:
    enabled: true
```

---

## 💾 Резервное копирование

### Database бэкапы
```bash
#!/bin/bash
# scripts/backup-database.sh

BACKUP_DIR="/backups/postgres"
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="thehotpotspot_${DATE}.sql"

# Создание бэкапа
pg_dump -h postgres -U postgres thehotpotspot > "${BACKUP_DIR}/${BACKUP_FILE}"

# Сжатие
gzip "${BACKUP_DIR}/${BACKUP_FILE}"

# Загрузка в S3
aws s3 cp "${BACKUP_DIR}/${BACKUP_FILE}.gz" s3://thehotpotspot-backups/database/

# Очистка старых бэкапов (старше 30 дней)
find ${BACKUP_DIR} -name "*.gz" -mtime +30 -delete

echo "Backup completed: ${BACKUP_FILE}.gz"
```

### Kubernetes CronJob для бэкапов
```yaml
# k8s/backup-cronjob.yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: database-backup
spec:
  schedule: "0 2 * * *"  # Каждый день в 2:00
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: postgres:13
            command:
            - /bin/bash
            - -c
            - |
              pg_dump -h postgres -U postgres thehotpotspot | gzip > /backup/backup_$(date +%Y%m%d_%H%M%S).sql.gz
              aws s3 cp /backup/*.gz s3://thehotpotspot-backups/database/
            env:
            - name: PGPASSWORD
              valueFrom:
                secretKeyRef:
                  name: postgres-credentials
                  key: password
            volumeMounts:
            - name: backup-storage
              mountPath: /backup
          volumes:
          - name: backup-storage
            emptyDir: {}
          restartPolicy: OnFailure
```

### Восстановление из бэкапа
```bash
#!/bin/bash
# scripts/restore-database.sh

BACKUP_FILE=$1
if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: $0 <backup_file>"
    exit 1
fi

# Скачивание бэкапа из S3
aws s3 cp "s3://thehotpotspot-backups/database/${BACKUP_FILE}" /tmp/

# Распаковка
gunzip "/tmp/${BACKUP_FILE}"

# Восстановление
psql -h postgres -U postgres thehotpotspot < "/tmp/${BACKUP_FILE%.gz}"

echo "Database restored from ${BACKUP_FILE}"
```

---

## 🚀 Деплой

### Blue-Green деплой
```bash
#!/bin/bash
# scripts/blue-green-deploy.sh

NEW_VERSION=$1
CURRENT_COLOR=$(kubectl get service api-service -o jsonpath='{.spec.selector.color}')

if [ "$CURRENT_COLOR" = "blue" ]; then
    NEW_COLOR="green"
else
    NEW_COLOR="blue"
fi

echo "Deploying version ${NEW_VERSION} to ${NEW_COLOR} environment"

# Деплой новой версии
kubectl set image deployment/api-server-${NEW_COLOR} api-server=thehotpotspot/api:${NEW_VERSION}
kubectl rollout status deployment/api-server-${NEW_COLOR}

# Переключение трафика
kubectl patch service api-service -p "{\"spec\":{\"selector\":{\"color\":\"${NEW_COLOR}\"}}}"

# Проверка здоровья
sleep 30
if curl -f http://api.thehotpotspot.com/health; then
    echo "Deployment successful"
    # Остановка старой версии
    kubectl scale deployment api-server-${CURRENT_COLOR} --replicas=0
else
    echo "Deployment failed, rolling back"
    kubectl patch service api-service -p "{\"spec\":{\"selector\":{\"color\":\"${CURRENT_COLOR}\"}}}"
    exit 1
fi
```

### Canary деплой
```yaml
# k8s/canary-deployment.yaml
apiVersion: argoproj.io/v1alpha1
kind: Rollout
metadata:
  name: api-rollout
spec:
  replicas: 5
  strategy:
    canary:
      steps:
      - setWeight: 20
      - pause: {duration: 10m}
      - setWeight: 40
      - pause: {duration: 10m}
      - setWeight: 60
      - pause: {duration: 10m}
      - setWeight: 80
      - pause: {duration: 10m}
      canaryService: api-canary
      stableService: api-stable
      trafficRouting:
        nginx:
          stableIngress: api-ingress
  selector:
    matchLabels:
      app: api-server
  template:
    metadata:
      labels:
        app: api-server
    spec:
      containers:
      - name: api-server
        image: thehotpotspot/api:latest
        ports:
        - containerPort: 8080
```

---

## 🔧 Troubleshooting

### Общие проблемы

#### Высокое использование CPU
```bash
# Проверка процессов
kubectl top pods

# Логи приложения
kubectl logs -f deployment/api-server

# Профилирование
kubectl exec -it deployment/api-server -- perf record -g -p 1
```

#### Проблемы с памятью
```bash
# Проверка использования памяти
kubectl top pods --containers

# Анализ heap
kubectl exec -it deployment/api-server -- curl http://localhost:8080/debug/pprof/heap
```

#### Проблемы с сетью
```bash
# Проверка сетевых политик
kubectl get networkpolicies

# Тестирование соединения
kubectl exec -it deployment/api-server -- nc -zv postgres 5432
```

### Мониторинг логов
```bash
# Просмотр логов в реальном времени
kubectl logs -f deployment/api-server

# Логи с фильтрацией
kubectl logs deployment/api-server | grep ERROR

# Логи за определенный период
kubectl logs deployment/api-server --since=1h
```

### Отладка производительности
```bash
# Профилирование CPU
kubectl exec -it deployment/api-server -- curl http://localhost:8080/debug/pprof/profile

# Профилирование памяти
kubectl exec -it deployment/api-server -- curl http://localhost:8080/debug/pprof/heap

# Трассировка
kubectl exec -it deployment/api-server -- curl http://localhost:8080/debug/pprof/trace
```

---

## 🤖 Автоматизация

### Terraform для инфраструктуры
```hcl
# terraform/main.tf
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
}

provider "kubernetes" {
  host                   = aws_eks_cluster.main.endpoint
  cluster_ca_certificate = base64decode(aws_eks_cluster.main.certificate_authority[0].data)
  token                  = data.aws_eks_cluster_auth.main.token
}

# EKS кластер
resource "aws_eks_cluster" "main" {
  name     = "thehotpotspot"
  role_arn = aws_iam_role.eks_cluster.arn
  version  = "1.24"

  vpc_config {
    subnet_ids = aws_subnet.private[*].id
  }
}

# Node group
resource "aws_eks_node_group" "main" {
  cluster_name    = aws_eks_cluster.main.name
  node_group_name = "main"
  node_role_arn   = aws_iam_role.eks_node.arn
  subnet_ids      = aws_subnet.private[*].id

  scaling_config {
    desired_size = 3
    max_size     = 10
    min_size     = 1
  }

  instance_types = ["t3.medium"]
}
```

### Ansible для конфигурации
```yaml
# ansible/playbook.yml
---
- hosts: all
  become: yes
  tasks:
    - name: Update system packages
      apt:
        update_cache: yes
        upgrade: yes

    - name: Install Docker
      apt:
        name: docker.io
        state: present

    - name: Install kubectl
      shell: |
        curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
        chmod +x kubectl
        mv kubectl /usr/local/bin/

    - name: Configure Docker daemon
      copy:
        content: |
          {
            "log-driver": "json-file",
            "log-opts": {
              "max-size": "10m",
              "max-file": "3"
            }
          }
        dest: /etc/docker/daemon.json
      notify: restart docker

  handlers:
    - name: restart docker
      systemd:
        name: docker
        state: restarted
```

### Helm чарты
```yaml
# helm/thehotpotspot/values.yaml
replicaCount: 3

image:
  repository: thehotpotspot/api
  tag: latest
  pullPolicy: IfNotPresent

service:
  type: ClusterIP
  port: 80
  targetPort: 8080

ingress:
  enabled: true
  className: nginx
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
  hosts:
    - host: api.thehotpotspot.com
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: api-tls
      hosts:
        - api.thehotpotspot.com

resources:
  limits:
    cpu: 500m
    memory: 512Mi
  requests:
    cpu: 250m
    memory: 256Mi

autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 10
  targetCPUUtilizationPercentage: 70
  targetMemoryUtilizationPercentage: 80

postgresql:
  enabled: true
  auth:
    postgresPassword: "postgres"
    database: "thehotpotspot"
  primary:
    persistence:
      size: 100Gi
```

---

## 📋 Чек-лист деплоя

### Pre-deployment
- [ ] Код прошел все тесты
- [ ] Security scan завершен
- [ ] Документация обновлена
- [ ] Бэкапы созданы
- [ ] Мониторинг настроен

### Deployment
- [ ] Инфраструктура готова
- [ ] Secrets обновлены
- [ ] Конфигурация проверена
- [ ] Health checks настроены
- [ ] Rollback план готов

### Post-deployment
- [ ] Приложение отвечает
- [ ] Метрики в норме
- [ ] Логи без ошибок
- [ ] Пользователи уведомлены
- [ ] Документация обновлена

---

## 🎯 Заключение

Этот DevOps guide покрывает все аспекты развертывания и поддержки The Hot Pot Spot в продакшн среде. Следуя этим практикам, вы обеспечите надежную, масштабируемую и безопасную работу системы.

Для получения дополнительной помощи обращайтесь к команде DevOps или создавайте issues в репозитории.
