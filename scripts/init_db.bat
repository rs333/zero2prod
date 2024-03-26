@echo off
setlocal enabledelayedexpansion

set "DB_USER=postgres"
set "DB_PASSWORD=password"
set "DB_NAME=newsletter"

set "DB_PORT=5432"
set "DB_HOST=localhost"

docker run ^
	-e POSTGRES_USER=%DB_USER% ^
	-e POSTGRES_PASSWORD=%DB_PASSWORD% ^
	-e POSTGRES_DB=%DB_NAME% ^
	-p %DB_PORT%:5432 ^
	-d postgres ^
	postgres -N 1000

:: Keep pinging Postgres until it's ready to accept commands
set "PGPASSWORD=%DB_PASSWORD%"
@REM :retry
@REM psql -h "%DB_HOST%" -U "%DB_USER%" -p "%DB_PORT%" -d "postgres" -c "\q"
@REM if errorlevel 1 (
@REM     echo Postgres is still unavailable - sleeping
@REM     timeout /t 1 >nul
@REM     goto retry
@REM )
echo Postgres is up and running on port %DB_PORT%

set "DATABASE_URL=postgres://%DB_USER%:%DB_PASSWORD%@%DB_HOST%:%DB_PORT%/%DB_NAME%"
sqlx database create