# Simple PostgreSQL check script

Write-Host "Checking PostgreSQL availability..." -ForegroundColor Yellow

# Check Docker
Write-Host "Checking Docker..." -ForegroundColor Cyan
try {
    $dockerVersion = docker --version
    Write-Host "Docker found: $dockerVersion" -ForegroundColor Green
} catch {
    Write-Host "Docker not found. Install Docker Desktop:" -ForegroundColor Red
    Write-Host "https://docs.docker.com/desktop/windows/install/" -ForegroundColor Yellow
    exit 1
}

# Check docker-compose
Write-Host "Checking docker-compose..." -ForegroundColor Cyan
try {
    $composeVersion = docker-compose --version
    Write-Host "docker-compose found: $composeVersion" -ForegroundColor Green
} catch {
    Write-Host "docker-compose not found" -ForegroundColor Red
    exit 1
}

# Check docker-compose.test.yml file
Write-Host "Checking configuration..." -ForegroundColor Cyan
if (Test-Path "docker-compose.test.yml") {
    Write-Host "docker-compose.test.yml found" -ForegroundColor Green
} else {
    Write-Host "docker-compose.test.yml not found" -ForegroundColor Red
    exit 1
}

# Try to start PostgreSQL
Write-Host "Starting PostgreSQL..." -ForegroundColor Cyan
try {
    docker-compose -f docker-compose.test.yml up -d postgres-test
    Write-Host "PostgreSQL started" -ForegroundColor Green
} catch {
    Write-Host "Failed to start PostgreSQL" -ForegroundColor Red
    Write-Host "Make sure Docker Desktop is running" -ForegroundColor Yellow
    exit 1
}

# Wait for PostgreSQL to be ready
Write-Host "Waiting for PostgreSQL to be ready..." -ForegroundColor Cyan
$timeout = 60
$counter = 0
$ready = $false

do {
    Start-Sleep -Seconds 2
    $counter += 2
    Write-Host "Checking... ($counter/$timeout sec)" -ForegroundColor Gray
    
    try {
        $result = docker-compose -f docker-compose.test.yml exec -T postgres-test pg_isready -U postgres -d test_blockchain 2>$null
        if ($LASTEXITCODE -eq 0) {
            $ready = $true
            Write-Host "PostgreSQL is ready!" -ForegroundColor Green
            break
        }
    } catch {
        # Ignore errors during check
    }
} while ($counter -lt $timeout)

if (-not $ready) {
    Write-Host "PostgreSQL not ready within $timeout seconds" -ForegroundColor Red
    Write-Host "Check logs: docker-compose -f docker-compose.test.yml logs postgres-test" -ForegroundColor Yellow
    exit 1
}

# Test database connection
Write-Host "Testing database connection..." -ForegroundColor Cyan
try {
    $env:DATABASE_URL = "postgresql://postgres:password@localhost:5433/test_blockchain"
    $result = docker-compose -f docker-compose.test.yml exec -T postgres-test psql -U postgres -d test_blockchain -c "SELECT 1;" 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Database connection successful" -ForegroundColor Green
    } else {
        Write-Host "Failed to connect to database" -ForegroundColor Red
    }
} catch {
    Write-Host "Error during connection test" -ForegroundColor Red
}

# Run tests
Write-Host "Running tests with PostgreSQL..." -ForegroundColor Cyan
try {
    $env:DATABASE_URL = "postgresql://postgres:password@localhost:5433/test_blockchain"
    cargo test --test test_database -- --nocapture
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Tests with PostgreSQL passed successfully!" -ForegroundColor Green
    } else {
        Write-Host "Tests with PostgreSQL failed" -ForegroundColor Red
    }
} catch {
    Write-Host "Error running tests" -ForegroundColor Red
}

# Stop PostgreSQL
Write-Host "Stopping PostgreSQL..." -ForegroundColor Cyan
try {
    docker-compose -f docker-compose.test.yml down
    Write-Host "PostgreSQL stopped" -ForegroundColor Green
} catch {
    Write-Host "Failed to stop PostgreSQL" -ForegroundColor Yellow
}

Write-Host "Check completed!" -ForegroundColor Green
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Install Docker Desktop (if not installed)" -ForegroundColor Gray
Write-Host "2. Run: .\scripts\run_tests_with_db.ps1" -ForegroundColor Gray
Write-Host "3. Or manually: docker-compose -f docker-compose.test.yml up -d" -ForegroundColor Gray
