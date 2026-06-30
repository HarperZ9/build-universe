@echo off
REM Build all .bld programs to native binaries via buildc + MSVC
REM Run from cmd.exe: cd C:\Users\Zain\BUILD-UNIVERSE\programs && build_all.bat
REM
REM Prerequisites:
REM   - buildc.exe built (cargo build --release in buildlang/compiler)
REM   - Visual Studio Build Tools 2022 installed

call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1

set BUILDC=..\buildlang\compiler\target\release\buildc.exe
set PASS=0
set FAIL=0
set SKIP=0

echo === Building all BuildLang programs ===
echo.

for %%f in (*.bld) do (
    set "NAME=%%~nf"
    REM Skip test files
    echo %%~nf | findstr /B "test_" >nul && (
        set /a SKIP+=1
        echo   SKIP %%~nf [test file]
    ) || (
        echo   Compiling %%~nf.bld...
        %BUILDC% %%f >nul 2>&1
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
            echo   FAIL %%~nf [buildc error]
        )
    )
)

echo.
echo === Results ===
echo   PASS: %PASS%
echo   FAIL: %FAIL%
echo   SKIP: %SKIP%
