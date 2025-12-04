# START SCRIPT FOR THE WHOLE TICKETING STACK

# Constants:

$DevDir = "dev"
$MigrationsDir = "extra_migrations/fang"
$SampleEnvFile = ".env.sample"
$EnvFile = ".env"
$ComposeEnvFile = "$PSScriptRoot/$DevDir/.docker-compose.env"
$ExpectedNumberOfContainers = 10

# Reusable functions:

function Test-Command()
{
    param (
        $Command
    )
    Get-Command $Command -ErrorAction SilentlyContinue
    return $?
}

function Test-Env()
{
    param (
        $EnvVar
    )
    return ([System.Environment]::GetEnvironmentVariable($EnvVar).Length -gt 0)
}

# Checking requirements:

Write-Host "Welcome to " -ForegroundColor White -NoNewline
Write-Host "Ticketing in Rust" -ForegroundColor Blue
Start-Sleep -Seconds 2

Write-Host "Checking requirements..." -ForegroundColor White

if (!(Test-Command "docker"))
{
    Write-Host "FATAL" -ForegroundColor Red -NoNewline
    Write-Host ": Docker could not be found (try installing it)"
    exit 1
}
else
{
    Write-Host "Docker found ✅"
}

if (!(Test-Command "cargo"))
{
    Write-Host "FATAL" -ForegroundColor Red -NoNewline
    Write-Host ": Cargo could not be found (try to install Rust toolchain)"
    exit 1
}
else
{
    Write-Host "Cargo found ✅"
}

if (!(Test-Command "diesel"))
{
    Write-Host "Diesel could not be found " -NoNewline
    Write-Host "❌" -ForegroundColor Red
    Write-Host "Trying to install Diesel... "
    if (!(cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"))
    {
        Write-Host "FAILED" -ForegroundColor Red
        exit 1
    }
    if (!(Test-Command "diesel"))
    {
        Write-Host "FAILED" -ForegroundColor Red
        exit 1
    }
    else
    {
        Write-Host "DONE" -ForegroundColor Green
    }
}
else
{
    Write-Host "Diesel found ✅"
}

if (!(Test-Path $PSScriptRoot/$DevDir))
{
    Write-Host "FATAL" -ForegroundColor Red -NoNewline
    Write-Host ": directory '$DevDir' is missing (try pull)"
    exit 1
}

if (!((Test-Path $EnvFile) -and (Test-Path $ComposeEnvFile)))
{
    Write-Host "Creating environment..."
    if (Test-Path $SampleEnvFile)
    {
        Copy-Item $SampleEnvFile $EnvFile
        Copy-Item $SampleEnvFile $ComposeEnvFile
        if (!(Test-Command "Set-DotEnv"))
        {
            Write-Host "Module to read environment not found. Trying to install it... " -ForegroundColor Yellow -NoNewline
            Find-Module -Name dotenv | Install-Module -Confirm
            if ($?)
            {
                Write-Host "DONE" -ForegroundColor Green
            }
            else
            {
                Write-Host "FAILED" -ForegroundColor Red
                exit 1
            }
        }
        Set-DotEnv -Path $EnvFile
    }
    else
    {
        Write-Host "FATAL" -ForegroundColor Red
        Write-Host ": sample environment file '$SampleEnvFile' is missing (try pull)"
        exit 1
    }
    Write-Host "DONE" -ForegroundColor Green
    Write-Host "Consider changing default passwords in .env file!" -ForegroundColor Yellow
}

if (!(Test-Env "CLIENT_PORT"))
{
    Write-Host "FATAL" -ForegroundColor Red -NoNewline
    Write-Host ": CLIENT_PORT environment variable not set (check .env file and/or source it)"
    exit 1
}
else
{
    Write-Host "CLIENT_PORT found ✅"
}

if (!(Test-Env "POSTGRES_URL"))
{
    Write-Host "FATAL" -ForegroundColor Red -NoNewline
    Write-Host ": POSTGRES_URL environment variable not set (check .env file and/or source it)"
    exit 1
}
else
{
    Write-Host "POSTGRES_URL found ✅"
}

# Starting containers:

Write-Host "Starting containers (Ctrl-C to abort)... " -ForegroundColor White

Set-Location $PSScriptRoot/$DevDir

$BackendPid = 0
$FrontendPid = 0

try
{
    docker compose up --detach
    while ($true)
    {
        $DockerPs = (docker compose ps)
        $NumberOfRunningContainers = ($DockerPs | Select-String -Pattern " Up ").Matches.Count
        if ($NumberOfRunningContainers -ge $ExpectedNumberOfContainers)
        {
            break
        }
    }
    Write-Host "DONE" -ForegroundColor Green

    # Running Diesel migration:

    Write-Host "Running migrations... " -ForegroundColor White
    while ($true)
    {
        $PostgresReady = ((docker compose ps)|Select-String -Pattern "postgres"|Select-String -Pattern "healthy").Matches.Count
        if ($PostgresReady -ge 1)
        {
            break
        }
    }
    Set-Location $PSScriptRoot
    if (!(diesel migration run --database-url "$env:POSTGRES_URL" --migration-dir $MigrationsDir))
    {
        Write-Host "FAILED" -ForegroundColor Red
        exit 1
    }
    Write-Host "DONE" -ForegroundColor Green

    Write-Host "Startup requirements fulfilled ✅" -ForegroundColor Green
    Write-Host "Component requirements might still be lacking" -ForegroundColor Yellow
    Write-Host "Check the corresponding output for hints"

    # Providing info:

    Write-Host "Starting Application backend & frontend... " -ForegroundColor White -NoNewline
    Write-Host "Ctrl-C/Ctrl-Break to exit both" -ForegroundColor Yellow
    Start-Sleep -Seconds 2

    # Starting dev backend and frontend:
    $p = Start-Process -Path "cargo" -ArgumentList "-Z","unstable-options","-C","./","watch","-c","-w","src","-x","run" -PassThru
    $BackendPid = $p.Id
    Out-File -FilePath "$PSScriptRoot/backend.pid" -InputObject "$BackendPid"

    Set-Location "$PSScriptRoot/frontend"
    $p = Start-Process -Path "trunk" -ArgumentList "serve","--port=$env:CLIENT_PORT" -PassThru
    $FrontendPid = $p.Id
    Out-File -FilePath "$PSScriptRoot/frontend.pid" -InputObject "$FrontendPid"

    while ($true)
    {
        Start-Sleep -Milliseconds 500
    }
}
finally
{
    if (!($BackendPid -eq 0))
    {
        Stop-Process $BackendPid
        Write-Host "Stopped Backend process ($BackendPid)"
    }
    if (!($FrontendPid -eq 0))
    {
        Stop-Process $FrontendPid
        Write-Host "Stopped Frontend process ($FrontendPid)"
    }
    Set-Location $PSScriptRoot/$DevDir
    docker compose down
}
