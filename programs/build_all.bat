@echo off
REM Build all .quanta programs to native binaries via quantac + MSVC
REM Run from cmd.exe: cd C:\Users\Zain\QUANTA-UNIVERSE\programs && build_all.bat
REM
REM Prerequisites:
REM   - quantac.exe built (cargo build --release in quantalang/compiler)
REM   - Visual Studio Build Tools 2022 installed

call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1

set QUANTAC=..\quantalang\compiler\target\release\quantac.exe
set PASS=0
set FAIL=0
set SKIP=0

echo === Building all QuantaLang programs ===
echo.

for %%f in (*.quanta) do (
    set "NAME=%%~nf"
    REM Skip test files
    echo %%~nf | findstr /B "test_" >nul && (
        set /a SKIP+=1
        echo   SKIP %%~nf [test file]
    ) || (
        echo   Compiling %%~nf.quanta...
        %QUANTAC% %%f >nul 2>&1
        if exist "%%~nf.c" (
            cl /O2 /nologo /Fe:q%%~nf.exe %%~nf.c >nul 2>&1
            if exist "q%%~nf.exe" (
                set /a PASS+=1
                echo   OK  q%%~nf.exe
            ) else (
                set /a FAIL+=1
                echo   FAIL %%~nf [C compile error]
            )
        ) else (
            set /a FAIL+=1
            echo   FAIL %%~nf [quantac error]
        )
    )
)

echo.
echo === Results ===
echo   PASS: %PASS%
echo   FAIL: %FAIL%
echo   SKIP: %SKIP%
